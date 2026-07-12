//! # Tempus Engine
//!
//! **Deterministic rule execution for decision systems.**
//!
//! Tempus Engine wraps [jsonlogic-fast](https://crates.io/crates/jsonlogic-fast)
//! with metadata, validation, chaining, explainability, and persistence —
//! everything you need to build auditable, production-grade decision pipelines.
//!
//! ## Quick start
//!
//! ```rust
//! use tempus_engine::metadata::RuleDefinition;
//! use tempus_engine::execute;
//!
//! let rule = RuleDefinition::new(
//!     "age-check",
//!     serde_json::json!({">": [{"var": "age"}, 18]}),
//! );
//! let result = execute(&rule, r#"{"age": 25}"#).unwrap();
//! assert_eq!(result, serde_json::json!(true));
//! ```
//!
//! ## Features
//!
//! | Feature | Description |
//! |---------|-------------|
//! | **Rule metadata** | Name, version, tags, and required context keys |
//! | **Context validation** | Fail fast when required keys are missing |
//! | **Rule chaining** | Pipe output of one rule as input to the next |
//! | **Explain mode** | Full decision trace with timestamps for audit |
//! | **Rule store** | Load rule sets from JSON or YAML files |
//! | **Batch execution** | Evaluate one rule against many contexts |
//!
//! ## Optional features
//!
//! - **`yaml`** — enables YAML rule store loading via `serde_yaml`.

pub mod error;
pub mod explain;
pub mod metadata;
pub mod store;

// Re-export the full jsonlogic-fast public API for convenience.
// Users of tempus-engine get evaluation "for free" without adding
// jsonlogic-fast as a separate dependency.
#[doc(inline)]
pub use jsonlogic_fast::error::{RuleEngineError, RuleEngineResult};
#[doc(inline)]
pub use jsonlogic_fast::extract::{
    extract_array, extract_bool, extract_f64, extract_object, extract_string,
};
#[doc(inline)]
pub use jsonlogic_fast::{
    evaluate, evaluate_batch, evaluate_batch_detailed, evaluate_batch_numeric,
    evaluate_batch_numeric_detailed, evaluate_numeric, evaluate_rule, serialize, serialize_value,
    validate_rule, EvaluationResult, NumericEvaluationResult,
};

#[doc(inline)]
pub use error::ValidationError;
#[doc(inline)]
pub use explain::ExplainResult;
#[doc(inline)]
pub use store::RuleStore;

use metadata::RuleDefinition;
use serde_json::Value;

/// Execute a rule definition (with metadata) against a JSON context.
///
/// This is the primary entry point for Tempus Engine. It wraps the raw
/// jsonlogic-fast evaluation with the rule's metadata context.
///
/// # Examples
///
/// ```rust
/// use tempus_engine::metadata::RuleDefinition;
/// use tempus_engine::execute;
///
/// let rule = RuleDefinition::new(
///     "flag-adult",
///     serde_json::json!({">": [{"var": "age"}, 18]}),
/// );
/// let result = execute(&rule, r#"{"age": 25}"#).unwrap();
/// assert_eq!(result, serde_json::json!(true));
///
/// let result = execute(&rule, r#"{"age": 16}"#).unwrap();
/// assert_eq!(result, serde_json::json!(false));
/// ```
/// Serialize the JSON-Logic blob of a [`RuleDefinition`] to a string.
///
/// Extracted so callers that evaluate the same rule many times can
/// pre-serialise once and pass the result straight to [`evaluate`].
fn serialize_logic(rule_def: &RuleDefinition) -> RuleEngineResult<String> {
    serde_json::to_string(&rule_def.logic).map_err(|e| RuleEngineError::InvalidRule(e.to_string()))
}

pub fn execute(rule_def: &RuleDefinition, context_json: &str) -> RuleEngineResult<Value> {
    let rule_json = serialize_logic(rule_def)?;
    evaluate(&rule_json, context_json)
}

