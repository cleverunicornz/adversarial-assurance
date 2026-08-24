//! `assurance check`: deterministic, fail-hard validation of `situation/assurance/`.
//!
//! Provenance: ordered rule aggregation and exact line diagnostics follow
//! bedrock's public checker contract; these rules implement this pack's schema.
//!
//! Rules:
//! - A001 adoption bindings
//! - A002 canonical schema, workflow, and registry version/state
//! - A003 run-folder placement and identity
//! - A004 constrained YAML 1.2 syntax
//! - A005 source formatting and prose block scalars
//! - A006 closed JSON-LD context, nouns, verbs, fields, and expansion
//! - A007 record shape
//! - A008 edge and path resolution
//! - A009 run coherence, actor binding, and witness digest linkage
//! - A010 successor and disposition legality

use crate::embedded;
use crate::error::{Fatal, Violation, line_of};
use crate::graph;
use crate::model::{
    Document, GRAPH_MANIFEST_REL, MOUNT_REL, PATH_PREFIX, RECORD_PREFIX, RecordKind, Vocabulary,
    pointer_path, portable,
};
use crate::substrate;
use crate::yaml_syntax::{self, Forbidden};
use jsonschema::Validator;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

pub struct Report {
    pub violations: Vec<Violation>,
    pub documents: Vec<Document>,
}

#[derive(Default)]
struct Layout {
    sources: Vec<Source>,
    run_slugs: BTreeSet<String>,
    evidence: BTreeMap<String, BTreeSet<PathBuf>>,
}

struct Source {
    rel: PathBuf,
    run_slug: String,
    expected_kind: RecordKind,
}

#[derive(Default)]
struct Bindings {
    variables: BTreeMap<String, String>,
}

impl Bindings {
    fn resolve<'a>(&'a self, value: &'a str) -> Option<&'a str> {
        let name = value.strip_prefix("{{")?.strip_suffix("}}")?;
        self.variables.get(name).map(String::as_str)
    }

    fn actor_values(&self) -> BTreeSet<&str> {
        ["reviewer_seat", "final_validator_seat"]
            .into_iter()
            .filter_map(|name| self.variables.get(name).map(String::as_str))
            .collect()
    }
}

#[derive(Clone, Copy)]
enum GeneratedPolicy {
    RequireCurrent,
    IgnoreDrift,
}

pub fn inspect(root: &Path) -> Result<Report, Fatal> {
    inspect_with_policy(root, GeneratedPolicy::RequireCurrent)
}

pub fn inspect_for_build(root: &Path) -> Result<Report, Fatal> {
    inspect_with_policy(root, GeneratedPolicy::IgnoreDrift)
}

fn inspect_with_policy(root: &Path, generated_policy: GeneratedPolicy) -> Result<Report, Fatal> {
    let vocabulary = Vocabulary::embedded()?;
    let validator = record_validator()?;
    let mut violations = Vec::new();

    let mount = root.join(MOUNT_REL);
    if !mount.is_dir() {
        violations.push(Violation::new(
            "A001",
            MOUNT_REL,
            1,
            "assurance mount is absent; form the repository with mount-capable bedrock, then run `assurance init`",
        ));
        return Ok(Report {
            violations,
            documents: Vec::new(),
        });
    }

    let bindings = check_bindings(root, &vocabulary, &mut violations);
    check_canonical_files(root, bindings.as_ref(), &mut violations);
    let layout = scan_layout(root, &mut violations);
    check_registry(root, &layout.run_slugs, &vocabulary, &mut violations);

    let mut documents = Vec::new();
    for source in &layout.sources {
        if let Some(document) = parse_record(root, source, &vocabulary, &mut violations) {
            check_format(&document, &mut violations);
            check_vocabulary(&document, &vocabulary, &mut violations);
            check_shape(&document, &validator, &mut violations);
            if document.kind.is_some()
                && document.id.is_some()
                && let Err(violation) = graph::expand(&document)
            {
                violations.push(violation);
            }
            documents.push(document);
        }
    }
    check_record_variables(&documents, bindings.as_ref(), &vocabulary, &mut violations);

    check_edges(root, &documents, &vocabulary, &mut violations);
    check_runs(
        root,
        &documents,
        &layout.run_slugs,
        &layout.evidence,
        bindings.as_ref(),
        &mut violations,
    );
    check_transitions(&documents, &vocabulary, &mut violations);
    if matches!(generated_policy, GeneratedPolicy::RequireCurrent) {
        check_generated(root, &documents, &mut violations);
    }

    violations.sort();
    violations.dedup();
    Ok(Report {
        violations,
        documents,
    })
}

pub fn print_violations(violations: &[Violation]) {
    for violation in violations {
        eprintln!("{violation}");
    }
}

fn record_validator() -> Result<Validator, Fatal> {
    let schema: Value = serde_json::from_str(embedded::RECORD_SCHEMA)
        .map_err(|error| Fatal(format!("embedded record schema is invalid JSON: {error}")))?;
    jsonschema::validator_for(&schema)
        .map_err(|error| Fatal(format!("embedded record schema does not compile: {error}")))
}

