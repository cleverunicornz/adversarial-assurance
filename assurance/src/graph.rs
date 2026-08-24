//! Offline JSON-LD expansion and deterministic per-run TriG projection.
//!
//! Provenance: the YAML-to-JSON-LD, sorted-quad, oxttl pipeline follows
//! bedrock's published compiler shape; prose projection and run semantics are
//! assurance-specific and implemented here from scratch.

use crate::embedded;
use crate::error::{Fatal, Violation};
use crate::model::{Document, GRAPH_MANIFEST_REL, GRAPH_PREFIX, MOUNT_REL, PATH_PREFIX, ROOT_IRI};
use oxjsonld::{JsonLdParser, JsonLdRemoteDocument};
use oxrdf::{GraphName, Literal, NamedNode, Quad};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::path::Path;

const PREFIXES: [(&str, &str); 4] = [
    ("rdf", "http://www.w3.org/1999/02/22-rdf-syntax-ns#"),
    ("xsd", "http://www.w3.org/2001/XMLSchema#"),
    ("aa", ROOT_IRI),
    ("graph", GRAPH_PREFIX),
];

pub fn expand(document: &Document) -> Result<Vec<Quad>, Violation> {
    let rel = document.rel_string();
    let mut projected = document.value.clone();
    let Some(root) = projected.as_object_mut() else {
        return Err(Violation::new(
            "A006",
            rel,
            1,
            "JSON-LD record must be a mapping",
        ));
    };
    root.remove("body");

    let json = serde_json::to_vec(&projected).expect("JSON value serializes");
    let context: serde_json::Value =
        serde_norway::from_str(embedded::CONTEXT).expect("embedded context is valid YAML");
    let parser = JsonLdParser::new()
        .with_base_iri(ROOT_IRI)
        .expect("static base IRI is valid")
        .for_slice(&json)
        .with_load_document_callback(move |url, _options| {
            if url == crate::model::CONTEXT_IRI {
                Ok(JsonLdRemoteDocument {
                    document: serde_json::to_vec(&context).expect("context serializes"),
                    document_url: url.to_owned(),
                })
            } else {
                Err(format!("remote context loading is disabled; {url} is not served").into())
            }
        });

    let graph_name = NamedNode::new(format!("{GRAPH_PREFIX}{}", document.run_slug))
        .expect("validated run slug makes a valid graph IRI");
    let mut quads = Vec::new();
    for result in parser {
        let mut quad = result.map_err(|error| {
            Violation::new(
                "A006",
                document.rel_string(),
                crate::error::line_of(&document.source, "@context"),
                format!("JSON-LD expansion failed: {error}"),
            )
        })?;
        if matches!(quad.subject, oxrdf::NamedOrBlankNode::BlankNode(_))
            || matches!(quad.object, oxrdf::Term::BlankNode(_))
            || matches!(quad.graph_name, GraphName::BlankNode(_))
        {
            return Err(Violation::new(
                "A006",
                document.rel_string(),
                crate::error::line_of(&document.source, "@id"),
                "blank nodes are forbidden",
            ));
        }
        if !matches!(quad.graph_name, GraphName::DefaultGraph) {
            return Err(Violation::new(
                "A006",
                document.rel_string(),
                crate::error::line_of(&document.source, "@id"),
                "authored named graphs are forbidden; run-folder placement owns graph membership",
            ));
        }
        quad.graph_name = graph_name.clone().into();
        quads.push(quad);
    }

    let id = document.id.as_deref().ok_or_else(|| {
        Violation::new(
            "A006",
            document.rel_string(),
            crate::error::line_of(&document.source, "@id"),
            "record has no usable @id",
        )
    })?;
    let subject = NamedNode::new(id).map_err(|error| {
        Violation::new(
            "A006",
            document.rel_string(),
            crate::error::line_of(&document.source, id),
            format!("@id is not a valid IRI: {error}"),
        )
    })?;
    let source_pointer = NamedNode::new(format!("{PATH_PREFIX}{}", document.rel_string()))
        .expect("validated record path makes a valid pointer IRI");
    let resolves_to = NamedNode::new(format!("{ROOT_IRI}resolves-to")).expect("static IRI");
    let content_digest = NamedNode::new(format!("{ROOT_IRI}content-digest")).expect("static IRI");
    quads.push(Quad::new(
        subject.clone(),
        resolves_to,
        source_pointer,
        graph_name.clone(),
    ));
    quads.push(Quad::new(
        subject,
        content_digest,
        Literal::new_simple_literal(format!("sha256:{}", sha256_hex(document.source.as_bytes()))),
        graph_name,
    ));

    Ok(quads)
}

