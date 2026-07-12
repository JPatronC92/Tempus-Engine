//! Validation error types for context schema enforcement.

use std::fmt;

/// Error raised when a context fails schema validation for a rule.
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationError {
    /// Name of the rule that rejected the context.
    pub rule_name: String,
    /// Keys that were required but absent from the context.
    pub missing_keys: Vec<String>,
}

impl ValidationError {
    pub(crate) fn new(rule_name: &str, missing_keys: Vec<String>) -> Self {
        Self {
            rule_name: rule_name.to_owned(),
            missing_keys,
        }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "rule '{}' validation failed: missing context keys: [{}]",
            self.rule_name,
            self.missing_keys.join(", ")
        )
    }
}

impl std::error::Error for ValidationError {}