fn check_bindings(
    root: &Path,
    vocabulary: &Vocabulary,
    violations: &mut Vec<Violation>,
) -> Option<Bindings> {
    let rel = "situation/assurance/assurance-init.yaml";
    let path = root.join(rel);
    let source = match std::fs::read_to_string(&path) {
        Ok(source) => source,
        Err(_) => {
            violations.push(Violation::new(
                "A001",
                rel,
                1,
                "missing binding file; rerun `assurance init` in a clean adoption",
            ));
            return None;
        }
    };
    let value: Value = match serde_norway::from_str(&source) {
        Ok(value) => value,
        Err(error) => {
            violations.push(Violation::new(
                "A001",
                rel,
                yaml_error_line(&error.to_string()),
                format!("binding file is not valid YAML: {error}"),
            ));
            return None;
        }
    };
    let Some(map) = value.as_object() else {
        violations.push(Violation::new(
            "A001",
            rel,
            1,
            "binding file must be a mapping",
        ));
        return None;
    };

    let expected: BTreeSet<&str> = ["version", "status", "substrate", "pack", "variables"]
        .into_iter()
        .collect();
    let mut valid = true;
    for key in map.keys() {
        if !expected.contains(key.as_str()) {
            valid = false;
            violations.push(Violation::new(
                "A001",
                rel,
                line_of(&source, key),
                format!(
                    "unknown binding `{key}`; assurance-init.yaml is the closed binding surface"
                ),
            ));
        }
    }

    if map.get("version").and_then(Value::as_u64) != Some(1) {
        valid = false;
        violations.push(Violation::new(
            "A001",
            rel,
            line_of(&source, "version"),
            "version must be 1",
        ));
    }
    if map.get("status").and_then(Value::as_str) != Some("CONFIGURED") {
        valid = false;
        violations.push(Violation::new(
            "A001",
            rel,
            line_of(&source, "status"),
            "status is not CONFIGURED; ask the human the bootstrap question, populate every variables key, then set `status: CONFIGURED`",
        ));
    }

    if nested_string(&value, &["substrate", "contract"]) != Some(substrate::CONTRACT) {
        valid = false;
        violations.push(Violation::new(
            "A001",
            rel,
            line_of(&source, "contract"),
            format!("substrate.contract must be `{}`", substrate::CONTRACT),
        ));
    }
    if value
        .get("substrate")
        .and_then(|substrate| substrate.get("minimum_contract_version"))
        .and_then(Value::as_u64)
        != Some(substrate::CONTRACT_VERSION)
    {
        valid = false;
        violations.push(Violation::new(
            "A001",
            rel,
            line_of(&source, "minimum_contract_version"),
            format!(
                "substrate.minimum_contract_version must be {}",
                substrate::CONTRACT_VERSION
            ),
        ));
    }

    let repository = nested_string(&value, &["pack", "repository"]);
    if !repository.is_some_and(valid_repository) {
        valid = false;
        violations.push(Violation::new(
            "A001",
            rel,
            line_of(&source, "repository"),
            "pack.repository must be a non-placeholder `owner/repository` coordinate",
        ));
    }
    let pack_ref = nested_string(&value, &["pack", "ref"]);
    if !pack_ref.is_some_and(|value| is_lower_hex(value, 40)) {
        valid = false;
        violations.push(Violation::new(
            "A001",
            rel,
            line_of(&source, "ref"),
            "pack.ref must be a full 40-character lowercase commit SHA",
        ));
    }

    let mut variables = BTreeMap::new();
    let variable_map = value.get("variables").and_then(Value::as_object);
    if variable_map.is_none() {
        valid = false;
        violations.push(Violation::new(
            "A001",
            rel,
            line_of(&source, "variables"),
            "variables must be a mapping containing the complete canonical variable set",
        ));
    }
    if let Some(variable_map) = variable_map {
        for name in variable_map.keys() {
            if !vocabulary.variables.contains(name) {
                valid = false;
                violations.push(Violation::new(
                    "A001",
                    rel,
                    line_of(&source, name),
                    format!(
                        "variable `{{{{{name}}}}}` is not in vocabulary version {}",
                        vocabulary.version
                    ),
                ));
            }
        }
        for name in &vocabulary.variables {
            match variable_map.get(name).and_then(Value::as_str) {
                Some(value) if configured_value(value) => {
                    if name == "witness_runner" && !valid_runner_label(value) {
                        valid = false;
                        violations.push(Violation::new(
                            "A001",
                            rel,
                            line_of(&source, name),
                            "variables.witness_runner must be one literal runner label",
                        ));
                    } else {
                        variables.insert(name.clone(), value.to_owned());
                    }
                }
                _ => {
                    valid = false;
                    violations.push(Violation::new(
                        "A001",
                        rel,
                        line_of(&source, name),
                        format!(
                            "required variable `{{{{{name}}}}}` is missing, empty, or still a placeholder"
                        ),
                    ));
                }
            }
        }
    }

    let substrate_before = violations.len();
    substrate::check(
        root,
        variables.get("witness_runner").map(String::as_str),
        violations,
    );
    if violations.len() != substrate_before {
        valid = false;
    }
    valid.then_some(Bindings { variables })
}

fn check_record_variables(
    documents: &[Document],
    bindings: Option<&Bindings>,
    vocabulary: &Vocabulary,
    violations: &mut Vec<Violation>,
) {
    for document in documents {
        let mut cursor = 0;
        while let Some(relative_start) = document.source[cursor..].find("{{") {
            let start = cursor + relative_start;
            let line = document.source[..start]
                .bytes()
                .filter(|byte| *byte == b'\n')
                .count() as u32
                + 1;
            let content_start = start + 2;
            let Some(relative_end) = document.source[content_start..].find("}}") else {
                violations.push(Violation::new(
                    "A001",
                    document.rel_string(),
                    line,
                    format!(
                        "unterminated variable reference; expected `{}` syntax",
                        vocabulary.variable_syntax
                    ),
                ));
                break;
            };
            let end = content_start + relative_end;
            let name = &document.source[content_start..end];
            if !valid_variable_name(name) || !vocabulary.variables.contains(name) {
                violations.push(Violation::new(
                    "A001",
                    document.rel_string(),
                    line,
                    format!(
                        "record variable `{{{{{name}}}}}` is not in the canonical variable set"
                    ),
                ));
            } else if bindings.is_none_or(|bindings| !bindings.variables.contains_key(name)) {
                violations.push(Violation::new(
                    "A001",
                    document.rel_string(),
                    line,
                    format!(
                        "record variable `{{{{{name}}}}}` is not declared and configured in assurance-init.yaml"
                    ),
                ));
            }
            cursor = end + 2;
        }
    }
}

fn check_canonical_files(
    root: &Path,
    bindings: Option<&Bindings>,
    violations: &mut Vec<Violation>,
) {
    let workflow = embedded::render_workflow(
        bindings.and_then(|bindings| bindings.variables.get("witness_runner").map(String::as_str)),
    );
    let files: [(&str, &[u8]); 4] = [
        (
            "situation/assurance/schema/context.yamlld",
            embedded::CONTEXT.as_bytes(),
        ),
        (
            "situation/assurance/schema/vocabulary.yaml",
            embedded::VOCABULARY.as_bytes(),
        ),
        (
            "situation/assurance/schema/records.schema.json",
            embedded::RECORD_SCHEMA.as_bytes(),
        ),
        (
            "situation/assurance/workflow/assurance.yml",
            workflow.as_bytes(),
        ),
    ];
    for (rel, canonical) in files {
        match std::fs::read(root.join(rel)) {
            Ok(bytes) if bytes == canonical => {}
            Ok(_) => violations.push(Violation::new(
                "A002",
                rel,
                1,
                "mount-owned canonical file differs from this checker; run `assurance update`",
            )),
            Err(_) => violations.push(Violation::new(
                "A002",
                rel,
                1,
                "mount-owned canonical file is missing; run `assurance update`",
            )),
        }
    }
}

