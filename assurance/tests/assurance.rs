//! End-to-end contracts for init, every check rule, every record type, and
//! byte-stable TriG output.

use assurance::model::RecordKind;
use std::path::{Path, PathBuf};
use std::process::Command;

fn manifest() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn fixtures() -> PathBuf {
    manifest().join("tests/fixtures")
}

fn executable() -> &'static str {
    env!("CARGO_BIN_EXE_assurance")
}

struct Scratch(PathBuf);

impl Scratch {
    fn new(label: &str) -> Self {
        static SEQUENCE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let sequence = SEQUENCE.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "assurance-test-{label}-{}-{sequence}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        Self(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn copy(source: impl AsRef<Path>, destination: impl AsRef<Path>) {
    let destination = destination.as_ref();
    if let Some(parent) = destination.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    std::fs::copy(source, destination).unwrap();
}

fn copy_fixture(relative: &str, destination: impl AsRef<Path>) {
    copy(fixtures().join(relative), destination);
}

fn form_substrate(root: &Path) {
    for namespace in [
        "definition",
        "architecture",
        "risk",
        "plan",
        "record",
        "references",
    ] {
        std::fs::create_dir_all(root.join("situation").join(namespace)).unwrap();
    }
    std::fs::create_dir_all(root.join("seed")).unwrap();
    std::fs::write(
        root.join("seed/substrate-lock.yaml"),
        r#"version: 1
checker:
  repository: "bedrock-pack/bedrock"
  ref: "abcdef0123456789abcdef0123456789abcdef01"
  package: "bedrock-package"
  binary: "bedrock"
mount_contract_versions:
  - 1
runner_labels:
  - "linux-assurance"
"#,
    )
    .unwrap();
}

fn materialize_valid(label: &str) -> Scratch {
    let scratch = Scratch::new(label);
    form_substrate(scratch.path());
    assurance::init::run(scratch.path()).unwrap();

    copy_fixture(
        "valid/assurance-init.yaml",
        scratch
            .path()
            .join("situation/assurance/assurance-init.yaml"),
    );
    assurance::init::update(scratch.path()).unwrap();
    copy_fixture(
        "valid/registry.yaml",
        scratch.path().join("situation/assurance/registry.yaml"),
    );
    copy_fixture(
        "valid/records/run.yamlld",
        scratch
            .path()
            .join("situation/assurance/runs/demo/run.yamlld"),
    );
    copy_fixture(
        "valid/records/promise.yamlld",
        scratch
            .path()
            .join("situation/assurance/runs/demo/promises/behavior.yamlld"),
    );
    copy_fixture(
        "valid/records/witness.yamlld",
        scratch
            .path()
            .join("situation/assurance/runs/demo/witnesses/observation.yamlld"),
    );
    copy_fixture(
        "valid/records/oracle.yamlld",
        scratch
            .path()
            .join("situation/assurance/runs/demo/oracles/verdict.yamlld"),
    );
    copy_fixture(
        "valid/evidence/observed.txt",
        scratch
            .path()
            .join("situation/assurance/runs/demo/evidence/observed.txt"),
    );
    let report = assurance::check::inspect_for_build(scratch.path()).unwrap();
    assert!(
        report.violations.is_empty(),
        "valid source fixture failed before build: {:?}",
        report.violations
    );
    assurance::graph::write_all(scratch.path(), &report.documents).unwrap();
    scratch
}

fn run(arguments: &[&str], current_dir: &Path) -> (i32, String, String) {
    let output = Command::new(executable())
        .args(arguments)
        .current_dir(current_dir)
        .output()
        .unwrap();
    (
        output.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&output.stdout).into_owned(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
    )
}

fn assert_valid(root: &Path) {
    let report = assurance::check::inspect(root).unwrap();
    assert!(
        report.violations.is_empty(),
        "expected valid fixture, got:\n{}",
        report
            .violations
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n")
    );
}

fn rebuild(root: &Path) {
    let report = assurance::check::inspect_for_build(root).unwrap();
    assert!(
        report.violations.is_empty(),
        "source fixture failed before rebuild: {:?}",
        report.violations
    );
    assurance::graph::write_all(root, &report.documents).unwrap();
}

#[test]
fn valid_fixture_covers_every_record_type() {
    let scratch = materialize_valid("record-types-good");
    let report = assurance::check::inspect(scratch.path()).unwrap();
    assert!(report.violations.is_empty(), "{:?}", report.violations);
    for kind in RecordKind::ALL {
        assert!(
            report
                .documents
                .iter()
                .any(|document| document.kind == Some(kind)),
            "missing positive {} fixture",
            kind.name()
        );
    }
    let promise = scratch
        .path()
        .join("situation/assurance/runs/demo/promises/behavior.yamlld");
    let source = std::fs::read_to_string(&promise).unwrap();
    std::fs::write(
        &promise,
        source.replace(
            "The fixture behavior",
            "{{lead_model}} observes that the fixture behavior",
        ),
    )
    .unwrap();
    rebuild(scratch.path());
    assert_valid(scratch.path());
}

#[test]
fn every_record_type_has_a_negative_shape_fixture() {
    let cases = [
        (
            "run",
            "invalid/record-types/run-missing-lane.yamlld",
            "situation/assurance/runs/demo/run.yamlld",
        ),
        (
            "promise",
            "invalid/record-types/promise-missing-envelope.yamlld",
            "situation/assurance/runs/demo/promises/behavior.yamlld",
        ),
        (
            "witness",
            "invalid/record-types/witness-missing-digest.yamlld",
            "situation/assurance/runs/demo/witnesses/observation.yamlld",
        ),
        (
            "oracle",
            "invalid/record-types/oracle-missing-disposition.yamlld",
            "situation/assurance/runs/demo/oracles/verdict.yamlld",
        ),
    ];
    for (name, fixture, destination) in cases {
        let scratch = materialize_valid(&format!("record-type-{name}-bad"));
        copy_fixture(fixture, scratch.path().join(destination));
        let report = assurance::check::inspect(scratch.path()).unwrap();
        assert!(
            report.violations.iter().any(|item| item.rule == "A007"),
            "negative {name} fixture must fail A007: {:?}",
            report.violations
        );
    }
}

#[test]
fn every_rule_has_positive_and_negative_polarity() {
    for rule in [
        "A001", "A002", "A003", "A004", "A005", "A006", "A007", "A008", "A009", "A010",
    ] {
        let scratch = materialize_valid(&format!("{rule}-polarity"));
        assert_valid(scratch.path());
        install_rule_failure(rule, scratch.path());

        let (code, stdout, stderr) = run(&["check", scratch.path().to_str().unwrap()], &manifest());
        assert_eq!(code, 1, "{rule} must exit 1\n{stdout}\n{stderr}");
        assert!(
            stderr
                .lines()
                .any(|line| line.starts_with(&format!("{rule} "))),
            "{rule} diagnostic missing:\n{stderr}"
        );
        if rule == "A001" {
            assert!(
                stderr.contains("required variable"),
                "A001 must name the missing or placeholder variable:\n{stderr}"
            );
        }
        for line in stderr.lines() {
            let mut pieces = line.splitn(3, ' ');
            let diagnostic_rule = pieces.next().unwrap_or_default();
            let location = pieces.next().unwrap_or_default();
            let message = pieces.next().unwrap_or_default();
            assert!(diagnostic_rule.starts_with('A'), "bad rule: {line}");
            assert!(location.rsplit_once(':').is_some(), "bad path:line: {line}");
            assert!(!message.is_empty(), "missing fix instruction: {line}");
        }
    }

    let scratch = materialize_valid("A001-undeclared-record-variable");
    let promise = scratch
        .path()
        .join("situation/assurance/runs/demo/promises/behavior.yamlld");
    let source = std::fs::read_to_string(&promise).unwrap();
    std::fs::write(
        &promise,
        source.replace(
            "The fixture behavior",
            "{{undeclared_model}} observes that the fixture behavior",
        ),
    )
    .unwrap();
    let report = assurance::check::inspect(scratch.path()).unwrap();
    assert!(
        report.violations.iter().any(|item| {
            item.rule == "A001"
                && item
                    .message
                    .contains("is not in the canonical variable set")
        }),
        "undeclared record variable must fail A001: {:?}",
        report.violations
    );
}

fn install_rule_failure(rule: &str, root: &Path) {
    match rule {
        "A001" => copy_fixture(
            "invalid/rules/A001-variables.yaml",
            root.join("situation/assurance/assurance-init.yaml"),
        ),
        "A002" => copy_fixture(
            "invalid/rules/A002-registry.yaml",
            root.join("situation/assurance/registry.yaml"),
        ),
        "A003" => copy_fixture(
            "invalid/rules/A003-unexpected.txt",
            root.join("situation/assurance/runs/demo/unexpected.txt"),
        ),
        "A004" => replace_promise(root, "invalid/rules/A004-anchor.yamlld"),
        "A005" => replace_promise(root, "invalid/rules/A005-plain-body.yamlld"),
        "A006" => replace_promise(root, "invalid/rules/A006-invented-verb.yamlld"),
        "A007" => replace_promise(root, "invalid/rules/A007-shape.yamlld"),
        "A008" => replace_promise(root, "invalid/rules/A008-dangling.yamlld"),
        "A009" => copy_fixture(
            "invalid/rules/A009-bad-digest.yamlld",
            root.join("situation/assurance/runs/demo/witnesses/observation.yamlld"),
        ),
        "A010" => copy_fixture(
            "invalid/rules/A010-blocked-successor.yamlld",
            root.join("situation/assurance/runs/demo/oracles/verdict.yamlld"),
        ),
        _ => unreachable!(),
    }
}

fn replace_promise(root: &Path, fixture: &str) {
    copy_fixture(
        fixture,
        root.join("situation/assurance/runs/demo/promises/behavior.yamlld"),
    );
}

#[test]
fn broken_witness_prints_exact_fix_instruction() {
    let scratch = materialize_valid("exact-diagnostic");
    install_rule_failure("A009", scratch.path());
    let (code, _, stderr) = run(&["check", scratch.path().to_str().unwrap()], &manifest());
    assert_eq!(code, 1);
    assert!(
        stderr.lines().any(|line| {
            line.starts_with(
                "A009 situation/assurance/runs/demo/witnesses/observation.yamlld:10 witness digest mismatch",
            ) && line.contains("evidence/observed.txt")
        }),
        "unexpected diagnostic:\n{stderr}"
    );
}

#[test]
fn build_is_byte_stable_and_prose_is_pointer_only() {
    let scratch = materialize_valid("determinism");
    let target = scratch.path().to_str().unwrap();
    let (check_code, check_stdout, check_stderr) = run(&["check", target], &manifest());
    assert_eq!(
        check_code, 0,
        "check failed\n{check_stdout}\n{check_stderr}"
    );

    let (first_code, first_stdout, first_stderr) = run(&["build", target], &manifest());
    assert_eq!(
        first_code, 0,
        "first build failed\n{first_stdout}\n{first_stderr}"
    );
    let graph_path = scratch
        .path()
        .join("situation/assurance/runs/demo/graph.trig");
    let first = std::fs::read(&graph_path).unwrap();

    let (second_code, second_stdout, second_stderr) = run(&["build", target], &manifest());
    assert_eq!(
        second_code, 0,
        "second build failed\n{second_stdout}\n{second_stderr}"
    );
    let second = std::fs::read(&graph_path).unwrap();
    assert_eq!(first, second, "same input must produce identical bytes");
    assert_eq!(
        first_stdout, second_stdout,
        "graph digest log must be stable"
    );

    let graph = String::from_utf8(first).unwrap();
    assert!(
        graph.contains("content-digest"),
        "digest pointer missing: {graph}"
    );
    assert!(
        graph.contains("promises\\/behavior.yamlld"),
        "source pointer missing: {graph}"
    );
    assert!(graph.contains("resolves-to"), "path edge missing: {graph}");
    assert!(
        !graph.contains("PROSE_MUST_NOT_BE_IN_GRAPH"),
        "body prose must never be inlined: {graph}"
    );
    assert!(graph.contains("@prefix aa: <urn:assurance:>"), "{graph}");
    assert!(
        !graph.contains("urn:bedrock:"),
        "expansion graph leaked bedrock-owned IRI: {graph}"
    );
    let graph_manifest = std::fs::read_to_string(
        scratch
            .path()
            .join("situation/assurance/graph-manifest.yaml"),
    )
    .unwrap();
    assert!(
        graph_manifest.contains("path: \"situation/assurance/runs/demo/graph.trig\"")
            && graph_manifest.contains(&assurance::graph::sha256_hex(&second)),
        "graph manifest does not bind the run graph: {graph_manifest}"
    );
}

#[test]
fn build_refuses_a_failed_check() {
    let scratch = materialize_valid("build-refusal");
    let graph = scratch
        .path()
        .join("situation/assurance/runs/demo/graph.trig");
    let before = std::fs::read(&graph).unwrap();
    install_rule_failure("A009", scratch.path());
    let (code, _, stderr) = run(&["build", scratch.path().to_str().unwrap()], &manifest());
    assert_eq!(code, 1);
    assert!(stderr.contains("A009 "), "rule output missing: {stderr}");
    assert!(
        stderr.contains("assurance build aborted: check failed"),
        "refusal missing: {stderr}"
    );
    assert_eq!(
        std::fs::read(&graph).unwrap(),
        before,
        "failed build must not change a committed graph"
    );
}

#[test]
fn init_refuses_unformed_and_unsupported_repositories() {
    let unformed = Scratch::new("init-unformed");
    let target = unformed.path().to_str().unwrap();
    let (code, _, stderr) = run(&["init", target], &manifest());
    assert_eq!(code, 1);
    assert!(
        stderr.contains(
            "situation/ is absent; form the repository with a mount-capable bedrock release"
        ),
        "{stderr}"
    );
    assert!(!unformed.path().join("situation/assurance").exists());

    let unsupported = Scratch::new("init-unsupported");
    for namespace in [
        "definition",
        "architecture",
        "risk",
        "plan",
        "record",
        "references",
    ] {
        std::fs::create_dir_all(unsupported.path().join("situation").join(namespace)).unwrap();
    }
    let target = unsupported.path().to_str().unwrap();
    let (code, _, stderr) = run(&["init", target], &manifest());
    assert_eq!(code, 1);
    assert!(
        stderr.contains("seed/substrate-lock.yaml is absent"),
        "{stderr}"
    );
    assert!(!unsupported.path().join("situation/assurance").exists());
}

#[test]
fn init_installs_mount_and_prints_registration_proposal() {
    let scratch = Scratch::new("init-contract");
    form_substrate(scratch.path());
    let target = scratch.path().to_str().unwrap();
    let (code, stdout, stderr) = run(&["init", target], &manifest());
    assert_eq!(code, 0, "{stdout}\n{stderr}");
    assert!(stdout.contains("BOOTSTRAP REQUIRED"), "{stdout}");
    assert!(
        stdout.contains("path: situation/architecture/mount-assurance.yamlld")
            && stdout.contains("\"@type\": \"urn:bedrock:ontology/ExpansionMount\"")
            && stdout.contains("graph_manifest_sha256:"),
        "registration proposal incomplete: {stdout}"
    );
    assert!(
        scratch
            .path()
            .join("situation/assurance/schema/vocabulary.yaml")
            .exists()
    );
    assert!(
        scratch
            .path()
            .join("situation/assurance/workflow/assurance.yml")
            .exists()
    );
    assert!(
        !scratch
            .path()
            .join(".github/workflows/assurance.yml")
            .exists()
    );
    assert!(
        !scratch
            .path()
            .join("situation/architecture/mount-assurance.yamlld")
            .exists(),
        "assurance init must print, never write, the bedrock registration"
    );
    let graph_manifest = std::fs::read_to_string(
        scratch
            .path()
            .join("situation/assurance/graph-manifest.yaml"),
    )
    .unwrap();
    assert_eq!(graph_manifest, "version: 1\ngraphs: []\n");
    let registry =
        std::fs::read_to_string(scratch.path().join("situation/assurance/registry.yaml")).unwrap();
    assert!(registry.contains("vocabulary_version: 3"), "{registry}");
    let init = std::fs::read_to_string(
        scratch
            .path()
            .join("situation/assurance/assurance-init.yaml"),
    )
    .unwrap();
    assert!(init.contains("contract: \"bedrock-expansion-mount/v1\""));
    assert!(init.contains("minimum_contract_version: 1"));

    let (check_code, _, check_stderr) = run(&["check", target], &manifest());
    assert_eq!(check_code, 1);
    assert!(check_stderr.contains("A001 "), "{check_stderr}");
    assert!(
        check_stderr.contains("status is not CONFIGURED"),
        "{check_stderr}"
    );
}

#[test]
fn changed_records_with_stale_graph_and_manifest_fail_until_build() {
    let scratch = materialize_valid("manifest-drift");
    let promise = scratch
        .path()
        .join("situation/assurance/runs/demo/promises/behavior.yamlld");
    let source = std::fs::read_to_string(&promise).unwrap();
    std::fs::write(
        &promise,
        source.replace("promised boundary", "revised promised boundary"),
    )
    .unwrap();

    let (code, _, stderr) = run(&["check", scratch.path().to_str().unwrap()], &manifest());
    assert_eq!(code, 1);
    assert!(
        stderr.contains("A002 situation/assurance/runs/demo/graph.trig:1")
            && stderr.contains("A002 situation/assurance/graph-manifest.yaml:1"),
        "stale generated outputs not named: {stderr}"
    );

    let (build_code, _, build_stderr) =
        run(&["build", scratch.path().to_str().unwrap()], &manifest());
    assert_eq!(build_code, 0, "{build_stderr}");
    assert_valid(scratch.path());
}

#[test]
fn every_committed_evidence_file_requires_same_run_witness() {
    let scratch = materialize_valid("inverse-evidence");
    let stray = scratch
        .path()
        .join("situation/assurance/runs/demo/evidence/unwitnessed.log");
    std::fs::write(&stray, "unwitnessed\n").unwrap();
    let report = assurance::check::inspect(scratch.path()).unwrap();
    assert!(
        report.violations.iter().any(|violation| {
            violation.rule == "A009"
                && violation.path.ends_with("evidence/unwitnessed.log")
                && violation.message.contains("not the resolves_to target")
        }),
        "inverse evidence coverage did not fail: {:?}",
        report.violations
    );
    std::fs::remove_file(&stray).unwrap();
    let record = scratch
        .path()
        .join("situation/assurance/runs/demo/evidence/stray.yamlld");
    std::fs::write(&record, "not: a record home\n").unwrap();
    let report = assurance::check::inspect(scratch.path()).unwrap();
    assert!(
        report.violations.iter().any(|violation| {
            violation.rule == "A003"
                && violation.path.ends_with("evidence/stray.yamlld")
                && violation.message.contains("may not contain .yamlld")
        }),
        "stray evidence record did not fail A003: {:?}",
        report.violations
    );
    std::fs::remove_file(record).unwrap();

    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        let link = scratch
            .path()
            .join("situation/assurance/runs/demo/evidence/observed-link.txt");
        symlink("observed.txt", &link).unwrap();
        let report = assurance::check::inspect(scratch.path()).unwrap();
        assert!(
            report.violations.iter().any(|violation| {
                violation.rule == "A003"
                    && violation.path.ends_with("evidence/observed-link.txt")
                    && violation.message.contains("may not be symlinks")
            }),
            "evidence symlink did not fail A003: {:?}",
            report.violations
        );
    }
}

