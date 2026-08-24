//! Assurance checker and deterministic YAML-LD to TriG compiler.
//!
//! Provenance: the command/check/compile separation follows bedrock's public
//! architecture; all assurance-specific implementation is original.

pub mod check;
pub mod cli;
pub mod embedded;
pub mod error;
pub mod graph;
pub mod init;
pub mod model;
pub mod yaml_syntax;
