//! # tempus-engine-wasm
//!
//! **WebAssembly bindings for [Tempus Engine](https://crates.io/crates/tempus-engine).**
//!
//! Wraps the Rust rule engine in a `wasm-bindgen` layer so it can be called from
//! **browser JavaScript**, **Node.js**, or any WASM runtime without a native
//! Rust toolchain on the consumer side.
//!
//! ## Exported functions
//!
//! | JS name | Purpose |
//! |---------|---------|
//! | `execute(ruleJson, contextJson)` | Evaluate one rule against one context |
//! | `executeBatch(ruleJson, contextsJson[])` | Evaluate one rule against many contexts |
//! | `executeChain(rulesJson[], initialContextJson)` | Pipeline: each output feeds the next rule |
//! | `executeExplain(ruleJson, contextJson)` | Full audit trace with timestamp |
//! | `getEngineInfo()` | Engine name and version metadata |
//!
//! ## Usage (JavaScript / Node.js)
//!
//! ```js
//! import init, { execute } from "tempus-engine-wasm";
//!
//! await init();
//!
//! const rule = JSON.stringify({
//!   name: "age-check",
//!   logic: { ">": [{ var: "age" }, 18] }
//! });
//!
//! const result = execute(rule, JSON.stringify({ age: 25 }));
//! console.log(JSON.parse(result)); // true
//! ```
//!
//! ## Rule definition format
//!
//! All functions expect a **JSON-serialised `RuleDefinition`**:
//!
//! ```json
//! {
//!   "name": "my-rule",
//!   "version": "1.0.0",
//!   "logic": { ">": [{ "var": "score" }, 700] },
//!   "tags": ["credit"],
//!   "required_context_keys": ["score"]
//! }
//! ```
//!
//! Only `name` and `logic` are required; all other fields are optional.
//!
//! ## Error handling
//!
//! Every function throws a JavaScript `Error` (via `JsValue`) on invalid JSON
//! input or evaluation failure. Wrap calls in `try/catch`.
//!
//! ## See also
//!
//! - [`tempus-engine`](https://crates.io/crates/tempus-engine) — the Rust core crate
//! - [GitHub repository](https://github.com/JPatronC92/Tempus-Engine)

use tempus_engine::metadata::RuleDefinition;
use wasm_bindgen::prelude::*;

// ──────────────────────────────────────────────────────────────────────────────
// Helper
// ──────────────────────────────────────────────────────────────────────────────

fn parse_rule(rule_json: &str) -> Result<RuleDefinition, JsValue> {
    serde_json::from_str(rule_json)
        .map_err(|e| JsValue::from_str(&format!("invalid rule JSON: {e}")))
}

fn to_js_string(v: &serde_json::Value) -> Result<String, JsValue> {
    serde_json::to_string(v).map_err(|e| JsValue::from_str(&e.to_string()))
}

// ──────────────────────────────────────────────────────────────────────────────
// Exported functions
// ──────────────────────────────────────────────────────────────────────────────

/// Execute a rule definition JSON against a context JSON string.
///
/// @param {string} ruleJson    - JSON representation of a `RuleDefinition`.
/// @param {string} contextJson - JSON context object.
/// @returns {string}           - JSON string with the evaluation result.
/// @throws  {Error}            - on invalid JSON or evaluation failure.
#[wasm_bindgen(js_name = execute)]
pub fn execute(rule_json: &str, context_json: &str) -> Result<String, JsValue> {
    let rule = parse_rule(rule_json)?;
    let result = tempus_engine::execute(&rule, context_json)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    to_js_string(&result)
}

/// Execute a rule against an array of context JSON strings.
///
/// @param {string}   ruleJson     - JSON representation of a `RuleDefinition`.
/// @param {string[]} contextsJson - Array of JSON context strings.
/// @returns {string[]}            - Array of JSON result strings.
/// @throws  {Error}               - on parse or evaluation failure.
#[wasm_bindgen(js_name = executeBatch)]
pub fn execute_batch(
    rule_json: &str,
    contexts_json: js_sys::Array,
) -> Result<js_sys::Array, JsValue> {
    let rule = parse_rule(rule_json)?;
    let contexts: Vec<String> = contexts_json
        .iter()
        .map(|v| {
            v.as_string()
                .ok_or_else(|| JsValue::from_str("context array must contain strings"))
        })
        .collect::<Result<_, _>>()?;

    let results = tempus_engine::execute_batch(&rule, &contexts)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let out = js_sys::Array::new();
    for r in &results {
        out.push(&JsValue::from_str(&to_js_string(r)?));
    }
    Ok(out)
}

/// Execute a chain of rules, injecting each output as `"decision"` into the
/// next rule's context.
///
/// @param {string} rulesJson          - JSON array of `RuleDefinition` objects.
/// @param {string} initialContextJson - Starting context JSON string.
/// @returns {{ result: string, context: string }} - Result and final context.
/// @throws  {Error}                               - on parse errors or empty chain.
#[wasm_bindgen(js_name = executeChain)]
pub fn execute_chain(
    rules_json: &str,
    initial_context_json: &str,
) -> Result<js_sys::Object, JsValue> {
    let rules: Vec<RuleDefinition> = serde_json::from_str(rules_json)
        .map_err(|e| JsValue::from_str(&format!("invalid rules JSON: {e}")))?;

    let (result, ctx) = tempus_engine::execute_chain(&rules, initial_context_json)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let obj = js_sys::Object::new();
    js_sys::Reflect::set(
        &obj,
        &"result".into(),
        &JsValue::from_str(&to_js_string(&result)?),
    )?;
    js_sys::Reflect::set(
        &obj,
        &"context".into(),
        &JsValue::from_str(&to_js_string(&ctx)?),
    )?;
    Ok(obj)
}

/// Execute a rule and return a full explanation trace as a JSON string.
///
/// @param {string} ruleJson    - JSON representation of a `RuleDefinition`.
/// @param {string} contextJson - JSON context object.
/// @returns {string}           - JSON string with the `ExplainResult` structure.
/// @throws  {Error}            - on parse errors or evaluation failure.
#[wasm_bindgen(js_name = executeExplain)]
pub fn execute_explain(rule_json: &str, context_json: &str) -> Result<String, JsValue> {
    let rule = parse_rule(rule_json)?;
    let trace = tempus_engine::execute_explain(&rule, context_json)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    serde_json::to_string(&trace).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Return engine metadata as a JSON string.
///
/// @returns {string} - JSON with `engine`, `version`, and `evaluator` fields.
#[wasm_bindgen(js_name = getEngineInfo)]
pub fn get_engine_info() -> Result<String, JsValue> {
    let info = tempus_engine::get_engine_info();
    to_js_string(&info)
}