#[test]
fn external_artifact_manifest_is_committed_and_witnessed() {
    let scratch = materialize_valid("external-manifest");
    let manifest_path = scratch
        .path()
        .join("situation/assurance/runs/demo/evidence/external/payload.external-artifact.yaml");
    std::fs::create_dir_all(manifest_path.parent().unwrap()).unwrap();
    let external = "version: 1\n\
external_uri: \"s3://example-bucket/object?versionId=immutable\"\n\
sha256: \"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\"\n\
size: 1048576\n\
provenance: \"Produced by the docketed proof executor at the pinned target.\"\n";
    std::fs::write(&manifest_path, external).unwrap();
    let digest = assurance::graph::sha256_hex(external.as_bytes());
    let witness = format!(
        r#""@context": "urn:assurance:context/v1"
"@id": "urn:assurance:record/demo/witness/external"
"@type": "urn:assurance:ontology/Witness"
schema_version: 1
sequence: 4
label: "External artifact manifest"
body: |
  The committed manifest represents an external payload; the checker hashes the manifest, not the remote object.
resolves_to: "urn:assurance:path/situation/assurance/runs/demo/evidence/external/payload.external-artifact.yaml"
artifact_sha256: "{digest}"
producer: "fixture-producer"
part_of: "urn:assurance:record/demo/run"
"#
    );
    std::fs::write(
        scratch
            .path()
            .join("situation/assurance/runs/demo/witnesses/external.yamlld"),
        witness,
    )
    .unwrap();
    rebuild(scratch.path());
    assert_valid(scratch.path());
}

