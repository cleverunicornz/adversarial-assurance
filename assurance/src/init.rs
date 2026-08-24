//! Actor-driven expansion-mount installer and updater.
//!
//! Provenance: embedding canonical seed files follows bedrock's standalone
//! installer model; this implementation owns only `situation/assurance/`.

use crate::embedded;
use crate::error::Fatal;
use crate::graph;
use crate::model::{GRAPH_MANIFEST_REL, MOUNT_REL};
use crate::substrate;
use serde_json::Value;
use std::path::Path;

pub fn run(root: &Path) -> Result<(), Fatal> {
    substrate::require_for_install(root, "init")?;
    let mount = root.join(MOUNT_REL);
    if mount.exists() {
        return Err(Fatal(format!(
            "assurance init: {MOUNT_REL}/ already exists; refusing to overwrite mount state"
        )));
    }

    let schema = mount.join("schema");
    let runs = mount.join("runs");
    let workflow = mount.join("workflow");
    std::fs::create_dir_all(&schema)
        .and_then(|_| std::fs::create_dir_all(&runs))
        .and_then(|_| std::fs::create_dir_all(&workflow))
        .map_err(|error| {
            Fatal(format!(
                "assurance init: cannot create mount skeleton: {error}"
            ))
        })?;

    write(&mount.join("assurance-init.yaml"), embedded::INIT_TEMPLATE)?;
    write(&mount.join("registry.yaml"), embedded::EMPTY_REGISTRY)?;
    write(&schema.join("context.yamlld"), embedded::CONTEXT)?;
    write(&schema.join("vocabulary.yaml"), embedded::VOCABULARY)?;
    write(&schema.join("records.schema.json"), embedded::RECORD_SCHEMA)?;
    write(
        &workflow.join("assurance.yml"),
        &embedded::render_workflow(None),
    )?;
    write(&runs.join(".gitkeep"), "")?;
    write(
        &root.join(GRAPH_MANIFEST_REL),
        embedded::EMPTY_GRAPH_MANIFEST,
    )?;

    println!("assurance init: installed {MOUNT_REL}/ expansion mount");
    print!("{}", embedded::BOOTSTRAP_INSTRUCTIONS);
    print_registration_proposal(root, "registration proposal")?;
    Ok(())
}

pub fn update(root: &Path) -> Result<(), Fatal> {
    substrate::require_for_install(root, "update")?;
    let mount = root.join(MOUNT_REL);
    let metadata = std::fs::symlink_metadata(&mount).map_err(|_| {
        Fatal(format!(
            "assurance update: {MOUNT_REL}/ is absent; run `assurance init` first"
        ))
    })?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(Fatal(format!(
            "assurance update: {MOUNT_REL}/ must be a real non-symlink directory"
        )));
    }
    let runner = configured_runner(root)?;
    let manifest = graph::manifest_from_disk(root)?;
    let files = [
        ("schema/context.yamlld", embedded::CONTEXT.as_bytes()),
        ("schema/vocabulary.yaml", embedded::VOCABULARY.as_bytes()),
        (
            "schema/records.schema.json",
            embedded::RECORD_SCHEMA.as_bytes(),
        ),
    ];
    let mut changed = Vec::new();
    for (relative, bytes) in files {
        let path = mount.join(relative);
        if write_if_changed(&path, bytes)? {
            changed.push(format!("{MOUNT_REL}/{relative}"));
        }
    }
    let workflow = embedded::render_workflow(Some(&runner));
    if write_if_changed(&mount.join("workflow/assurance.yml"), workflow.as_bytes())? {
        changed.push(format!("{MOUNT_REL}/workflow/assurance.yml"));
    }
    if write_if_changed(&root.join(GRAPH_MANIFEST_REL), &manifest)? {
        changed.push(GRAPH_MANIFEST_REL.to_owned());
    }

    if changed.is_empty() {
        println!("assurance update: mount-owned canonical files already current");
    } else {
        println!("assurance update: refreshed {} file(s):", changed.len());
        for path in &changed {
            println!("  + {path}");
        }
    }

    let report = crate::check::inspect(root)?;
    if !report.violations.is_empty() {
        crate::check::print_violations(&report.violations);
        return Err(Fatal(format!(
            "assurance update: refreshed mount does not pass check ({} violation(s))",
            report.violations.len()
        )));
    }
    print_registration_proposal(root, "replacement registration proposal")?;
    Ok(())
}

