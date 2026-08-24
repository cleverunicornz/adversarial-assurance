//! Canonical files compiled into the binary for deterministic installation.
//!
//! The binary copy is authoritative for its schema version. `assurance check`
//! requires the materialized adoption copy to be byte-identical, so vocabulary
//! additions require a versioned pack and checker change.

pub const CONTEXT: &str = include_str!("../../schema/context.yamlld");
pub const VOCABULARY: &str = include_str!("../../schema/vocabulary.yaml");
pub const RECORD_SCHEMA: &str = include_str!("../../schema/records.schema.json");
pub const WORKFLOW: &str = include_str!("../../workflow/assurance.yml");

pub const WORKFLOW_RUNNER_TOKEN: &str = "__ASSURANCE_WITNESS_RUNNER__";

pub fn render_workflow(witness_runner: Option<&str>) -> String {
    WORKFLOW.replace(
        WORKFLOW_RUNNER_TOKEN,
        witness_runner.unwrap_or("REPLACE_WITH_WITNESS_RUNNER"),
    )
}

pub const INIT_TEMPLATE: &str = r#"# One mount binding surface. Bootstrap replaces every REPLACE_WITH value.
version: 1
status: UNCONFIGURED
substrate:
  contract: "bedrock-expansion-mount/v1"
  minimum_contract_version: 1
pack:
  repository: "REPLACE_WITH_PACK_REPOSITORY"
  ref: "REPLACE_WITH_FULL_COMMIT_SHA"
variables:
  lead_model: "REPLACE_WITH_LEAD_MODEL"
  executor_model: "REPLACE_WITH_EXECUTOR_MODEL"
  validator_model: "REPLACE_WITH_VALIDATOR_MODEL"
  harness: "REPLACE_WITH_HARNESS"
  witness_runner: "REPLACE_WITH_WITNESS_RUNNER"
  reviewer_seat: "REPLACE_WITH_REVIEWER_SEAT"
  final_validator_seat: "REPLACE_WITH_FINAL_VALIDATOR_SEAT"
"#;

pub const EMPTY_REGISTRY: &str = r#"version: 1
vocabulary_version: 3
state: CONFIGURED_EMPTY
"#;

pub const EMPTY_GRAPH_MANIFEST: &str = "version: 1\ngraphs: []\n";

pub const BOOTSTRAP_INSTRUCTIONS: &str = r#"
BOOTSTRAP REQUIRED
1. Ask the human once for exactly these variables: lead model, executor model,
   independent-validator model, harness, CI witness runner, reviewer seat, and
   final-validator seat.
2. Resolve this pack's repository coordinate and pin its full 40-character
   commit. Do not ask the human to invent either value.
3. Populate every key under `variables:` in
   `situation/assurance/assurance-init.yaml`, replace every placeholder, then
   set `status: CONFIGURED`.
4. Run `assurance update`. It renders the mount-owned workflow template for
   the configured substrate-approved runner and refreshes canonical mount files.
5. Copy `situation/assurance/workflow/assurance.yml` to the consumer workflow
   location and require its single `assurance-witness` job before merge.
6. Submit the printed ExpansionMount registration proposal through the normal
   bedrock authoring loop. Assurance never writes `situation/architecture/`.
7. The first campaign commit contains `run.yamlld` and one complete
   Promise/Witness/Oracle triad; partial first state is checker-red.

Until configuration and registration are complete, the two-checker witness
fails closed. CI runs bedrock check, assurance check, assurance build, and both
generated-output gates on the configured witness runner. Manual local
check/build remains authoring preflight and is never evidence.
"#;