#[test]
fn update_refreshes_only_mount_owned_canonical_files() {
    let scratch = materialize_valid("update");
    let init_path = scratch
        .path()
        .join("situation/assurance/assurance-init.yaml");
    let record_path = scratch
        .path()
        .join("situation/assurance/runs/demo/promises/behavior.yamlld");
    let evidence_path = scratch
        .path()
        .join("situation/assurance/runs/demo/evidence/observed.txt");
    let graph_path = scratch
        .path()
        .join("situation/assurance/runs/demo/graph.trig");
    let init_before = std::fs::read(&init_path).unwrap();
    let record_before = std::fs::read(&record_path).unwrap();
    let evidence_before = std::fs::read(&evidence_path).unwrap();
    let graph_before = std::fs::read(&graph_path).unwrap();

    std::fs::write(
        scratch
            .path()
            .join("situation/assurance/schema/context.yamlld"),
        "tampered\n",
    )
    .unwrap();
    std::fs::write(
        scratch
            .path()
            .join("situation/assurance/workflow/assurance.yml"),
        "tampered\n",
    )
    .unwrap();
    std::fs::write(
        scratch
            .path()
            .join("situation/assurance/graph-manifest.yaml"),
        "tampered\n",
    )
    .unwrap();

    let (code, stdout, stderr) = run(&["update", scratch.path().to_str().unwrap()], &manifest());
    assert_eq!(code, 0, "{stdout}\n{stderr}");
    assert!(
        stdout.contains("replacement registration proposal"),
        "{stdout}"
    );
    assert_eq!(std::fs::read(init_path).unwrap(), init_before);
    let workflow = std::fs::read_to_string(
        scratch
            .path()
            .join("situation/assurance/workflow/assurance.yml"),
    )
    .unwrap();
    assert!(workflow.contains("runs-on: linux-assurance"), "{workflow}");
    assert!(!workflow.contains("__ASSURANCE_WITNESS_RUNNER__"));
    assert_eq!(std::fs::read(record_path).unwrap(), record_before);
    assert_eq!(std::fs::read(graph_path).unwrap(), graph_before);
    assert_eq!(std::fs::read(evidence_path).unwrap(), evidence_before);
    assert_valid(scratch.path());
}