pub fn print_registration_proposal(root: &Path, label: &str) -> Result<(), Fatal> {
    let manifest = std::fs::read(root.join(GRAPH_MANIFEST_REL)).map_err(|error| {
        Fatal(format!(
            "assurance: cannot read {GRAPH_MANIFEST_REL} for registration proposal: {error}"
        ))
    })?;
    let digest = graph::sha256_hex(&manifest);
    println!(
        "\nASSURANCE EXPANSION MOUNT {label} — PROPOSAL ONLY; write through the bedrock authoring loop:\n\
path: situation/architecture/mount-assurance.yamlld\n\
---\n\
\"@context\": \"urn:bedrock:context/v1\"\n\
\"@id\": \"urn:bedrock:vertex/mount-assurance\"\n\
\"@type\": \"urn:bedrock:ontology/ExpansionMount\"\n\
label: \"Assurance expansion mount\"\n\
statement: \"The assurance expansion owns adversarial campaign records and graphs under its registered mount.\"\n\
mount_contract_version: 1\n\
mount_name: \"assurance\"\n\
mount_path: \"urn:bedrock:path/situation/assurance\"\n\
checker_identity: \"assurance\"\n\
checker_arguments:\n\
  - \"check\"\n\
  - \".\"\n\
init_path: \"urn:bedrock:path/situation/assurance/assurance-init.yaml\"\n\
graph_manifest_path: \"urn:bedrock:path/situation/assurance/graph-manifest.yaml\"\n\
graph_manifest_sha256: \"{digest}\"\n\
---"
    );
    Ok(())
}

fn configured_runner(root: &Path) -> Result<String, Fatal> {
    let path = root.join(MOUNT_REL).join("assurance-init.yaml");
    let source = std::fs::read_to_string(&path).map_err(|error| {
        Fatal(format!(
            "assurance update: cannot read {MOUNT_REL}/assurance-init.yaml: {error}"
        ))
    })?;
    let value: Value = serde_norway::from_str(&source).map_err(|error| {
        Fatal(format!(
            "assurance update: assurance-init.yaml is not valid YAML: {error}"
        ))
    })?;
    if value.get("status").and_then(Value::as_str) != Some("CONFIGURED") {
        return Err(Fatal(
            "assurance update: bindings are not CONFIGURED; populate the init variables first"
                .to_owned(),
        ));
    }
    value
        .get("variables")
        .and_then(|variables| variables.get("witness_runner"))
        .and_then(Value::as_str)
        .filter(|runner| {
            !runner.is_empty()
                && runner.chars().all(|character| {
                    character.is_ascii_alphanumeric() || matches!(character, '_' | '-' | '.')
                })
        })
        .map(str::to_owned)
        .ok_or_else(|| {
            Fatal(
                "assurance update: variables.witness_runner must be one literal configured label"
                    .to_owned(),
            )
        })
}

fn write(path: &Path, content: &str) -> Result<(), Fatal> {
    std::fs::write(path, content.as_bytes()).map_err(|error| {
        Fatal(format!(
            "assurance init: cannot write {}: {error}",
            path.display()
        ))
    })
}

fn write_if_changed(path: &Path, bytes: &[u8]) -> Result<bool, Fatal> {
    if std::fs::read(path).is_ok_and(|existing| existing == bytes) {
        return Ok(false);
    }
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|error| {
            Fatal(format!(
                "assurance update: cannot create {}: {error}",
                parent.display()
            ))
        })?;
    }
    std::fs::write(path, bytes).map_err(|error| {
        Fatal(format!(
            "assurance update: cannot write {}: {error}",
            path.display()
        ))
    })?;
    Ok(true)
}
