//! Bedrock substrate formation and shipped checker-lock validation.

use crate::error::{Fatal, Violation};
use crate::model::SUBSTRATE_LOCK_REL;
use serde_json::Value;
use std::collections::BTreeSet;
use std::path::Path;

pub const CONTRACT: &str = "bedrock-expansion-mount/v1";
pub const CONTRACT_VERSION: u64 = 1;
pub const WITNESS_RUNNER: &str = "org-ci-linux-x64";
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
    pub package: String,
    pub checker_ref: String,
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
            "assurance {command}: {message}; run `bedrock update` with bedrock 0.7.0 or newer, then rerun `assurance {command}`"
        ))
    })
}

pub fn check(root: &Path, runner: Option<&str>, violations: &mut Vec<Violation>) {
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
        Ok(_) => {
            if let Some(runner) = runner
                && runner != WITNESS_RUNNER
            {
                violations.push(Violation::new(
                    "A001",
                    "situation/assurance/assurance-init.yaml",
                    1,
                    format!(
                        "variables.witness_runner `{runner}` is not substrate-approved; use `{WITNESS_RUNNER}`"
                    ),
                ));
            }
        }
        Err(message) => violations.push(Violation::new("A001", SUBSTRATE_LOCK_REL, 1, message)),
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
    let value: Value = serde_json::from_str(&source)
        .map_err(|error| format!("{SUBSTRATE_LOCK_REL} is not valid JSON: {error}"))?;
    if value.get("contract").and_then(Value::as_str) != Some(CONTRACT) {
        return Err(format!(
            "{SUBSTRATE_LOCK_REL} contract must be `{CONTRACT}`"
        ));
    }
    let checker = value
        .get("checker")
        .and_then(Value::as_object)
        .ok_or_else(|| format!("{SUBSTRATE_LOCK_REL} checker must be an object"))?;
    let package = required_string(checker.get("package"), "checker.package")?;
    let checker_ref = required_string(checker.get("ref"), "checker.ref")?;
    if !valid_token(package) || !valid_token(checker_ref) {
        return Err(format!(
            "{SUBSTRATE_LOCK_REL} checker package/ref must be literal safe tokens"
        ));
    }
    let mount_contract_versions = integer_set(
        value.get("supported_mount_contract_versions"),
        "supported_mount_contract_versions",
    )?;
    if !mount_contract_versions.contains(&CONTRACT_VERSION) {
        return Err(format!(
            "{SUBSTRATE_LOCK_REL} does not support mount contract version {CONTRACT_VERSION}"
        ));
    }
    Ok(SubstrateLock {
        package: package.to_owned(),
        checker_ref: checker_ref.to_owned(),
        mount_contract_versions,
    })
}

fn required_string<'a>(value: Option<&'a Value>, field: &str) -> Result<&'a str, String> {
    value
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| format!("{SUBSTRATE_LOCK_REL} {field} must be a non-empty string"))
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

fn valid_token(value: &str) -> bool {
    !value.is_empty()
        && value.chars().all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '_' | '-' | '.')
        })
}
