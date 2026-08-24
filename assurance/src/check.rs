//! `assurance check`: deterministic, fail-hard validation of `.assurance/`.
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
use crate::model::{Document, PATH_PREFIX, RecordKind, Vocabulary, pointer_path, portable};
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
}

struct Source {
    rel: PathBuf,
    run_slug: String,
    expected_kind: RecordKind,
}

#[derive(Default)]
struct Bindings {
    actors: BTreeSet<String>,
}

pub fn inspect(root: &Path) -> Result<Report, Fatal> {
    let vocabulary = Vocabulary::embedded()?;
    let validator = record_validator()?;
    let mut violations = Vec::new();

    let assurance = root.join(".assurance");
    if !assurance.is_dir() {
        violations.push(Violation::new(
            "A001",
            ".assurance",
            1,
            "missing adoption state; run `assurance init`, complete its one binding question, and commit the result",
        ));
        return Ok(Report {
            violations,
            documents: Vec::new(),
        });
    }

    let bindings = check_bindings(root, &mut violations);
    check_canonical_files(root, &mut violations);
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

    check_edges(root, &documents, &vocabulary, &mut violations);
    check_runs(
        root,
        &documents,
        &layout.run_slugs,
        bindings.as_ref(),
        &mut violations,
    );
    check_transitions(&documents, &vocabulary, &mut violations);

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

fn check_bindings(root: &Path, violations: &mut Vec<Violation>) -> Option<Bindings> {
    let rel = ".assurance/assurance-init.yaml";
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

    let expected: BTreeSet<&str> = [
        "version",
        "status",
        "pack",
        "models",
        "harnesses",
        "runners",
        "actors",
    ]
    .into_iter()
    .collect();
    for key in map.keys() {
        if !expected.contains(key.as_str()) {
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
        violations.push(Violation::new(
            "A001",
            rel,
            line_of(&source, "version"),
            "version must be 1",
        ));
    }
    if map.get("status").and_then(Value::as_str) != Some("CONFIGURED") {
        violations.push(Violation::new(
            "A001",
            rel,
            line_of(&source, "status"),
            "status is not CONFIGURED; ask the human the bootstrap questions, replace every placeholder, then set `status: CONFIGURED`",
        ));
        return None;
    }

    let mut valid = true;
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

    for role in ["lead", "worker", "validator", "reviewer"] {
        if !nested_string(&value, &["models", role]).is_some_and(configured_value) {
            valid = false;
            violations.push(Violation::new(
                "A001",
                rel,
                line_of(&source, role),
                format!("models.{role} must name the human-selected model"),
            ));
        }
    }

    let harnesses_valid = value
        .get("harnesses")
        .and_then(Value::as_array)
        .is_some_and(|items| {
            !items.is_empty()
                && items
                    .iter()
                    .all(|item| item.as_str().is_some_and(configured_value))
        });
    if !harnesses_valid {
        valid = false;
        violations.push(Violation::new(
            "A001",
            rel,
            line_of(&source, "harnesses"),
            "harnesses must contain at least one human-selected harness",
        ));
    }

    let runner = nested_string(&value, &["runners", "witness"]);
    if !runner.is_some_and(|value| configured_value(value) && valid_runner_label(value)) {
        valid = false;
        violations.push(Violation::new(
            "A001",
            rel,
            line_of(&source, "witness"),
            "runners.witness must be one literal configured runner label",
        ));
    }

    let mut actors = BTreeSet::new();
    for seat in ["final_validator", "reviewer"] {
        match nested_string(&value, &["actors", seat]) {
            Some(actor) if configured_value(actor) => {
                actors.insert(actor.to_owned());
            }
            _ => {
                valid = false;
                violations.push(Violation::new(
                    "A001",
                    rel,
                    line_of(&source, seat),
                    format!("actors.{seat} must name the human-selected actor"),
                ));
            }
        }
    }

    valid.then_some(Bindings { actors })
}

fn check_canonical_files(root: &Path, violations: &mut Vec<Violation>) {
    let files = [
        (".assurance/schema/context.yamlld", embedded::CONTEXT),
        (".assurance/schema/vocabulary.yaml", embedded::VOCABULARY),
        (
            ".assurance/schema/records.schema.json",
            embedded::RECORD_SCHEMA,
        ),
        (".github/workflows/assurance.yml", embedded::WORKFLOW),
    ];
    for (rel, canonical) in files {
        match std::fs::read(root.join(rel)) {
            Ok(bytes) if bytes == canonical.as_bytes() => {}
            Ok(_) => violations.push(Violation::new(
                "A002",
                rel,
                1,
                "materialized pack file differs from this checker's canonical version; adopt a versioned pack change instead of editing the local copy",
            )),
            Err(_) => violations.push(Violation::new(
                "A002",
                rel,
                1,
                "canonical materialized pack file is missing; rerun `assurance init` in a clean adoption",
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
    let rel = ".assurance/registry.yaml";
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
    let assurance = root.join(".assurance");
    let allowed_root: BTreeSet<&str> = ["assurance-init.yaml", "registry.yaml", "schema", "runs"]
        .into_iter()
        .collect();
    for path in sorted_entries(&assurance) {
        let name = file_name(&path);
        if !allowed_root.contains(name.as_str()) {
            violations.push(Violation::new(
                "A003",
                format!(".assurance/{name}"),
                1,
                "unexpected adoption entry; only assurance-init.yaml, registry.yaml, schema/, and runs/ are allowed",
            ));
        }
    }

    let schema = assurance.join("schema");
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
                    format!(".assurance/schema/{name}"),
                    1,
                    "unexpected schema entry",
                ));
            }
        }
    }

    let runs = assurance.join("runs");
    let mut layout = Layout::default();
    if !runs.is_dir() {
        violations.push(Violation::new(
            "A003",
            ".assurance/runs",
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
        let run_rel = format!(".assurance/runs/{run_slug}");
        if !run_path.is_dir() {
            violations.push(Violation::new(
                "A003",
                run_rel,
                1,
                "runs/ may contain only kebab-case run directories and .gitkeep",
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
        scan_run(root, &run_path, &run_slug, &mut layout.sources, violations);
    }
    layout
}

fn scan_run(
    root: &Path,
    run_path: &Path,
    run_slug: &str,
    sources: &mut Vec<Source>,
    violations: &mut Vec<Violation>,
) {
    let allowed: BTreeSet<&str> = [
        "run.yamlld",
        "promises",
        "witnesses",
        "oracles",
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
                "unexpected run entry; expected run.yamlld, the triad directories, and optional graph.trig",
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
            format!(".assurance/runs/{run_slug}/run.yamlld"),
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
                format!(".assurance/runs/{run_slug}/{directory}"),
                1,
                format!("every run requires a flat {directory}/ directory"),
            ));
            continue;
        }
        for entry in sorted_entries(&path) {
            let name = file_name(&entry);
            let rel = rel_to(root, &entry);
            if entry.is_dir() {
                violations.push(Violation::new(
                    "A003",
                    rel,
                    1,
                    format!("{directory}/ is flat; nested directories are forbidden"),
                ));
                continue;
            }
            if !name.ends_with(".yamlld") {
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
            "@type is not a noun in vocabulary version 1; agents may not invent nouns",
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
                format!("term `{key}` is not in vocabulary version 1; agents may not invent fields or verbs"),
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
                format!(".assurance/runs/{run_slug}"),
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
                    format!(".assurance/runs/{run_slug}"),
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

        for witness in records
            .iter()
            .copied()
            .filter(|document| document.kind == Some(RecordKind::Witness))
        {
            check_witness_digest(root, witness, violations);
        }
        if let Some(bindings) = bindings {
            for oracle in records
                .iter()
                .copied()
                .filter(|document| document.kind == Some(RecordKind::Oracle))
            {
                if let Some(actor) = oracle.value.get("actor").and_then(Value::as_str)
                    && !bindings.actors.contains(actor)
                {
                    violations.push(Violation::new(
                        "A009",
                        oracle.rel_string(),
                        line_of(&oracle.source, actor),
                        "Oracle actor is not bound to final_validator or reviewer in assurance-init.yaml",
                    ));
                }
            }
        }
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
                        format!("lane chronology {source_lane} -> {target_lane} is not admitted by vocabulary version 1"),
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
        "https://adversarial-assurance.dev/record/{}/{}",
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
    !value.trim().is_empty() && !value.starts_with("REPLACE_WITH_")
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