#[test]
fn substrate_block_is_closed_and_required_by_a001() {
    let scratch = materialize_valid("substrate-a001");
    let init_path = scratch
        .path()
        .join("situation/assurance/assurance-init.yaml");
    let source = std::fs::read_to_string(&init_path).unwrap();
    let without_substrate = source.replace(
        "substrate:\n  contract: \"bedrock-expansion-mount/v1\"\n  minimum_contract_version: 1\n",
        "",
    );
    std::fs::write(&init_path, without_substrate).unwrap();
    let report = assurance::check::inspect(scratch.path()).unwrap();
    assert!(
        report.violations.iter().any(|violation| {
            violation.rule == "A001"
                && (violation.message.contains("substrate.contract")
                    || violation.message.contains("minimum_contract_version"))
        }),
        "missing substrate block did not fail A001: {:?}",
        report.violations
    );

    std::fs::write(&init_path, source).unwrap();
    let lock_path = scratch.path().join("seed/substrate-lock.yaml");
    let lock = std::fs::read_to_string(&lock_path).unwrap();
    std::fs::write(
        &lock_path,
        lock.replace("\"linux-assurance\"", "\"different-runner\""),
    )
    .unwrap();
    let report = assurance::check::inspect(scratch.path()).unwrap();
    assert!(
        report.violations.iter().any(|violation| {
            violation.rule == "A001"
                && violation
                    .message
                    .contains("is not approved by seed/substrate-lock.yaml")
        }),
        "unapproved runner did not fail A001: {:?}",
        report.violations
    );
}
