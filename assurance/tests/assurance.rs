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

fn materialize_valid(label: &str) -> Scratch {
    let scratch = Scratch::new(label);
    assurance::init::run(scratch.path()).unwrap();

    copy_fixture(
        "valid/assurance-init.yaml",
        scratch.path().join(".assurance/assurance-init.yaml"),
    );
    copy_fixture(
        "valid/registry.yaml",
        scratch.path().join(".assurance/registry.yaml"),
    );
    copy_fixture(
        "valid/records/run.yamlld",
        scratch.path().join(".assurance/runs/demo/run.yamlld"),
    );
    copy_fixture(
        "valid/records/promise.yamlld",
        scratch
            .path()
            .join(".assurance/runs/demo/promises/behavior.yamlld"),
    );
    copy_fixture(
        "valid/records/witness.yamlld",
        scratch
            .path()
            .join(".assurance/runs/demo/witnesses/observation.yamlld"),
    );
    copy_fixture(
        "valid/records/oracle.yamlld",
        scratch
            .path()
            .join(".assurance/runs/demo/oracles/verdict.yamlld"),
    );
    for name in ["observed.txt", "promise.txt", "oracle.txt"] {
        copy_fixture(
            &format!("valid/evidence/{name}"),
            scratch.path().join("evidence").join(name),
        );
    }
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
        .join(".assurance/runs/demo/promises/behavior.yamlld");
    let source = std::fs::read_to_string(&promise).unwrap();
    std::fs::write(
        &promise,
        source.replace(
            "The fixture behavior",
            "{{lead_model}} observes that the fixture behavior",
        ),
    )
    .unwrap();
    assert_valid(scratch.path());
}

#[test]
fn every_record_type_has_a_negative_shape_fixture() {
    let cases = [
        (
            "run",
            "invalid/record-types/run-missing-lane.yamlld",
            ".assurance/runs/demo/run.yamlld",
        ),
        (
            "promise",
            "invalid/record-types/promise-missing-envelope.yamlld",
            ".assurance/runs/demo/promises/behavior.yamlld",
        ),
        (
            "witness",
            "invalid/record-types/witness-missing-digest.yamlld",
            ".assurance/runs/demo/witnesses/observation.yamlld",
        ),
        (
            "oracle",
            "invalid/record-types/oracle-missing-disposition.yamlld",
            ".assurance/runs/demo/oracles/verdict.yamlld",
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
        .join(".assurance/runs/demo/promises/behavior.yamlld");
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
            root.join(".assurance/assurance-init.yaml"),
        ),
        "A002" => copy_fixture(
            "invalid/rules/A002-registry.yaml",
            root.join(".assurance/registry.yaml"),
        ),
        "A003" => copy_fixture(
            "invalid/rules/A003-unexpected.txt",
            root.join(".assurance/runs/demo/unexpected.txt"),
        ),
        "A004" => replace_promise(root, "invalid/rules/A004-anchor.yamlld"),
        "A005" => replace_promise(root, "invalid/rules/A005-plain-body.yamlld"),
        "A006" => replace_promise(root, "invalid/rules/A006-invented-verb.yamlld"),
        "A007" => replace_promise(root, "invalid/rules/A007-shape.yamlld"),
        "A008" => replace_promise(root, "invalid/rules/A008-dangling.yamlld"),
        "A009" => copy_fixture(
            "invalid/rules/A009-bad-digest.yamlld",
            root.join(".assurance/runs/demo/witnesses/observation.yamlld"),
        ),
        "A010" => copy_fixture(
            "invalid/rules/A010-blocked-successor.yamlld",
            root.join(".assurance/runs/demo/oracles/verdict.yamlld"),
        ),
        _ => unreachable!(),
    }
}

fn replace_promise(root: &Path, fixture: &str) {
    copy_fixture(
        fixture,
        root.join(".assurance/runs/demo/promises/behavior.yamlld"),
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
                "A009 .assurance/runs/demo/witnesses/observation.yamlld:10 witness digest mismatch",
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
    let graph_path = scratch.path().join(".assurance/runs/demo/graph.trig");
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
}

#[test]
fn build_refuses_a_failed_check() {
    let scratch = materialize_valid("build-refusal");
    install_rule_failure("A009", scratch.path());
    let (code, _, stderr) = run(&["build", scratch.path().to_str().unwrap()], &manifest());
    assert_eq!(code, 1);
    assert!(stderr.contains("A009 "), "rule output missing: {stderr}");
    assert!(
        stderr.contains("assurance build aborted: check failed"),
        "refusal missing: {stderr}"
    );
    assert!(
        !scratch
            .path()
            .join(".assurance/runs/demo/graph.trig")
            .exists(),
        "failed build must not emit a graph"
    );
}

#[test]
fn init_installs_unconfigured_actor_bootstrap() {
    let scratch = Scratch::new("init-contract");
    let target = scratch.path().to_str().unwrap();
    let (code, stdout, stderr) = run(&["init", target], &manifest());
    assert_eq!(code, 0, "{stdout}\n{stderr}");
    assert!(stdout.contains("BOOTSTRAP REQUIRED"), "{stdout}");
    assert!(stdout.contains("only adoption binding file"), "{stdout}");
    assert!(
        scratch
            .path()
            .join(".assurance/schema/vocabulary.yaml")
            .exists()
    );
    assert!(
        scratch
            .path()
            .join(".github/workflows/assurance.yml")
            .exists()
    );
    let registry =
        std::fs::read_to_string(scratch.path().join(".assurance/registry.yaml")).unwrap();
    assert!(registry.contains("CONFIGURED_EMPTY"), "{registry}");
    let init =
        std::fs::read_to_string(scratch.path().join(".assurance/assurance-init.yaml")).unwrap();
    for variable in [
        "lead_model",
        "executor_model",
        "validator_model",
        "harness",
        "witness_runner",
        "reviewer_seat",
        "final_validator_seat",
    ] {
        assert!(init.contains(variable), "missing {variable}: {init}");
    }

    let (check_code, _, check_stderr) = run(&["check", target], &manifest());
    assert_eq!(check_code, 1);
    assert!(check_stderr.contains("A001 "), "{check_stderr}");
    assert!(
        check_stderr.contains("status is not CONFIGURED"),
        "{check_stderr}"
    );
}