fn check_registry(
    root: &Path,
    run_slugs: &BTreeSet<String>,
    vocabulary: &Vocabulary,
    violations: &mut Vec<Violation>,
) {
    let rel = "situation/assurance/registry.yaml";
    let source = match std::fs::read_to_string(root.join(rel)) {
        Ok(source) => source,
        Err(_) => {
            violations.push(Violation::new(
                "A002",
                rel,
                1,
                "registry is missing; rerun `assurance init` in a clean adoption",
            ));
            return;
        }
    };
    let value: Value = match serde_norway::from_str(&source) {
        Ok(value) => value,
        Err(error) => {
            violations.push(Violation::new(
                "A002",
                rel,
                yaml_error_line(&error.to_string()),
                format!("registry is not valid YAML: {error}"),
            ));
            return;
        }
    };
    let expected_state = if run_slugs.is_empty() {
        "CONFIGURED_EMPTY"
    } else {
        "ACTIVE"
    };
    if value.get("version").and_then(Value::as_u64) != Some(1) {
        violations.push(Violation::new(
            "A002",
            rel,
            line_of(&source, "version"),
            "registry version must be 1",
        ));
    }
    if value.get("vocabulary_version").and_then(Value::as_u64) != Some(vocabulary.version) {
        violations.push(Violation::new(
            "A002",
            rel,
            line_of(&source, "vocabulary_version"),
            format!(
                "vocabulary_version must be {}; additions require a versioned schema change",
                vocabulary.version
            ),
        ));
    }
    if value.get("state").and_then(Value::as_str) != Some(expected_state) {
        violations.push(Violation::new(
            "A002",
            rel,
            line_of(&source, "state"),
            format!("registry state must be {expected_state} for the current runs/ contents"),
        ));
    }
    let allowed: BTreeSet<&str> = ["version", "vocabulary_version", "state"]
        .into_iter()
        .collect();
    if let Some(map) = value.as_object() {
        for key in map.keys() {
            if !allowed.contains(key.as_str()) {
                violations.push(Violation::new(
                    "A002",
                    rel,
                    line_of(&source, key),
                    format!("unknown registry field `{key}`"),
                ));
            }
        }
    }
}

fn scan_layout(root: &Path, violations: &mut Vec<Violation>) -> Layout {
    let mount = root.join(MOUNT_REL);
    let allowed_root: BTreeSet<&str> = [
        "assurance-init.yaml",
        "registry.yaml",
        "schema",
        "runs",
        "workflow",
        "graph-manifest.yaml",
    ]
    .into_iter()
    .collect();
    for path in sorted_entries(&mount) {
        let name = file_name(&path);
        if !allowed_root.contains(name.as_str()) {
            violations.push(Violation::new(
                "A003",
                format!("{MOUNT_REL}/{name}"),
                1,
                "unexpected mount entry; only bindings, registry, schema/, runs/, workflow/, and graph-manifest.yaml are allowed",
            ));
        }
    }

    let schema = mount.join("schema");
    let allowed_schema: BTreeSet<&str> =
        ["context.yamlld", "vocabulary.yaml", "records.schema.json"]
            .into_iter()
            .collect();
    if schema.is_dir() {
        for path in sorted_entries(&schema) {
            let name = file_name(&path);
            if !allowed_schema.contains(name.as_str()) {
                violations.push(Violation::new(
                    "A003",
                    format!("{MOUNT_REL}/schema/{name}"),
                    1,
                    "unexpected schema entry",
                ));
            }
        }
    }

    let workflow = mount.join("workflow");
    if !workflow.is_dir() {
        violations.push(Violation::new(
            "A003",
            format!("{MOUNT_REL}/workflow"),
            1,
            "mount-owned workflow template directory is missing",
        ));
    } else {
        for path in sorted_entries(&workflow) {
            if file_name(&path) != "assurance.yml" || !path.is_file() {
                violations.push(Violation::new(
                    "A003",
                    rel_to(root, &path),
                    1,
                    "workflow/ contains only the mount-owned assurance.yml template",
                ));
            }
        }
    }

    let runs = mount.join("runs");
    let mut layout = Layout::default();
    if !runs.is_dir() {
        violations.push(Violation::new(
            "A003",
            format!("{MOUNT_REL}/runs"),
            1,
            "runs/ directory is missing",
        ));
        return layout;
    }

    for run_path in sorted_entries(&runs) {
        let run_slug = file_name(&run_path);
        if run_slug == ".gitkeep" && run_path.is_file() {
            continue;
        }
        let run_rel = format!("{MOUNT_REL}/runs/{run_slug}");
        let metadata = match std::fs::symlink_metadata(&run_path) {
            Ok(metadata) => metadata,
            Err(_) => continue,
        };
        if metadata.file_type().is_symlink() || !metadata.is_dir() {
            violations.push(Violation::new(
                "A003",
                run_rel,
                1,
                "runs/ may contain only real kebab-case run directories and .gitkeep",
            ));
            continue;
        }
        if !valid_slug(&run_slug) {
            violations.push(Violation::new(
                "A003",
                &run_rel,
                1,
                "run directory name must be lowercase kebab-case",
            ));
            continue;
        }
        layout.run_slugs.insert(run_slug.clone());
        scan_run(
            root,
            &run_path,
            &run_slug,
            &mut layout.sources,
            &mut layout.evidence,
            violations,
        );
    }
    layout
}