/// Execute a rule definition and coerce the result to f64.
///
/// # Examples
///
/// ```rust
/// use tempus_engine::metadata::RuleDefinition;
/// use tempus_engine::execute_numeric;
///
/// let rule = RuleDefinition::new(
///     "fee-calc",
///     serde_json::json!({"*": [{"var": "amount"}, 0.029]}),
/// );
/// let fee = execute_numeric(&rule, r#"{"amount": 1000}"#).unwrap();
/// assert!((fee - 29.0).abs() < 1e-6);
/// ```
pub fn execute_numeric(rule_def: &RuleDefinition, context_json: &str) -> RuleEngineResult<f64> {
    let rule_json = serde_json::to_string(&rule_def.logic)
        .map_err(|e| RuleEngineError::InvalidRule(e.to_string()))?;
    evaluate_numeric(&rule_json, context_json)
}

/// Execute a rule definition against a batch of contexts.
///
/// # Examples
///
/// ```rust
/// use tempus_engine::metadata::RuleDefinition;
/// use tempus_engine::execute_batch;
///
/// let rule = RuleDefinition::new(
///     "adult-check",
///     serde_json::json!({">": [{"var": "age"}, 18]}),
/// );
/// let contexts = vec![
///     r#"{"age": 25}"#.to_string(),
///     r#"{"age": 16}"#.to_string(),
/// ];
/// let results = execute_batch(&rule, &contexts).unwrap();
/// assert_eq!(results[0], serde_json::json!(true));
/// assert_eq!(results[1], serde_json::json!(false));
/// ```
pub fn execute_batch(
    rule_def: &RuleDefinition,
    contexts_json: &[String],
) -> RuleEngineResult<Vec<Value>> {
    let rule_json = serde_json::to_string(&rule_def.logic)
        .map_err(|e| RuleEngineError::InvalidRule(e.to_string()))?;
    evaluate_batch(&rule_json, contexts_json)
}

/// Execute a rule definition against a batch of contexts with detailed results.
pub fn execute_batch_detailed(
    rule_def: &RuleDefinition,
    contexts_json: &[String],
) -> RuleEngineResult<Vec<EvaluationResult>> {
    let rule_json = serde_json::to_string(&rule_def.logic)
        .map_err(|e| RuleEngineError::InvalidRule(e.to_string()))?;
    evaluate_batch_detailed(&rule_json, contexts_json)
}

/// Execute a sequence of rules where the output of each rule is merged into
/// the context for the next rule (`output → "decision"` key injection).
///
/// The final context — with `"decision"` set to the last rule's output — is
/// returned together with the last result value. This enables decision
/// pipelines such as:
///
///   1. `score-rule`  → output `"high"` →  injected as `{"decision": "high"}`
///   2. `limit-rule`  reads `{"var": "decision"}` and decides the credit limit
///
/// Returns an error if any rule in the chain fails.
///
/// # Examples
///
/// ```rust
/// use tempus_engine::metadata::RuleDefinition;
/// use tempus_engine::execute_chain;
///
/// // Rule 1: classify score as "high" or "low"
/// let classify = RuleDefinition::new(
///     "classify",
///     serde_json::json!({"if": [{">": [{"var": "score"}, 700]}, "high", "low"]}),
/// );
/// // Rule 2: grant limit based on the classification injected as "decision"
/// let limit = RuleDefinition::new(
///     "limit",
///     serde_json::json!({"if": [{"==": [{"var": "decision"}, "high"]}, 10000, 1000]}),
/// );
///
/// let (result, _ctx) = execute_chain(&[classify, limit], r#"{"score": 750}"#).unwrap();
/// assert_eq!(result, serde_json::json!(10000));
/// ```
pub fn execute_chain(
    rules: &[RuleDefinition],
    initial_context_json: &str,
) -> RuleEngineResult<(Value, Value)> {
    if rules.is_empty() {
        return Err(RuleEngineError::InvalidRule(
            "rule chain must contain at least one rule".to_owned(),
        ));
    }
    let mut context: Value = serde_json::from_str(initial_context_json)
        .map_err(|e| RuleEngineError::InvalidRule(format!("invalid context JSON: {e}")))?;

    let mut last_result = Value::Null;
    let len = rules.len();
    for (i, rule) in rules.iter().enumerate() {
        let rule_json = serialize_logic(rule)?;
        let ctx_str = serde_json::to_string(&context)
            .map_err(|e| RuleEngineError::InvalidRule(e.to_string()))?;
        last_result = evaluate(&rule_json, &ctx_str)?;
        // Inject the result as "decision" into the context for the next rule.
        // Skip on the last iteration — no subsequent rule will read it.
        if i + 1 < len {
            if let Some(obj) = context.as_object_mut() {
                obj.insert("decision".to_owned(), last_result.clone());
            }
        }
    }
    // Ensure the final context also carries the last decision for the caller.
    if let Some(obj) = context.as_object_mut() {
        obj.insert("decision".to_owned(), last_result.clone());
    }
    Ok((last_result, context))
}

