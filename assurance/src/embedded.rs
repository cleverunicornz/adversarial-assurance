//! Canonical files compiled into the binary for deterministic installation.
//!
//! The binary copy is authoritative for its schema version. `assurance check`
//! requires the materialized adoption copy to be byte-identical, so vocabulary
//! additions require a versioned pack and checker change.

pub const CONTEXT: &str = include_str!("../../schema/context.yamlld");
pub const VOCABULARY: &str = include_str!("../../schema/vocabulary.yaml");
pub const RECORD_SCHEMA: &str = include_str!("../../schema/records.schema.json");
pub const WORKFLOW: &str = include_str!("../../workflow/assurance.yml");

pub const INIT_TEMPLATE: &str = r#"# One adoption binding surface. Bootstrap replaces every REPLACE_WITH value.
version: 1
status: UNCONFIGURED
pack:
  repository: "REPLACE_WITH_PACK_REPOSITORY"
  ref: "REPLACE_WITH_FULL_COMMIT_SHA"
models:
  lead: "REPLACE_WITH_MODEL"
  worker: "REPLACE_WITH_MODEL"
  validator: "REPLACE_WITH_MODEL"
  reviewer: "REPLACE_WITH_MODEL"
harnesses:
  - "REPLACE_WITH_HARNESS"
runners:
  witness: "REPLACE_WITH_RUNNER_LABEL"
actors:
  final_validator: "REPLACE_WITH_ACTOR"
  reviewer: "REPLACE_WITH_ACTOR"
"#;

pub const EMPTY_REGISTRY: &str = r#"version: 1
vocabulary_version: 1
state: CONFIGURED_EMPTY
"#;

pub const BOOTSTRAP_INSTRUCTIONS: &str = r#"
BOOTSTRAP REQUIRED
1. Ask the human once: which model fills lead, worker, validator, and reviewer;
   which harness or harnesses launch them; which runner label hosts the CI
   witness; and who occupies final-validator and reviewer actor seats.
2. Resolve this pack's repository coordinate and pin its full 40-character
   commit. Do not ask the human to invent either value.
3. Write every answer into `.assurance/assurance-init.yaml`, then set
   `status: CONFIGURED`. This is the only adoption binding file.
4. Commit `.assurance/` and `.github/workflows/assurance.yml`. Configure the
   `assurance-required` job as a required merge check in repository rules.

Until step 3 is complete, `assurance check` fails with an A001 instruction.
CI is the sole witnessed checker seat. Manual local check/build is authoring
preflight only and is never evidence or a merge-authoritative fallback.
"#;
