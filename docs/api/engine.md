# Engine API Reference

## Core Functions

### `execute`

```rust
pub fn execute(rule_def: &RuleDefinition, context_json: &str) -> RuleEngineResult<Value>
```

Execute a rule definition against a JSON context. Returns the evaluation result as a `serde_json::Value`.

---

### `execute_numeric`

```rust
pub fn execute_numeric(rule_def: &RuleDefinition, context_json: &str) -> RuleEngineResult<f64>
```

Execute a rule and coerce the result to `f64`. Useful for scoring rules.

---

### `execute_batch`

```rust
pub fn execute_batch(
    rule_def: &RuleDefinition,
    contexts_json: &[String],
) -> RuleEngineResult<Vec<Value>>
```

Execute a rule against a slice of context strings. Returns one result per context.

---

### `execute_batch_detailed`

```rust
pub fn execute_batch_detailed(
    rule_def: &RuleDefinition,
    contexts_json: &[String],
) -> RuleEngineResult<Vec<EvaluationResult>>
```

Same as `execute_batch` but returns per-context success/error details. Never short-circuits on errors.

---

### `execute_chain`

```rust
pub fn execute_chain(
    rules: &[RuleDefinition],
    initial_context_json: &str,
) -> RuleEngineResult<(Value, Value)>
```

Execute a sequence of rules. After each rule, its output is injected as `"decision"` into the context for the next rule. Returns `(last_result, final_context)`.

See [Rule Chaining guide](../guides/chaining.md).

---

### `execute_explain`

```rust
pub fn execute_explain(
    rule_def: &RuleDefinition,
    context_json: &str,
) -> RuleEngineResult<ExplainResult>
```

Execute a rule and return a full `ExplainResult` trace with rule metadata, context snapshot, result, and timestamp.

See [Explain Mode guide](../guides/explain.md).

---

### `get_engine_info`

```rust
pub fn get_engine_info() -> Value
```

Returns engine version and evaluator metadata:

```json
{
  "engine": "tempus-engine",
  "version": "0.1.0",
  "evaluator": { "engine": "jsonlogic-fast", "version": "0.1.0" }
}
```

---

## `RuleDefinition`

```rust
pub struct RuleDefinition {
    pub name: String,
    pub logic: Value,
    pub version: Option<String>,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub required_context_keys: Vec<String>,
}
```

### Builder Methods

| Method | Description |
|--------|-------------|
| `new(name, logic)` | Create a minimal rule |
| `.with_version(v)` | Set semantic version |
| `.with_description(d)` | Set human-readable description |
| `.with_tags(tags)` | Set categorization tags |
| `.with_required_keys(keys)` | Declare required context keys |

### `validate_context`

```rust
pub fn validate_context(&self, context: &Value) -> Result<(), ValidationError>
```

Validates that all `required_context_keys` are present in the context object.

---

## `RuleStore`

```rust
pub struct RuleStore { /* ... */ }
```

| Method | Description |
|--------|-------------|
| `new()` | Create empty store |
| `from_json_str(s)` | Load from JSON array string |
| `from_json_file(path)` | Load from JSON file |
| `from_yaml_str(s)` | Load from YAML string *(feature: `yaml`)* |
| `from_yaml_file(path)` | Load from YAML file *(feature: `yaml`)* |
| `get(name)` | Retrieve rule by name |
| `all()` | Slice of all rules |
| `insert(rule)` | Add or replace rule at runtime |

See [Rule Store guide](../guides/store.md).