/// Execute a rule with full tracing metadata attached to the result.
///
/// Returns an [`ExplainResult`] that records the rule snapshot, context
/// snapshot, final decision, and timestamp — suitable for audit logs and
/// right-to-explanation compliance (e.g. GDPR Article 22).
///
/// # Examples
///
/// ```rust
/// use tempus_engine::metadata::RuleDefinition;
/// use tempus_engine::execute_explain;
///
/// let rule = RuleDefinition::new(
///     "credit-check",
///     serde_json::json!({">": [{"var": "score"}, 700]}),
/// ).with_version("1.0.0");
///
/// let trace = execute_explain(&rule, r#"{"score": 750}"#).unwrap();
/// assert_eq!(trace.rule_name, "credit-check");
/// assert_eq!(trace.result, serde_json::json!(true));
/// assert!(!trace.evaluated_at.is_empty());
/// ```
pub fn execute_explain(
    rule_def: &RuleDefinition,
    context_json: &str,
) -> RuleEngineResult<ExplainResult> {
    let context_snapshot: Value = serde_json::from_str(context_json)
        .map_err(|e| RuleEngineError::InvalidRule(format!("invalid context JSON: {e}")))?;
    let rule_json = serialize_logic(rule_def)?;
    let result = evaluate(&rule_json, context_json)?;
    Ok(ExplainResult::new(
        rule_def.name.clone(),
        rule_def.version.clone(),
        rule_def.tags.clone(),
        rule_def.logic.clone(),
        context_snapshot,
        result,
    ))
}

