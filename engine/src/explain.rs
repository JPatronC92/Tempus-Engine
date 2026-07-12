//! Explainability support for auditable decision traces.
//!
//! Use [`crate::execute_explain`] to get an [`ExplainResult`] containing
//! the full execution snapshot — rule, context, result, and timestamp.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The outcome of an [`crate::execute_explain`] call.
///
/// Contains the evaluation result together with a full execution trace
/// for audit, debugging, and GDPR right-to-explanation use cases.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplainResult {
    /// Name of the rule that was evaluated.
    pub rule_name: String,
    /// Semantic version of the rule, if set.
    pub rule_version: Option<String>,
    /// Tags attached to the rule.
    pub rule_tags: Vec<String>,
    /// The raw JSON-Logic blob that was executed.
    pub logic_snapshot: Value,
    /// The context that was passed to the evaluator.
    pub context_snapshot: Value,
    /// The final evaluation output.
    pub result: Value,
    /// RFC-3339 UTC timestamp (populated at call time).
    pub evaluated_at: String,
    /// Name of the engine and its version.
    pub engine: String,
}

impl ExplainResult {
    pub(crate) fn new(
        rule_name: String,
        rule_version: Option<String>,
        rule_tags: Vec<String>,
        logic_snapshot: Value,
        context_snapshot: Value,
        result: Value,
    ) -> Self {
        Self {
            rule_name,
            rule_version,
            rule_tags,
            logic_snapshot,
            context_snapshot,
            result,
            evaluated_at: Utc::now().to_rfc3339(),
            engine: format!("tempus-engine@{}", env!("CARGO_PKG_VERSION")),
        }
    }
}