fn scan_run(
    root: &Path,
    run_path: &Path,
    run_slug: &str,
    sources: &mut Vec<Source>,
    evidence_by_run: &mut BTreeMap<String, BTreeSet<PathBuf>>,
    violations: &mut Vec<Violation>,
) {
    let allowed: BTreeSet<&str> = [
        "run.yamlld",
        "promises",
        "witnesses",
        "oracles",
        "evidence",
        "graph.trig",
    ]
    .into_iter()
    .collect();
    for path in sorted_entries(run_path) {
        let name = file_name(&path);
        if !allowed.contains(name.as_str()) {
            violations.push(Violation::new(
                "A003",
                rel_to(root, &path),
                1,
                "unexpected run entry; expected run.yamlld, triad directories, evidence/, and graph.trig",
            ));
        }
    }

    let run_file = run_path.join("run.yamlld");
    if run_file.is_file() {
        sources.push(Source {
            rel: run_file
                .strip_prefix(root)
                .unwrap_or(&run_file)
                .to_path_buf(),
            run_slug: run_slug.to_owned(),
            expected_kind: RecordKind::Run,
        });
    } else {
        violations.push(Violation::new(
            "A003",
            format!("{MOUNT_REL}/runs/{run_slug}/run.yamlld"),
            1,
            "every run requires exactly one run.yamlld Run record",
        ));
    }

    for kind in [RecordKind::Promise, RecordKind::Witness, RecordKind::Oracle] {
        let directory = kind.directory().expect("triad kind has a directory");
        let path = run_path.join(directory);
        if !path.is_dir() {
            violations.push(Violation::new(
                "A003",
                format!("{MOUNT_REL}/runs/{run_slug}/{directory}"),
                1,
                format!("every run requires a flat {directory}/ directory"),
            ));
            continue;
        }
        for entry in sorted_entries(&path) {
            let name = file_name(&entry);
            let rel = rel_to(root, &entry);
            let metadata = match std::fs::symlink_metadata(&entry) {
                Ok(metadata) => metadata,
                Err(_) => continue,
            };
            if metadata.file_type().is_symlink() || metadata.is_dir() {
                violations.push(Violation::new(
                    "A003",
                    rel,
                    1,
                    format!("{directory}/ is flat and contains no symlinks"),
                ));
                continue;
            }
            if !metadata.is_file() || !name.ends_with(".yamlld") {
                violations.push(Violation::new(
                    "A003",
                    rel,
                    1,
                    format!("{directory}/ contains only kebab-case .yamlld records"),
                ));
                continue;
            }
            let stem = name.trim_end_matches(".yamlld");
            if !valid_slug(stem) {
                violations.push(Violation::new(
                    "A003",
                    rel,
                    1,
                    "record filename must be lowercase kebab-case",
                ));
                continue;
            }
            sources.push(Source {
                rel: entry.strip_prefix(root).unwrap_or(&entry).to_path_buf(),
                run_slug: run_slug.to_owned(),
                expected_kind: kind,
            });
        }
    }

    let evidence = run_path.join("evidence");
    if !evidence.is_dir() {
        violations.push(Violation::new(
            "A003",
            format!("{MOUNT_REL}/runs/{run_slug}/evidence"),
            1,
            "every run requires an evidence/ directory",
        ));
        return;
    }
    let mut files = BTreeSet::new();
    scan_evidence(root, &evidence, &mut files, violations);
    evidence_by_run.insert(run_slug.to_owned(), files);
}

fn scan_evidence(
    root: &Path,
    path: &Path,
    files: &mut BTreeSet<PathBuf>,
    violations: &mut Vec<Violation>,
) {
    for entry in sorted_entries(path) {
        let rel = entry.strip_prefix(root).unwrap_or(&entry).to_path_buf();
        let metadata = match std::fs::symlink_metadata(&entry) {
            Ok(metadata) => metadata,
            Err(error) => {
                violations.push(Violation::new(
                    "A003",
                    portable(&rel),
                    1,
                    format!("cannot inspect evidence entry: {error}"),
                ));
                continue;
            }
        };
        if metadata.file_type().is_symlink() {
            violations.push(Violation::new(
                "A003",
                portable(&rel),
                1,
                "evidence entries may not be symlinks",
            ));
        } else if metadata.is_dir() {
            scan_evidence(root, &entry, files, violations);
        } else if !metadata.is_file() {
            violations.push(Violation::new(
                "A003",
                portable(&rel),
                1,
                "evidence entries must be regular files",
            ));
        } else if entry
            .extension()
            .is_some_and(|extension| extension == "yamlld")
        {
            violations.push(Violation::new(
                "A003",
                portable(&rel),
                1,
                "evidence/ may not contain .yamlld records",
            ));
        } else {
            files.insert(rel);
        }
    }
}

fn parse_record(
    root: &Path,
    source: &Source,
    vocabulary: &Vocabulary,
    violations: &mut Vec<Violation>,
) -> Option<Document> {
    let rel = portable(&source.rel);
    let text = match std::fs::read_to_string(root.join(&source.rel)) {
        Ok(text) => text,
        Err(error) => {
            violations.push(Violation::new(
                "A004",
                rel,
                1,
                format!("record is not readable UTF-8 text: {error}"),
            ));
            return None;
        }
    };
    for forbidden in yaml_syntax::scan(text.as_bytes()) {
        let (line, message) = match forbidden {
            Forbidden::Anchor { line, name } => (
                line,
                format!("YAML anchors are forbidden (found `&{name}`)"),
            ),
            Forbidden::Alias { line, name } => (
                line,
                format!("YAML aliases are forbidden (found `*{name}`)"),
            ),
            Forbidden::MergeKey { line } => {
                (line, "YAML merge keys (`<<`) are forbidden".to_owned())
            }
            Forbidden::Tag { line } => (line, "explicit YAML tags are forbidden".to_owned()),
            Forbidden::Scanner { line } => (line, "YAML token scan failed".to_owned()),
        };
        violations.push(Violation::new("A004", &rel, line, message));
    }

    let value: Value = match serde_norway::from_str(&text) {
        Ok(value) => value,
        Err(error) => {
            violations.push(Violation::new(
                "A004",
                rel,
                yaml_error_line(&error.to_string()),
                format!("record does not parse as YAML 1.2: {error}"),
            ));
            return None;
        }
    };
    if !value.is_object() {
        violations.push(Violation::new(
            "A004",
            rel,
            1,
            "record must be a top-level YAML mapping",
        ));
        return None;
    }

    let type_iri = value.get("@type").and_then(Value::as_str);
    let kind = type_iri.and_then(|iri| vocabulary.kind_for_iri(iri));
    if let Some(kind) = kind
        && kind != source.expected_kind
    {
        violations.push(Violation::new(
            "A003",
            &rel,
            line_of(&text, "@type"),
            format!(
                "{} record is placed in the {} location",
                kind.name(),
                source.expected_kind.name()
            ),
        ));
    }

    let id = value.get("@id").and_then(Value::as_str).map(str::to_owned);
    if let Some(actual) = &id {
        let expected = expected_id(source);
        if actual != &expected {
            violations.push(Violation::new(
                "A003",
                &rel,
                line_of(&text, actual),
                format!("@id must match placement exactly: {expected}"),
            ));
        }
    }

    Some(Document {
        rel: source.rel.clone(),
        run_slug: source.run_slug.clone(),
        source: text,
        value,
        kind,
        id,
    })
}