/// Return engine information including the underlying jsonlogic-fast version.
///
/// # Examples
///
/// ```rust
/// let info = tempus_engine::get_engine_info();
/// assert_eq!(info["engine"], "tempus-engine");
/// assert!(info["version"].is_string());
/// ```
pub fn get_engine_info() -> Value {
    let core_info = jsonlogic_fast::get_core_info();
    serde_json::json!({
        "engine": "tempus-engine",
        "version": env!("CARGO_PKG_VERSION"),
        "evaluator": core_info,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn sample_rule() -> RuleDefinition {
        RuleDefinition::new(
            "score-check",
            json!({"if": [{">": [{"var": "score"}, 700]}, "approve", "review"]}),
        )
    }

    #[test]
    fn execute_returns_correct_result() {
        let rule = sample_rule();
        let result = execute(&rule, r#"{"score": 800}"#).unwrap();
        assert_eq!(result, json!("approve"));
    }

    #[test]
    fn execute_returns_review_for_low_score() {
        let rule = sample_rule();
        let result = execute(&rule, r#"{"score": 500}"#).unwrap();
        assert_eq!(result, json!("review"));
    }

    #[test]
    fn execute_numeric_works() {
        let rule = RuleDefinition::new("fee-calc", json!({"*": [{"var": "amount"}, 0.029]}));
        let result = execute_numeric(&rule, r#"{"amount": 1000}"#).unwrap();
        assert!((result - 29.0).abs() < 1e-6);
    }

    #[test]
    fn execute_batch_processes_multiple_contexts() {
        let rule = sample_rule();
        let contexts = vec![
            r#"{"score": 800}"#.to_string(),
            r#"{"score": 500}"#.to_string(),
            r#"{"score": 742}"#.to_string(),
        ];
        let results = execute_batch(&rule, &contexts).unwrap();
        assert_eq!(
            results,
            vec![json!("approve"), json!("review"), json!("approve")]
        );
    }

    #[test]
    fn execute_batch_detailed_reports_errors() {
        let rule = sample_rule();
        let contexts = vec![r#"{"score": 800}"#.to_string(), "{bad json}".to_string()];
        let results = execute_batch_detailed(&rule, &contexts).unwrap();
        assert_eq!(results[0].result, Some(json!("approve")));
        assert!(results[1].error.is_some());
    }

    #[test]
    fn get_engine_info_returns_version() {
        let info = get_engine_info();
        assert_eq!(info["engine"], "tempus-engine");
        assert!(info["version"].as_str().is_some());
        assert!(info["evaluator"]["engine"].as_str().is_some());
    }

    #[test]
    fn rule_metadata_is_preserved() {
        let mut rule = sample_rule();
        rule.version = Some("1.2.0".to_string());
        rule.tags = vec!["credit".to_string(), "prod".to_string()];

        assert_eq!(rule.name, "score-check");
        assert_eq!(rule.version, Some("1.2.0".to_string()));
        assert_eq!(rule.tags.len(), 2);

        // Metadata doesn't affect evaluation
        let result = execute(&rule, r#"{"score": 800}"#).unwrap();
        assert_eq!(result, json!("approve"));
    }

    #[test]
    fn re_exported_evaluate_works_directly() {
        let result = evaluate(r#"{">":[{"var":"x"},1]}"#, r#"{"x":5}"#).unwrap();
        assert_eq!(result, json!(true));
    }

    #[test]
    fn re_exported_validate_rule_works() {
        assert!(validate_rule(r#"{">":[{"var":"x"},1]}"#).unwrap());
    }

    // --- execute_chain tests ---

    #[test]
    fn execute_chain_single_rule() {
        let rules = vec![sample_rule()];
        let (result, _ctx) = execute_chain(&rules, r#"{"score": 800}"#).unwrap();
        assert_eq!(result, json!("approve"));
    }

    #[test]
    fn execute_chain_pipes_decision_to_next_rule() {
        // Rule 1: score → "approve" or "review"
        let rule1 = RuleDefinition::new(
            "score-gate",
            json!({"if": [{">": [{"var": "score"}, 700]}, "approve", "review"]}),
        );
        // Rule 2: reads "decision" from context
        let rule2 = RuleDefinition::new("decision-logger", json!({"var": "decision"}));
        let (result, ctx) = execute_chain(&[rule1, rule2], r#"{"score": 800}"#).unwrap();
        assert_eq!(result, json!("approve"));
        assert_eq!(ctx["decision"], json!("approve"));
    }

    #[test]
    fn execute_chain_empty_returns_error() {
        assert!(execute_chain(&[], r#"{}"#).is_err());
    }

    // --- execute_explain tests ---

    #[test]
    fn execute_explain_captures_metadata() {
        let rule = sample_rule()
            .with_version("1.0.0")
            .with_tags(vec!["credit".into()]);
        let trace = execute_explain(&rule, r#"{"score": 800}"#).unwrap();
        assert_eq!(trace.rule_name, "score-check");
        assert_eq!(trace.rule_version.as_deref(), Some("1.0.0"));
        assert_eq!(trace.result, json!("approve"));
        assert_eq!(trace.context_snapshot["score"], json!(800));
        assert!(!trace.evaluated_at.is_empty());
    }
}
