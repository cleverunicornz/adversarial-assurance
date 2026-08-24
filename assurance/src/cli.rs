//! Dependency-free command parser and dispatcher.

use crate::error::Fatal;
use std::path::PathBuf;

pub const HELP: &str = "\
assurance — validate and compile Promise/Witness/Oracle YAML-LD records.

Usage:
  assurance init  [DIR]  install .assurance/ and the witness workflow, then print bootstrap instructions
  assurance check [DIR]  validate adoption bindings, schema, records, edges, witnesses, and transitions
  assurance build [DIR]  refuse unless check passes, then write one byte-stable TriG graph per run
  assurance help         print this contract
  assurance --version    print the binary version

DIR defaults to the current directory. Check failures are one line each:
  RULE path:line message

CI is the sole witnessed checker seat. Local check/build is optional authoring
preflight and never a merge-authoritative fallback or witness.
";

pub enum Command {
    Init(PathBuf),
    Check(PathBuf),
    Build(PathBuf),
    Help,
    Version,
}

pub fn parse(args: &[String]) -> Result<Command, Fatal> {
    if args.is_empty() {
        return Ok(Command::Help);
    }
    if matches!(args[0].as_str(), "help" | "-h" | "--help") {
        return Ok(Command::Help);
    }
    if matches!(args[0].as_str(), "-V" | "--version") {
        return Ok(Command::Version);
    }
    if args.len() > 2 {
        return Err(Fatal(
            "expected one command and at most one DIR; try `assurance help`".to_owned(),
        ));
    }
    let root = args
        .get(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    match args[0].as_str() {
        "init" => Ok(Command::Init(root)),
        "check" => Ok(Command::Check(root)),
        "build" => Ok(Command::Build(root)),
        command if command.starts_with('-') => Err(Fatal(format!(
            "unknown flag `{command}`; try `assurance help`"
        ))),
        command => Err(Fatal(format!(
            "unknown command `{command}`; try `assurance help`"
        ))),
    }
}

pub fn run(command: Command) -> Result<i32, Fatal> {
    match command {
        Command::Help => {
            print!("{HELP}");
            Ok(0)
        }
        Command::Version => {
            println!("assurance {}", env!("CARGO_PKG_VERSION"));
            Ok(0)
        }
        Command::Init(root) => {
            crate::init::run(&root)?;
            Ok(0)
        }
        Command::Check(root) => {
            let report = crate::check::inspect(&root)?;
            if report.violations.is_empty() {
                println!("assurance check: 0 violations");
                Ok(0)
            } else {
                crate::check::print_violations(&report.violations);
                println!("assurance check: {} violation(s)", report.violations.len());
                Ok(1)
            }
        }
        Command::Build(root) => {
            let report = crate::check::inspect(&root)?;
            if !report.violations.is_empty() {
                crate::check::print_violations(&report.violations);
                return Err(Fatal(format!(
                    "assurance build aborted: check failed with {} violation(s)",
                    report.violations.len()
                )));
            }
            let count = crate::graph::write_all(&root, &report.documents)?;
            println!("assurance build: {count} run graph(s) written");
            Ok(0)
        }
    }
}