fn check_format(document: &Document, violations: &mut Vec<Violation>) {
    let rel = document.rel_string();
    if document.source.contains('\r') {
        violations.push(Violation::new(
            "A005",
            &rel,
            1,
            "records use LF line endings only",
        ));
    }
    if document.source.contains('\t') {
        violations.push(Violation::new(
            "A005",
            &rel,
            line_of(&document.source, "\t"),
            "tab indentation is forbidden",
        ));
    }
    if !document.source.ends_with('\n') {
        violations.push(Violation::new(
            "A005",
            &rel,
            document.source.lines().count().max(1) as u32,
            "record must end with one newline",
        ));
    }
    for key in ["@context", "@id", "@type"] {
        if document.value.get(key).is_some()
            && !document
                .source
                .lines()
                .any(|line| line.trim_start().starts_with(&format!("\"{key}\":")))
        {
            violations.push(Violation::new(
                "A005",
                &rel,
                line_of(&document.source, key),
                format!("`{key}` must be a double-quoted YAML key"),
            ));
        }
    }
    if document.value.get("body").is_some() {
        let body_line = document
            .source
            .lines()
            .enumerate()
            .find(|(_, line)| line.trim_start().starts_with("body:"));
        match body_line {
            Some((_, line)) if valid_block_header(line.trim_start()) => {}
            Some((index, _)) => violations.push(Violation::new(
                "A005",
                &rel,
                index as u32 + 1,
                "body must use a literal or folded YAML block scalar (`|` or `>`) so prose remains reviewable",
            )),
            None => violations.push(Violation::new(
                "A005",
                &rel,
                1,
                "body must be authored as a YAML block scalar",
            )),
        }
    }
}

fn check_vocabulary(document: &Document, vocabulary: &Vocabulary, violations: &mut Vec<Violation>) {
    let rel = document.rel_string();
    if document.value.get("@context").and_then(Value::as_str) != Some(vocabulary.context.as_str()) {
        violations.push(Violation::new(
            "A006",
            &rel,
            line_of(&document.source, "@context"),
            format!(
                "@context must be the single offline context `{}`",
                vocabulary.context
            ),
        ));
    }
    let type_iri = document.value.get("@type").and_then(Value::as_str);
    if type_iri.is_none_or(|iri| vocabulary.kind_for_iri(iri).is_none()) {
        violations.push(Violation::new(
            "A006",
            &rel,
            line_of(&document.source, "@type"),
            format!(
                "@type is not a noun in vocabulary version {}; agents may not invent nouns",
                vocabulary.version
            ),
        ));
    }
    if document.value.get("schema_version").and_then(Value::as_u64)
        != Some(vocabulary.record_schema_version)
    {
        violations.push(Violation::new(
            "A006",
            &rel,
            line_of(&document.source, "schema_version"),
            format!(
                "schema_version must be {}",
                vocabulary.record_schema_version
            ),
        ));
    }

    let reserved: BTreeSet<&str> = ["@context", "@id", "@type"].into_iter().collect();
    if let Some(map) = document.value.as_object() {
        for key in map.keys() {
            if reserved.contains(key.as_str())
                || vocabulary.source_fields.contains(key)
                || vocabulary.verbs.contains_key(key)
            {
                continue;
            }
            violations.push(Violation::new(
                "A006",
                &rel,
                line_of(&document.source, key),
                format!(
                    "term `{key}` is not in vocabulary version {}; agents may not invent fields or verbs",
                    vocabulary.version
                ),
            ));
        }
    }

    if let Some(kind) = document.kind {
        for (name, spec) in &vocabulary.verbs {
            if document.value.get(name).is_some() && !spec.domain.contains(kind.name()) {
                violations.push(Violation::new(
                    "A006",
                    &rel,
                    line_of(&document.source, name),
                    format!("verb `{name}` cannot have {} as its subject", kind.name()),
                ));
            }
        }
    }
}

fn check_shape(document: &Document, validator: &Validator, violations: &mut Vec<Violation>) {
    for error in validator.iter_errors(&document.value) {
        let instance_path = error.instance_path().to_string();
        let needle = instance_path
            .rsplit('/')
            .next()
            .filter(|part| !part.is_empty())
            .unwrap_or("@type");
        violations.push(Violation::new(
            "A007",
            document.rel_string(),
            line_of(&document.source, needle),
            format!("shape violation at {instance_path}: {error}"),
        ));
    }
}

fn check_edges(
    root: &Path,
    documents: &[Document],
    vocabulary: &Vocabulary,
    violations: &mut Vec<Violation>,
) {
    let mut by_id: BTreeMap<&str, &Document> = BTreeMap::new();
    for document in documents {
        let Some(id) = document.id.as_deref() else {
            continue;
        };
        if let Some(first) = by_id.insert(id, document) {
            violations.push(Violation::new(
                "A008",
                document.rel_string(),
                line_of(&document.source, id),
                format!("duplicate @id; first declared at {}", first.rel_string()),
            ));
        }
    }

    for document in documents {
        let Some(subject_kind) = document.kind else {
            continue;
        };
        for (verb, spec) in &vocabulary.verbs {
            for target in document.edge_values(verb) {
                if verb == "resolves_to" {
                    check_path_edge(root, document, target, violations);
                    continue;
                }
                let Some(target_document) = by_id.get(target).copied() else {
                    violations.push(Violation::new(
                        "A008",
                        document.rel_string(),
                        line_of(&document.source, target),
                        format!("verb `{verb}` points at missing vertex {target}"),
                    ));
                    continue;
                };
                let Some(target_kind) = target_document.kind else {
                    continue;
                };
                let range_ok = spec.range.contains(target_kind.name())
                    || (spec.range.contains("SameNoun") && target_kind == subject_kind);
                if !range_ok {
                    violations.push(Violation::new(
                        "A008",
                        document.rel_string(),
                        line_of(&document.source, target),
                        format!(
                            "verb `{verb}` requires range {:?}, got {}",
                            spec.range,
                            target_kind.name()
                        ),
                    ));
                }
            }
        }
    }
}

