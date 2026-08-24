//! Bedrock substrate formation and independent checker-lock validation.

use crate::error::{Fatal, Violation};
use crate::model::SUBSTRATE_LOCK_REL;
use serde_json::Value;
use std::collections::BTreeSet;
use std::path::Path;

pub const CONTRACT: &str = "bedrock-expansion-mount/v1";
pub const CONTRACT_VERSION: u64 = 1;
pub const BASE_NAMESPACES: [&str; 6] = [
    "definition",
    "architecture",
    "risk",
    "plan",
    "record",
    "references",
];

#[derive(Clone, Debug)]
pub struct SubstrateLock {
    pub repository: String,
    pub git_ref: String,
    pub package: String,
    pub binary: String,
    pub runner_labels: BTreeSet<String>,
    pub mount_contract_versions: BTreeSet<u64>,
}

pub fn require_for_install(root: &Path, command: &str) -> Result<SubstrateLock, Fatal> {
    let situation = root.join("situation");
    if !situation.is_dir() {
        return Err(Fatal(format!(
            "assurance {command}: situation/ is absent; form the repository with a mount-capable bedrock release, then rerun `assurance {command}`"
        )));
    }
    for namespace in BASE_NAMESPACES {
        if !situation.join(namespace).is_dir() {
            return Err(Fatal(format!(
                "assurance {command}: situation/{namespace}/ is absent; finish bedrock formation before mounting assurance"
            )));
        }
    }
    load(root).map_err(|message| {
        Fatal(format!(
            "assurance {command}: {message}; update bedrock to a release supporting {CONTRACT}, then rerun `assurance {command}`"
        ))
    })
}

pub fn check(root: &Path, runner: Option<&str>, violations: &mut Vec<Violation>) {
    let rel = SUBSTRATE_LOCK_REL;
    for namespace in BASE_NAMESPACES {
        let path = format!("situation/{namespace}");
        if !root.join(&path).is_dir() {
            violations.push(Violation::new(
                "A001",
                path,
                1,
                "bedrock formation namespace is missing",
            ));
        }
    }
    match load(root) {
        Ok(lock) => {
            if !lock.mount_contract_versions.contains(&CONTRACT_VERSION) {
                violations.push(Violation::new(
                    "A001",
                    rel,
                    1,
                    format!(
                        "substrate lock does not support mount contract version {CONTRACT_VERSION}"
                    ),
                ));
            }
            if let Some(runner) = runner
                && !lock.runner_labels.contains(runner)
            {
                violations.push(Violation::new(
                    "A001",
                    "situation/assurance/assurance-init.yaml",
                    1,
                    format!(
                        "variables.witness_runner `{runner}` is not approved by {SUBSTRATE_LOCK_REL}"
                    ),
                ));
            }
        }
        Err(message) => violations.push(Violation::new("A001", rel, 1, message)),
    }
}

pub fn load(root: &Path) -> Result<SubstrateLock, String> {
    let path = root.join(SUBSTRATE_LOCK_REL);
    let metadata =
        std::fs::symlink_metadata(&path).map_err(|_| format!("{SUBSTRATE_LOCK_REL} is absent"))?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(format!(
            "{SUBSTRATE_LOCK_REL} must be a regular non-symlink file"
        ));
    }
    let source = std::fs::read_to_string(&path)
        .map_err(|error| format!("cannot read {SUBSTRATE_LOCK_REL}: {error}"))?;
    let value: Value = serde_norway::from_str(&source)
        .map_err(|error| format!("{SUBSTRATE_LOCK_REL} is not valid YAML: {error}"))?;
    if value.get("version").and_then(Value::as_u64) != Some(1) {
        return Err(format!("{SUBSTRATE_LOCK_REL} version must be 1"));
    }
    let checker = value
        .get("checker")
        .and_then(Value::as_object)
        .ok_or_else(|| format!("{SUBSTRATE_LOCK_REL} checker must be a mapping"))?;
    let repository = required_string(checker.get("repository"), "checker.repository")?;
    if !valid_repository(repository) {
        return Err(format!(
            "{SUBSTRATE_LOCK_REL} checker.repository must be owner/repository"
        ));
    }
    let git_ref = required_string(checker.get("ref"), "checker.ref")?;
    if !is_lower_hex(git_ref, 40) {
        return Err(format!(
            "{SUBSTRATE_LOCK_REL} checker.ref must be a full lowercase commit SHA"
        ));
    }
    let package = required_string(checker.get("package"), "checker.package")?;
    let binary = required_string(checker.get("binary"), "checker.binary")?;
    if !valid_token(package) || !valid_token(binary) {
        return Err(format!(
            "{SUBSTRATE_LOCK_REL} checker package and binary must be literal safe tokens"
        ));
    }
    let mount_contract_versions = integer_set(
        value.get("mount_contract_versions"),
        "mount_contract_versions",
    )?;
    if !mount_contract_versions.contains(&CONTRACT_VERSION) {
        return Err(format!("{SUBSTRATE_LOCK_REL} does not support {CONTRACT}"));
    }
    let runner_labels = string_set(value.get("runner_labels"), "runner_labels")?;
    if runner_labels.is_empty() || runner_labels.iter().any(|label| !valid_token(label)) {
        return Err(format!(
            "{SUBSTRATE_LOCK_REL} runner_labels must contain literal safe labels"
        ));
    }
    Ok(SubstrateLock {
        repository: repository.to_owned(),
        git_ref: git_ref.to_owned(),
        package: package.to_owned(),
        binary: binary.to_owned(),
        runner_labels,
        mount_contract_versions,
    })
}

fn required_string<'a>(value: Option<&'a Value>, field: &str) -> Result<&'a str, String> {
    value
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| format!("{SUBSTRATE_LOCK_REL} {field} must be a non-empty string"))
}

fn string_set(value: Option<&Value>, field: &str) -> Result<BTreeSet<String>, String> {
    value
        .and_then(Value::as_array)
        .ok_or_else(|| format!("{SUBSTRATE_LOCK_REL} {field} must be a list"))?
        .iter()
        .map(|item| {
            item.as_str()
                .map(str::to_owned)
                .ok_or_else(|| format!("{SUBSTRATE_LOCK_REL} {field} must contain strings"))
        })
        .collect()
}

fn integer_set(value: Option<&Value>, field: &str) -> Result<BTreeSet<u64>, String> {
    value
        .and_then(Value::as_array)
        .ok_or_else(|| format!("{SUBSTRATE_LOCK_REL} {field} must be a list"))?
        .iter()
        .map(|item| {
            item.as_u64()
                .ok_or_else(|| format!("{SUBSTRATE_LOCK_REL} {field} must contain integers"))
        })
        .collect()
}

fn valid_repository(value: &str) -> bool {
    value.split('/').count() == 2 && value.split('/').all(valid_token)
}

fn valid_token(value: &str) -> bool {
    !value.is_empty()
        && value.chars().all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '_' | '-' | '.')
        })
}

fn is_lower_hex(value: &str, length: usize) -> bool {
    value.len() == length
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}
