//! Stable failure diagnostics.
//!
//! Every validation failure renders as `RULE path:line message`; callers sort
//! violations before printing so identical input produces identical output.

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Violation {
    pub rule: &'static str,
    pub path: String,
    pub line: u32,
    pub message: String,
}

impl Violation {
    pub fn new(
        rule: &'static str,
        path: impl Into<String>,
        line: u32,
        message: impl Into<String>,
    ) -> Self {
        Self {
            rule,
            path: path.into(),
            line: line.max(1),
            message: message.into(),
        }
    }
}

impl std::fmt::Display for Violation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{} {}:{} {}",
            self.rule, self.path, self.line, self.message
        )
    }
}

#[derive(Debug)]
pub struct Fatal(pub String);

impl std::fmt::Display for Fatal {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for Fatal {}

impl From<&str> for Fatal {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

pub fn line_of(source: &str, needle: &str) -> u32 {
    source
        .lines()
        .position(|line| line.contains(needle))
        .map(|index| index as u32 + 1)
        .unwrap_or(1)
}