fn check_path_edge(
    root: &Path,
    document: &Document,
    pointer: &str,
    violations: &mut Vec<Violation>,
) {
    let Some(path) = pointer_path(pointer) else {
        violations.push(Violation::new(
            "A008",
            document.rel_string(),
            line_of(&document.source, pointer),
            format!("path edge must use `{PATH_PREFIX}<safe-repo-relative-path>`"),
        ));
        return;
    };
    let target = root.join(&path);
    if !target.is_file() {
        violations.push(Violation::new(
            "A008",
            document.rel_string(),
            line_of(&document.source, pointer),
            format!("path edge target `{}` is not a real file", portable(&path)),
        ));
        return;
    }
    if !canonical_within(root, &target) {
        violations.push(Violation::new(
            "A008",
            document.rel_string(),
            line_of(&document.source, pointer),
            format!(
                "path edge target `{}` escapes the repository",
                portable(&path)
            ),
        ));
    }
}

fn check_runs(
    root: &Path,
    documents: &[Document],
    run_slugs: &BTreeSet<String>,
    evidence_by_run: &BTreeMap<String, BTreeSet<PathBuf>>,
    bindings: Option<&Bindings>,
    violations: &mut Vec<Violation>,
) {
    for run_slug in run_slugs {
        let records: Vec<&Document> = documents
            .iter()
            .filter(|document| &document.run_slug == run_slug)
            .collect();
        let run_records: Vec<&Document> = records
            .iter()
            .copied()
            .filter(|document| document.kind == Some(RecordKind::Run))
            .collect();
        if run_records.len() != 1 {
            violations.push(Violation::new(
                "A009",
                format!("{MOUNT_REL}/runs/{run_slug}"),
                1,
                format!(
                    "run must contain one usable Run record, found {}",
                    run_records.len()
                ),
            ));
            continue;
        }
        let run = run_records[0];
        let run_id = run.id.as_deref().unwrap_or_default();
        for kind in [RecordKind::Promise, RecordKind::Witness, RecordKind::Oracle] {
            let count = records
                .iter()
                .filter(|document| document.kind == Some(kind))
                .count();
            if count == 0 {
                violations.push(Violation::new(
                    "A009",
                    format!("{MOUNT_REL}/runs/{run_slug}"),
                    1,
                    format!("run must contain at least one {} record", kind.name()),
                ));
            }
        }

        for document in &records {
            if document.kind == Some(RecordKind::Run) {
                continue;
            }
            let part_of = document.value.get("part_of").and_then(Value::as_str);
            if part_of != Some(run_id) {
                violations.push(Violation::new(
                    "A009",
                    document.rel_string(),
                    line_of(&document.source, "part_of"),
                    format!("part_of must name this folder's Run vertex {run_id}"),
                ));
            }
        }

        for promise in records
            .iter()
            .copied()
            .filter(|document| document.kind == Some(RecordKind::Promise))
        {
            for verb in ["witnessed_by", "judged_by"] {
                for target in promise.edge_values(verb) {
                    if documents
                        .iter()
                        .find(|document| document.id.as_deref() == Some(target))
                        .is_some_and(|document| document.run_slug != *run_slug)
                    {
                        violations.push(Violation::new(
                            "A009",
                            promise.rel_string(),
                            line_of(&promise.source, target),
                            format!("{verb} must stay inside the Promise's run"),
                        ));
                    }
                }
            }
        }

        let evidence_files = evidence_by_run.get(run_slug).cloned().unwrap_or_default();
        let mut witnessed_files = BTreeSet::new();
        for witness in records
            .iter()
            .copied()
            .filter(|document| document.kind == Some(RecordKind::Witness))
        {
            check_witness_digest(root, witness, violations);
            let Some(pointer) = witness.value.get("resolves_to").and_then(Value::as_str) else {
                continue;
            };
            let Some(path) = pointer_path(pointer) else {
                continue;
            };
            let expected_root = PathBuf::from(format!("{MOUNT_REL}/runs/{run_slug}/evidence"));
            if !path.starts_with(&expected_root) {
                violations.push(Violation::new(
                    "A009",
                    witness.rel_string(),
                    line_of(&witness.source, pointer),
                    format!(
                        "Witness resolves_to must target a same-run committed file under {MOUNT_REL}/runs/{run_slug}/evidence/"
                    ),
                ));
                continue;
            }
            if evidence_files.contains(&path) {
                witnessed_files.insert(path.clone());
                if path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| name.ends_with(".external-artifact.yaml"))
                {
                    check_external_artifact_manifest(root, witness, &path, violations);
                }
            }
        }
        for path in evidence_files.difference(&witnessed_files) {
            violations.push(Violation::new(
                "A009",
                portable(path),
                1,
                "committed evidence file is not the resolves_to target of a same-run Witness",
            ));
        }
        if let Some(bindings) = bindings {
            for oracle in records
                .iter()
                .copied()
                .filter(|document| document.kind == Some(RecordKind::Oracle))
            {
                if let Some(actor) = oracle.value.get("actor").and_then(Value::as_str) {
                    let resolved = bindings.resolve(actor).unwrap_or(actor);
                    if !bindings.actor_values().contains(resolved) {
                        violations.push(Violation::new(
                            "A009",
                            oracle.rel_string(),
                            line_of(&oracle.source, actor),
                            "Oracle actor is not bound to reviewer_seat or final_validator_seat in assurance-init.yaml",
                        ));
                    }
                }
            }
        }
    }
}