pub fn compile_run(run_slug: &str, documents: &[Document]) -> Result<Vec<u8>, Fatal> {
    let mut quads = Vec::new();
    for document in documents
        .iter()
        .filter(|document| document.run_slug == run_slug)
    {
        quads.extend(expand(document).map_err(|violation| Fatal(violation.to_string()))?);
    }
    let sorted = sort_dedup(quads);
    let bytes = serialize(&sorted)?;
    let parsed = parse_back(&bytes)?;
    if sort_dedup(parsed) != sorted {
        return Err(Fatal(format!(
            "assurance build: parse-back dataset differs for run {run_slug}"
        )));
    }
    Ok(bytes)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompiledGraphs {
    pub runs: BTreeMap<String, Vec<u8>>,
    pub manifest: Vec<u8>,
}

pub fn compile_all(documents: &[Document]) -> Result<CompiledGraphs, Fatal> {
    let mut runs = BTreeMap::new();
    for document in documents {
        runs.entry(document.run_slug.clone())
            .or_insert_with(Vec::new);
    }
    for (run_slug, bytes) in &mut runs {
        *bytes = compile_run(run_slug, documents)?;
    }
    let manifest = render_manifest(&runs).into_bytes();
    Ok(CompiledGraphs { runs, manifest })
}

pub fn graph_rel(run_slug: &str) -> String {
    format!("{MOUNT_REL}/runs/{run_slug}/graph.trig")
}

pub fn render_manifest(runs: &BTreeMap<String, Vec<u8>>) -> String {
    if runs.is_empty() {
        return "version: 1\ngraphs: []\n".to_owned();
    }
    let mut output = String::from("version: 1\ngraphs:\n");
    for (run_slug, bytes) in runs {
        output.push_str(&format!(
            "  - path: \"{}\"\n    sha256: \"{}\"\n",
            graph_rel(run_slug),
            sha256_hex(bytes)
        ));
    }
    output
}

pub fn manifest_from_disk(root: &Path) -> Result<Vec<u8>, Fatal> {
    let runs_dir = root.join(MOUNT_REL).join("runs");
    let mut runs = BTreeMap::new();
    let mut entries: Vec<_> = std::fs::read_dir(&runs_dir)
        .map_err(|error| {
            Fatal(format!(
                "assurance update: cannot read {}: {error}",
                runs_dir.display()
            ))
        })?
        .flatten()
        .map(|entry| entry.path())
        .collect();
    entries.sort();
    for run_dir in entries {
        let metadata = std::fs::symlink_metadata(&run_dir).map_err(|error| {
            Fatal(format!(
                "assurance update: cannot inspect {}: {error}",
                run_dir.display()
            ))
        })?;
        if metadata.file_type().is_symlink() || !metadata.is_dir() {
            continue;
        }
        let Some(run_slug) = run_dir.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        let graph = run_dir.join("graph.trig");
        if graph.is_file() {
            runs.insert(
                run_slug.to_owned(),
                std::fs::read(&graph).map_err(|error| {
                    Fatal(format!(
                        "assurance update: cannot read {}: {error}",
                        graph.display()
                    ))
                })?,
            );
        }
    }
    Ok(render_manifest(&runs).into_bytes())
}

pub fn write_all(root: &Path, documents: &[Document]) -> Result<usize, Fatal> {
    let compiled = compile_all(documents)?;
    for (run_slug, bytes) in &compiled.runs {
        let rel = graph_rel(run_slug);
        let path = root.join(&rel);
        std::fs::write(&path, bytes).map_err(|error| {
            Fatal(format!(
                "assurance build: cannot write {}: {error}",
                path.display()
            ))
        })?;
        println!("assurance build: {rel} sha256 {}", sha256_hex(bytes));
    }
    std::fs::write(root.join(GRAPH_MANIFEST_REL), &compiled.manifest).map_err(|error| {
        Fatal(format!(
            "assurance build: cannot write {GRAPH_MANIFEST_REL}: {error}"
        ))
    })?;
    println!(
        "assurance build: {GRAPH_MANIFEST_REL} sha256 {}",
        sha256_hex(&compiled.manifest)
    );
    Ok(compiled.runs.len())
}

pub fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    let mut text = String::with_capacity(64);
    for byte in digest {
        use std::fmt::Write as _;
        write!(&mut text, "{byte:02x}").expect("writing to String cannot fail");
    }
    text
}

fn sort_dedup(mut quads: Vec<Quad>) -> Vec<Quad> {
    quads.sort_by_key(|quad| {
        (
            quad.graph_name.to_string(),
            quad.subject.to_string(),
            quad.predicate.to_string(),
            quad.object.to_string(),
        )
    });
    quads.dedup();
    quads
}

fn serialize(quads: &[Quad]) -> Result<Vec<u8>, Fatal> {
    let mut serializer = oxttl::TriGSerializer::new();
    for (prefix, iri) in PREFIXES {
        serializer = serializer
            .with_prefix(prefix, iri)
            .map_err(|error| Fatal(format!("assurance build: invalid prefix {prefix}: {error}")))?;
    }
    let mut writer = serializer.for_writer(Vec::new());
    for quad in quads {
        writer
            .serialize_quad(quad.as_ref())
            .map_err(|error| Fatal(format!("assurance build: TriG write failed: {error}")))?;
    }
    writer
        .finish()
        .map_err(|error| Fatal(format!("assurance build: TriG finish failed: {error}")))
}

fn parse_back(bytes: &[u8]) -> Result<Vec<Quad>, Fatal> {
    oxttl::TriGParser::new()
        .for_slice(bytes)
        .map(|result| {
            result.map_err(|error| {
                Fatal(format!(
                    "assurance build: emitted TriG does not parse back: {error}"
                ))
            })
        })
        .collect()
}
