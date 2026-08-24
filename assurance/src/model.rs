//! Canonical vocabulary and parsed record model.

use crate::embedded;
use crate::error::Fatal;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Component, Path, PathBuf};

pub const CONTEXT_IRI: &str = "https://adversarial-assurance.dev/context/v1";
pub const ROOT_IRI: &str = "https://adversarial-assurance.dev/";
pub const RECORD_PREFIX: &str = "https://adversarial-assurance.dev/record/";
pub const PATH_PREFIX: &str = "https://adversarial-assurance.dev/path/";
pub const GRAPH_PREFIX: &str = "https://adversarial-assurance.dev/graph/";

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum RecordKind {
    Promise,
    Witness,
    Oracle,
    Run,
}

impl RecordKind {
    pub const ALL: [Self; 4] = [Self::Promise, Self::Witness, Self::Oracle, Self::Run];

    pub fn name(self) -> &'static str {
        match self {
            Self::Promise => "Promise",
            Self::Witness => "Witness",
            Self::Oracle => "Oracle",
            Self::Run => "Run",
        }
    }

    pub fn iri(self) -> &'static str {
        match self {
            Self::Promise => "https://adversarial-assurance.dev/ontology/Promise",
            Self::Witness => "https://adversarial-assurance.dev/ontology/Witness",
            Self::Oracle => "https://adversarial-assurance.dev/ontology/Oracle",
            Self::Run => "https://adversarial-assurance.dev/ontology/Run",
        }
    }

    pub fn directory(self) -> Option<&'static str> {
        match self {
            Self::Promise => Some("promises"),
            Self::Witness => Some("witnesses"),
            Self::Oracle => Some("oracles"),
            Self::Run => None,
        }
    }

    pub fn id_segment(self) -> &'static str {
        match self {
            Self::Promise => "promise",
            Self::Witness => "witness",
            Self::Oracle => "oracle",
            Self::Run => "run",
        }
    }

    pub fn from_iri(iri: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|kind| kind.iri() == iri)
    }
}

#[derive(Clone, Debug)]
pub struct Document {
    pub rel: PathBuf,
    pub run_slug: String,
    pub source: String,
    pub value: Value,
    pub kind: Option<RecordKind>,
    pub id: Option<String>,
}

impl Document {
    pub fn rel_string(&self) -> String {
        portable(&self.rel)
    }

    pub fn sequence(&self) -> Option<u64> {
        self.value.get("sequence").and_then(Value::as_u64)
    }