fn check_generated(root: &Path, documents: &[Document], violations: &mut Vec<Violation>) {
    let compiled = match graph::compile_all(documents) {
        Ok(compiled) => compiled,
        Err(error) => {
            violations.push(Violation::new(
                "A002",
                GRAPH_MANIFEST_REL,
                1,
                format!("cannot regenerate deterministic graphs: {error}"),
            ));
            return;
        }
    };
    for (run_slug, expected) in &compiled.runs {
        let rel = graph::graph_rel(run_slug);
        let path = root.join(&rel);
        let metadata = match std::fs::symlink_metadata(&path) {
            Ok(metadata) => metadata,
            Err(_) => {
                violations.push(Violation::new(
                    "A002",
                    rel,
                    1,
                    "generated run graph is missing; run `assurance build` and commit it",
                ));
                continue;
            }
        };
        if metadata.file_type().is_symlink() || !metadata.is_file() {
            violations.push(Violation::new(
                "A002",
                rel,
                1,
                "generated run graph must be a regular non-symlink file",
            ));
            continue;
        }
        match std::fs::read(&path) {
            Ok(actual) if actual == *expected => {}
            Ok(_) => violations.push(Violation::new(
                "A002",
                rel,
                1,
                "committed run graph differs from deterministic record projection; run `assurance build` and commit it",
            )),
            Err(error) => violations.push(Violation::new(
                "A002",
                rel,
                1,
                format!("cannot read generated run graph: {error}"),
            )),
        }
    }

    let manifest_path = root.join(GRAPH_MANIFEST_REL);
    let metadata = match std::fs::symlink_metadata(&manifest_path) {
        Ok(metadata) => metadata,
        Err(_) => {
            violations.push(Violation::new(
                "A002",
                GRAPH_MANIFEST_REL,
                1,
                "graph manifest is missing; run `assurance build` and commit it",
            ));
            return;
        }
    };
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        violations.push(Violation::new(
            "A002",
            GRAPH_MANIFEST_REL,
            1,
            "graph manifest must be a regular non-symlink file owned by assurance build",
        ));
        return;
    }
    match std::fs::read(&manifest_path) {
        Ok(actual) if actual == compiled.manifest => {}
        Ok(_) => violations.push(Violation::new(
            "A002",
            GRAPH_MANIFEST_REL,
            1,
            "graph manifest differs from sorted deterministic run graph paths and digests; run `assurance build` and commit it",
        )),
        Err(error) => violations.push(Violation::new(
            "A002",
            GRAPH_MANIFEST_REL,
            1,
            format!("cannot read graph manifest: {error}"),
        )),
    }
}

fn check_external_artifact_manifest(
    root: &Path,
    _witness: &Document,
    path: &Path,
    violations: &mut Vec<Violation>,
) {
    let rel = portable(path);
    let source = match std::fs::read_to_string(root.join(path)) {
        Ok(source) => source,
        Err(error) => {
            violations.push(Violation::new(
                "A009",
                rel,
                1,
                format!("external-artifact manifest is not readable UTF-8 YAML: {error}"),
            ));
            return;
        }
    };
    let value: Value = match serde_norway::from_str(&source) {
        Ok(value) => value,
        Err(error) => {
            violations.push(Violation::new(
                "A009",
                rel,
                yaml_error_line(&error.to_string()),
                format!("external-artifact manifest is not valid YAML: {error}"),
            ));
            return;
        }
    };
    let Some(map) = value.as_object() else {
        violations.push(Violation::new(
            "A009",
            rel,
            1,
            "external-artifact manifest must be a mapping",
        ));
        return;
    };
    let allowed: BTreeSet<&str> = ["version", "external_uri", "sha256", "size", "provenance"]
        .into_iter()
        .collect();
    for key in map.keys() {
        if !allowed.contains(key.as_str()) {
            violations.push(Violation::new(
                "A009",
                &rel,
                line_of(&source, key),
                format!("unknown external-artifact manifest field `{key}`"),
            ));
        }
    }
    if map.get("version").and_then(Value::as_u64) != Some(1) {
        violations.push(Violation::new(
            "A009",
            &rel,
            line_of(&source, "version"),
            "external-artifact manifest version must be 1",
        ));
    }
    if !map
        .get("external_uri")
        .and_then(Value::as_str)
        .is_some_and(valid_external_uri)
    {
        violations.push(Violation::new(
            "A009",
            &rel,
            line_of(&source, "external_uri"),
            "external_uri must be an immutable absolute non-file URI",
        ));
    }
    if !map
        .get("sha256")
        .and_then(Value::as_str)
        .is_some_and(|digest| is_lower_hex(digest, 64))
    {
        violations.push(Violation::new(
            "A009",
            &rel,
            line_of(&source, "sha256"),
            "external payload sha256 must be 64 lowercase hexadecimal characters",
        ));
    }
    if map.get("size").and_then(Value::as_u64).is_none() {
        violations.push(Violation::new(
            "A009",
            &rel,
            line_of(&source, "size"),
            "external payload size must be a non-negative integer",
        ));
    }
    if map
        .get("provenance")
        .and_then(Value::as_str)
        .is_none_or(|value| value.trim().is_empty())
    {
        violations.push(Violation::new(
            "A009",
            rel,
            line_of(&source, "provenance"),
            "external payload provenance must be non-empty",
        ));
    }
}

fn check_witness_digest(root: &Path, witness: &Document, violations: &mut Vec<Violation>) {
    let Some(pointer) = witness.value.get("resolves_to").and_then(Value::as_str) else {
        return;
    };
    let Some(path) = pointer_path(pointer) else {
        return;
    };
    let Ok(bytes) = std::fs::read(root.join(&path)) else {
        return;
    };
    let expected = witness
        .value
        .get("artifact_sha256")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let actual = graph::sha256_hex(&bytes);
    if expected != actual {
        violations.push(Violation::new(
            "A009",
            witness.rel_string(),
            line_of(&witness.source, "artifact_sha256"),
            format!(
                "witness digest mismatch for `{}`: expected {expected}, actual {actual}",
                portable(&path)
            ),
        ));
    }
}

