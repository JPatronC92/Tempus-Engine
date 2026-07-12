#![allow(clippy::useless_conversion)] // PyO3 proc-macros generate identity conversions

use ::tempus_engine::metadata::RuleDefinition;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use serde_json::Value;

// Aliases to avoid name clash with the #[pymodule] fn tempus_engine below.
use ::tempus_engine::{
    execute as te_execute, execute_batch as te_execute_batch, execute_chain as te_execute_chain,
    execute_explain as te_execute_explain, get_engine_info as te_get_engine_info,
};

// ──────────────────────────────────────────────────────────────────────────────
// Helper
// ──────────────────────────────────────────────────────────────────────────────

fn parse_rule(rule_json: &str) -> PyResult<RuleDefinition> {
    serde_json::from_str(rule_json)
        .map_err(|e| PyValueError::new_err(format!("invalid rule JSON: {e}")))
}

fn to_json_string(v: &Value) -> PyResult<String> {
    serde_json::to_string(v).map_err(|e| PyValueError::new_err(e.to_string()))
}

// ──────────────────────────────────────────────────────────────────────────────
// Exposed functions
// ──────────────────────────────────────────────────────────────────────────────

/// Execute a rule definition JSON against a context JSON.
///
/// Args:
///     rule_json:    JSON string representing a ``RuleDefinition``.
///     context_json: JSON string representing the evaluation context.
///
/// Returns:
///     JSON string with the evaluation result.
///
/// Raises:
///     ValueError: if either argument is not valid JSON or the rule fails.
#[pyfunction]
fn execute(rule_json: &str, context_json: &str) -> PyResult<String> {
    let rule = parse_rule(rule_json)?;
    let result =
        te_execute(&rule, context_json).map_err(|e| PyValueError::new_err(e.to_string()))?;
    to_json_string(&result)
}

/// Execute a rule definition against a batch of context JSON strings.
///
/// Args:
///     rule_json:     JSON string representing a ``RuleDefinition``.
///     contexts_json: List of JSON context strings.
///
/// Returns:
///     List of JSON result strings (one per context).
///
/// Raises:
///     ValueError: on rule or context parse errors.
#[pyfunction]
fn execute_batch(rule_json: &str, contexts_json: Vec<String>) -> PyResult<Vec<String>> {
    let rule = parse_rule(rule_json)?;
    let results = te_execute_batch(&rule, &contexts_json)
        .map_err(|e| PyValueError::new_err(e.to_string()))?;
    results.iter().map(to_json_string).collect()
}

/// Execute a chain of rules, piping each output as ``"decision"`` into the
/// next rule's context.
///
/// Args:
///     rules_json:           JSON array of ``RuleDefinition`` objects.
///     initial_context_json: Starting context JSON string.
///
/// Returns:
///     Tuple ``(result_json, final_context_json)``.
///
/// Raises:
///     ValueError: on parse errors or empty chain.
#[pyfunction]
fn execute_chain(rules_json: &str, initial_context_json: &str) -> PyResult<(String, String)> {
    let rules: Vec<RuleDefinition> = serde_json::from_str(rules_json)
        .map_err(|e| PyValueError::new_err(format!("invalid rules JSON: {e}")))?;
    let (result, ctx) = te_execute_chain(&rules, initial_context_json)
        .map_err(|e| PyValueError::new_err(e.to_string()))?;
    Ok((to_json_string(&result)?, to_json_string(&ctx)?))
}

/// Execute a rule and return a full explanation trace as a JSON string.
///
/// The returned JSON includes rule metadata, context snapshot, result,
/// and an ISO-8601 timestamp.
///
/// Args:
///     rule_json:    JSON string representing a ``RuleDefinition``.
///     context_json: JSON string representing the evaluation context.
///
/// Returns:
///     JSON string with the ``ExplainResult`` structure.
///
/// Raises:
///     ValueError: on parse errors or evaluation failure.
#[pyfunction]
fn execute_explain(rule_json: &str, context_json: &str) -> PyResult<String> {
    let rule = parse_rule(rule_json)?;
    let trace = te_execute_explain(&rule, context_json)
        .map_err(|e| PyValueError::new_err(e.to_string()))?;
    serde_json::to_string(&trace).map_err(|e| PyValueError::new_err(e.to_string()))
}

/// Return engine metadata as a JSON string.
///
/// Returns:
///     JSON string with ``engine``, ``version``, and ``evaluator`` fields.
#[pyfunction]
fn get_engine_info() -> PyResult<String> {
    let info = te_get_engine_info();
    to_json_string(&info)
}

// ──────────────────────────────────────────────────────────────────────────────
// Module registration
// ──────────────────────────────────────────────────────────────────────────────

/// Tempus Engine — Python bindings.
#[pymodule]
fn tempus_engine(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(execute, m)?)?;
    m.add_function(wrap_pyfunction!(execute_batch, m)?)?;
    m.add_function(wrap_pyfunction!(execute_chain, m)?)?;
    m.add_function(wrap_pyfunction!(execute_explain, m)?)?;
    m.add_function(wrap_pyfunction!(get_engine_info, m)?)?;
    Ok(())
}
