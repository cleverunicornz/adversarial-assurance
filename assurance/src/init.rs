//! Actor-driven adoption installer.
//!
//! Provenance: embedding canonical seed files follows bedrock's standalone
//! installer model; this implementation installs the assurance-specific tree.

use crate::embedded;
use crate::error::Fatal;
use std::path::Path;

pub fn run(root: &Path) -> Result<(), Fatal> {
    if root.exists() && !root.is_dir() {
        return Err(Fatal(format!(
            "assurance init: target is not a directory: {}",
            root.display()
        )));
    }
    std::fs::create_dir_all(root)
        .map_err(|error| Fatal(format!("assurance init: cannot create target: {error}")))?;

    let assurance = root.join(".assurance");
    let workflow = root.join(".github/workflows/assurance.yml");
    if assurance.exists() {
        return Err(Fatal(
            "assurance init: .assurance already exists; refusing to overwrite adoption state"
                .to_owned(),
        ));
    }
    if workflow.exists() {
        return Err(Fatal(
            "assurance init: .github/workflows/assurance.yml already exists; refusing to overwrite it"
                .to_owned(),
        ));
    }

    let schema = assurance.join("schema");
    let runs = assurance.join("runs");
    std::fs::create_dir_all(&schema)
        .and_then(|_| std::fs::create_dir_all(&runs))
        .and_then(|_| {
            workflow
                .parent()
                .map(std::fs::create_dir_all)
                .transpose()
                .map(|_| ())
        })
        .map_err(|error| Fatal(format!("assurance init: cannot create skeleton: {error}")))?;

    write(
        &assurance.join("assurance-init.yaml"),
        embedded::INIT_TEMPLATE,
    )?;
    write(&assurance.join("registry.yaml"), embedded::EMPTY_REGISTRY)?;
    write(&schema.join("context.yamlld"), embedded::CONTEXT)?;
    write(&schema.join("vocabulary.yaml"), embedded::VOCABULARY)?;
    write(&schema.join("records.schema.json"), embedded::RECORD_SCHEMA)?;
    write(&runs.join(".gitkeep"), "")?;
    write(&workflow, embedded::WORKFLOW)?;

    println!("assurance init: installed .assurance/ skeleton");
    println!("assurance init: installed .github/workflows/assurance.yml");
    print!("{}", embedded::BOOTSTRAP_INSTRUCTIONS);
    Ok(())
}

fn write(path: &Path, content: &str) -> Result<(), Fatal> {
    std::fs::write(path, content.as_bytes()).map_err(|error| {
        Fatal(format!(
            "assurance init: cannot write {}: {error}",
            path.display()
        ))
    })
}