    pub fn edge_values(&self, verb: &str) -> Vec<&str> {
        match self.value.get(verb) {
            Some(Value::String(value)) => vec![value.as_str()],
            Some(Value::Array(values)) => values.iter().filter_map(Value::as_str).collect(),
            _ => Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct VerbSpec {
    pub iri: String,
    pub domain: BTreeSet<String>,
    pub range: BTreeSet<String>,
}

#[derive(Clone, Debug)]
pub struct Vocabulary {
    pub version: u64,
    pub record_schema_version: u64,
    pub context: String,
    pub variable_syntax: String,
    pub variables: BTreeSet<String>,
    pub nouns: BTreeMap<String, String>,
    pub verbs: BTreeMap<String, VerbSpec>,
    pub source_fields: BTreeSet<String>,
    pub dispositions: BTreeSet<String>,
    pub lanes: BTreeSet<String>,
    pub admission_edges: BTreeSet<(String, String)>,
    pub independent_lanes: BTreeSet<String>,
    pub human_invoked_lanes: BTreeSet<String>,
}

impl Vocabulary {
    pub fn embedded() -> Result<Self, Fatal> {
        let value: Value = serde_norway::from_str(embedded::VOCABULARY)
            .map_err(|error| Fatal(format!("embedded vocabulary is invalid YAML: {error}")))?;
        let root = value
            .as_object()
            .ok_or_else(|| Fatal("embedded vocabulary must be a mapping".to_owned()))?;
        let version = integer(root.get("version"), "version")?;
        let record_schema_version =
            integer(root.get("record_schema_version"), "record_schema_version")?;
        let context = string(root.get("context"), "context")?.to_owned();
        let variable_syntax = string(root.get("variable_syntax"), "variable_syntax")?.to_owned();
        let variables = string_set(root.get("variables"), "variables")?;

        let mut nouns = BTreeMap::new();
        for (name, entry) in object(root.get("nouns"), "nouns")? {
            let iri = string(entry.get("iri"), "nouns.*.iri")?;
            nouns.insert(name.clone(), iri.to_owned());
        }

        let mut verbs = BTreeMap::new();
        for (name, entry) in object(root.get("verbs"), "verbs")? {
            let iri = string(entry.get("iri"), "verbs.*.iri")?.to_owned();
            let domain = string_set(entry.get("domain"), "verbs.*.domain")?;
            let range = string_set(entry.get("range"), "verbs.*.range")?;
            verbs.insert(name.clone(), VerbSpec { iri, domain, range });
        }

        let mut source_fields = BTreeSet::new();
        for (name, entry) in object(root.get("fields"), "fields")? {
            if !entry
                .get("generated")
                .and_then(Value::as_bool)
                .unwrap_or(false)
            {
                source_fields.insert(name.clone());
            }
        }

        let dispositions = string_set(root.get("dispositions"), "dispositions")?;
        let lanes = object(root.get("lanes"), "lanes")?
            .keys()
            .cloned()
            .collect();
        let admission_edges = pair_set(root.get("admission_edges"), "admission_edges")?;
        let independent_lanes = string_set(root.get("independent_lanes"), "independent_lanes")?;
        let human_invoked_lanes =
            string_set(root.get("human_invoked_lanes"), "human_invoked_lanes")?;

        Ok(Self {
            version,
            record_schema_version,
            context,
            variable_syntax,
            variables,
            nouns,
            verbs,
            source_fields,
            dispositions,
            lanes,
            admission_edges,
            independent_lanes,
            human_invoked_lanes,
        })
    }

    pub fn kind_for_iri(&self, iri: &str) -> Option<RecordKind> {
        self.nouns.iter().find_map(|(name, registered)| {
            (registered == iri).then(|| kind_for_name(name)).flatten()
        })
    }
}

fn kind_for_name(name: &str) -> Option<RecordKind> {
    RecordKind::ALL.into_iter().find(|kind| kind.name() == name)
}

fn integer(value: Option<&Value>, name: &str) -> Result<u64, Fatal> {
    value
        .and_then(Value::as_u64)
        .ok_or_else(|| Fatal(format!("embedded vocabulary `{name}` must be an integer")))
}

fn string<'a>(value: Option<&'a Value>, name: &str) -> Result<&'a str, Fatal> {
    value
        .and_then(Value::as_str)
        .ok_or_else(|| Fatal(format!("embedded vocabulary `{name}` must be a string")))
}

fn object<'a>(
    value: Option<&'a Value>,
    name: &str,
) -> Result<&'a serde_json::Map<String, Value>, Fatal> {
    value
        .and_then(Value::as_object)
        .ok_or_else(|| Fatal(format!("embedded vocabulary `{name}` must be a mapping")))
}

fn string_set(value: Option<&Value>, name: &str) -> Result<BTreeSet<String>, Fatal> {
    let values = value
        .and_then(Value::as_array)
        .ok_or_else(|| Fatal(format!("embedded vocabulary `{name}` must be a list")))?;
    values
        .iter()
        .map(|item| {
            item.as_str()
                .map(str::to_owned)
                .ok_or_else(|| Fatal(format!("embedded vocabulary `{name}` must contain strings")))
        })
        .collect()
}

fn pair_set(value: Option<&Value>, name: &str) -> Result<BTreeSet<(String, String)>, Fatal> {
    let values = value
        .and_then(Value::as_array)
        .ok_or_else(|| Fatal(format!("embedded vocabulary `{name}` must be a list")))?;
    values
        .iter()
        .map(|item| {
            let pair = item
                .as_array()
                .filter(|pair| pair.len() == 2)
                .ok_or_else(|| Fatal(format!("embedded vocabulary `{name}` needs string pairs")))?;
            Ok((
                string(pair.first(), name)?.to_owned(),
                string(pair.get(1), name)?.to_owned(),
            ))
        })
        .collect()
}

pub fn pointer_path(pointer: &str) -> Option<PathBuf> {
    let value = pointer.strip_prefix(PATH_PREFIX)?;
    if value.is_empty() || value.contains("//") || value.contains(char::is_whitespace) {
        return None;
    }
    let path = PathBuf::from(value);
    if path.components().any(|component| {
        matches!(
            component,
            Component::ParentDir | Component::RootDir | Component::Prefix(_)
        )
    }) {
        return None;
    }
    Some(path)
}

pub fn portable(path: &Path) -> String {
    path.components()
        .filter_map(|component| match component {
            Component::Normal(value) => Some(value.to_string_lossy().into_owned()),
            Component::CurDir => None,
            _ => Some(component.as_os_str().to_string_lossy().into_owned()),
        })
        .collect::<Vec<_>>()
        .join("/")
}
