//! Offline JSON-LD expansion and deterministic per-run TriG projection.
//!
//! Provenance: the YAML-to-JSON-LD, sorted-quad, oxttl pipeline follows
//! bedrock's published compiler shape; prose projection and run semantics are
//! assurance-specific and implemented here from scratch.

use crate::embedded;
use crate::error::{Fatal, Violation};
use crate::model::{Document, GRAPH_PREFIX, PATH_PREFIX, ROOT_IRI};
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

pub fn write_all(root: &Path, documents: &[Document]) -> Result<usize, Fatal> {
    let mut grouped: BTreeMap<String, Vec<u8>> = BTreeMap::new();
    for document in documents {
        grouped.entry(document.run_slug.clone()).or_default();
    }
    for (run_slug, bytes) in &mut grouped {
        *bytes = compile_run(run_slug, documents)?;
    }

    for (run_slug, bytes) in &grouped {
        let path = root
            .join(".assurance/runs")
            .join(run_slug)
            .join("graph.trig");
        std::fs::write(&path, bytes).map_err(|error| {
            Fatal(format!(
                "assurance build: cannot write {}: {error}",
                path.display()
            ))
        })?;
        println!(
            "assurance build: .assurance/runs/{run_slug}/graph.trig sha256 {}",
            sha256_hex(bytes)
        );
    }
    Ok(grouped.len())
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