fn check_transitions(
    documents: &[Document],
    vocabulary: &Vocabulary,
    violations: &mut Vec<Violation>,
) {
    let by_id: BTreeMap<&str, &Document> = documents
        .iter()
        .filter_map(|document| document.id.as_deref().map(|id| (id, document)))
        .collect();

    for document in documents {
        let successor_id = document.value.get("succeeded_by").and_then(Value::as_str);
        if document.value.get("disposition").and_then(Value::as_str) == Some("BLOCKED")
            && successor_id.is_some()
        {
            violations.push(Violation::new(
                "A010",
                document.rel_string(),
                line_of(&document.source, "succeeded_by"),
                "BLOCKED is terminal; progression after BLOCKED is forbidden",
            ));
        }
        let Some(successor_id) = successor_id else {
            continue;
        };
        let Some(successor) = by_id.get(successor_id).copied() else {
            continue;
        };
        if successor
            .sequence()
            .zip(document.sequence())
            .is_some_and(|(next, current)| next <= current)
        {
            violations.push(Violation::new(
                "A010",
                document.rel_string(),
                line_of(&document.source, successor_id),
                "succeeded_by must point to a strictly greater sequence",
            ));
        }

        if document.kind == Some(RecordKind::Run) {
            if document.value.get("state").and_then(Value::as_str) != Some("CLOSED") {
                violations.push(Violation::new(
                    "A010",
                    document.rel_string(),
                    line_of(&document.source, "succeeded_by"),
                    "only a CLOSED Run may declare a successor",
                ));
            }
            let source_lane = document.value.get("lane").and_then(Value::as_str);
            let target_lane = successor.value.get("lane").and_then(Value::as_str);
            if let (Some(source_lane), Some(target_lane)) = (source_lane, target_lane)
                && source_lane != target_lane
            {
                if !vocabulary
                    .admission_edges
                    .contains(&(source_lane.to_owned(), target_lane.to_owned()))
                {
                    violations.push(Violation::new(
                        "A010",
                        document.rel_string(),
                        line_of(&document.source, successor_id),
                        format!(
                            "lane chronology {source_lane} -> {target_lane} is not admitted by vocabulary version {}",
                            vocabulary.version
                        ),
                    ));
                }
                let non_pass = documents.iter().any(|candidate| {
                    candidate.run_slug == document.run_slug
                        && candidate.kind == Some(RecordKind::Oracle)
                        && candidate.value.get("disposition").and_then(Value::as_str)
                            != Some("PASS")
                });
                if non_pass {
                    violations.push(Violation::new(
                        "A010",
                        document.rel_string(),
                        line_of(&document.source, successor_id),
                        "cross-lane progression requires every Oracle in the source run to be PASS",
                    ));
                }
            }
        }
    }

    for run in documents
        .iter()
        .filter(|document| document.kind == Some(RecordKind::Run))
    {
        let lane = run.value.get("lane").and_then(Value::as_str);
        if lane != Some("7000") {
            violations.push(Violation::new(
                "A010",
                run.rel_string(),
                line_of(&run.source, "lane"),
                "this pack exercises lane 7000 only; other vocabulary lanes are reserved",
            ));
        }
        if lane.is_some_and(|lane| vocabulary.human_invoked_lanes.contains(lane))
            && run.value.get("human_invoked").and_then(Value::as_bool) != Some(true)
        {
            violations.push(Violation::new(
                "A010",
                run.rel_string(),
                line_of(&run.source, "human_invoked"),
                "lane 7000 is human-invoked and requires human_invoked: true",
            ));
        }
        if run.source.contains("complete-awaiting-promotion")
            && run.value.get("state").and_then(Value::as_str) != Some("OPEN")
        {
            violations.push(Violation::new(
                "A010",
                run.rel_string(),
                line_of(&run.source, "state"),
                "complete-awaiting-promotion is a body marker while Run state remains OPEN",
            ));
        }
        let has_blocked_oracle = documents.iter().any(|candidate| {
            candidate.run_slug == run.run_slug
                && candidate.kind == Some(RecordKind::Oracle)
                && candidate.value.get("disposition").and_then(Value::as_str) == Some("BLOCKED")
        });
        if has_blocked_oracle && run.value.get("succeeded_by").is_some() {
            violations.push(Violation::new(
                "A010",
                run.rel_string(),
                line_of(&run.source, "succeeded_by"),
                "Run contains a BLOCKED Oracle; progression is forbidden",
            ));
        }
    }
}

fn expected_id(source: &Source) -> String {
    let base = format!(
        "{RECORD_PREFIX}{}/{}",
        source.run_slug,
        source.expected_kind.id_segment()
    );
    if source.expected_kind == RecordKind::Run {
        base
    } else {
        let stem = source
            .rel
            .file_stem()
            .and_then(|value| value.to_str())
            .unwrap_or_default();
        format!("{base}/{stem}")
    }
}

fn nested_string<'a>(value: &'a Value, path: &[&str]) -> Option<&'a str> {
    let mut current = value;
    for key in path {
        current = current.get(*key)?;
    }
    current.as_str()
}

fn configured_value(value: &str) -> bool {
    let value = value.trim();
    !value.is_empty()
        && value != "UNCONFIGURED"
        && !value.starts_with("REPLACE_WITH_")
        && !value.contains("{{")
        && !value.contains("}}")
}

fn valid_variable_name(value: &str) -> bool {
    let mut bytes = value.bytes();
    bytes.next().is_some_and(|byte| byte.is_ascii_lowercase())
        && bytes.all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
}

fn valid_repository(value: &str) -> bool {
    configured_value(value)
        && value.split('/').count() == 2
        && value
            .split('/')
            .all(|part| !part.is_empty() && part.chars().all(valid_coordinate_char))
}

fn valid_coordinate_char(character: char) -> bool {
    character.is_ascii_alphanumeric() || matches!(character, '_' | '-' | '.')
}

fn valid_runner_label(value: &str) -> bool {
    !value.is_empty() && value.chars().all(valid_coordinate_char)
}

fn is_lower_hex(value: &str, length: usize) -> bool {
    value.len() == length
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn valid_external_uri(value: &str) -> bool {
    if value.starts_with("file:") || value.chars().any(char::is_whitespace) {
        return false;
    }
    let Some((scheme, remainder)) = value.split_once(':') else {
        return false;
    };
    !remainder.is_empty()
        && scheme
            .bytes()
            .next()
            .is_some_and(|byte| byte.is_ascii_alphabetic())
        && scheme
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'+' | b'-' | b'.'))
}

fn valid_slug(value: &str) -> bool {
    !value.is_empty()
        && !value.starts_with('-')
        && !value.ends_with('-')
        && !value.contains("--")
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
}

fn valid_block_header(line: &str) -> bool {
    let Some(style) = line.strip_prefix("body:").map(str::trim) else {
        return false;
    };
    matches!(style, "|" | "|-" | "|+" | ">" | ">-" | ">+")
}

fn canonical_within(root: &Path, target: &Path) -> bool {
    let Ok(root) = std::fs::canonicalize(root) else {
        return false;
    };
    let Ok(target) = std::fs::canonicalize(target) else {
        return false;
    };
    target.starts_with(root)
}

fn sorted_entries(directory: &Path) -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = std::fs::read_dir(directory)
        .map(|entries| entries.flatten().map(|entry| entry.path()).collect())
        .unwrap_or_default();
    paths.sort();
    paths
}

fn file_name(path: &Path) -> String {
    path.file_name()
        .map(|value| value.to_string_lossy().into_owned())
        .unwrap_or_default()
}

fn rel_to(root: &Path, path: &Path) -> String {
    portable(path.strip_prefix(root).unwrap_or(path))
}

fn yaml_error_line(message: &str) -> u32 {
    let words: Vec<&str> = message.split_whitespace().collect();
    words
        .windows(2)
        .find_map(|pair| {
            (pair[0] == "line")
                .then(|| pair[1].trim_end_matches([',', ':']))
                .and_then(|value| value.parse().ok())
        })
        .unwrap_or(1)
}
