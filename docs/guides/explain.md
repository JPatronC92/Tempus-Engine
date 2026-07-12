# Explain Mode

The explain mode captures a full execution trace for audit, debugging, and GDPR right-to-explanation requirements.

## `ExplainResult` Structure

| Field | Type | Description |
|-------|------|-------------|
| `rule_name` | `String` | Name of the rule |
| `rule_version` | `Option<String>` | Semantic version of the rule |
| `rule_tags` | `Vec<String>` | Tags attached to the rule |
| `logic_snapshot` | `Value` | The JSON-Logic blob that was executed |
| `context_snapshot` | `Value` | The context that was evaluated |
| `result` | `Value` | The final output |
| `evaluated_at` | `String` | ISO-8601 UTC timestamp |
| `engine` | `String` | Engine name and version |

## Rust Example

```rust
use tempus_engine::{execute_explain, metadata::RuleDefinition};
use serde_json::json;

let rule = RuleDefinition::new(
    "fraud-block",
    json!({">": [{"var": "risk_score"}, 0.8]}),
)
.with_version("2.0.0")
.with_tags(vec!["fraud".into(), "prod".into()]);

let trace = execute_explain(&rule, r#"{"risk_score": 0.92}"#).unwrap();

println!("Rule:      {}", trace.rule_name);
println!("Version:   {:?}", trace.rule_version);
println!("Result:    {}", trace.result);
println!("Timestamp: {}", trace.evaluated_at);

// Serialize as JSON for audit log storage
let audit_json = serde_json::to_string_pretty(&trace).unwrap();
```

## Python Example

```python
import json, tempus_engine as te

rule = json.dumps({
    "name": "fraud-block",
    "version": "2.0.0",
    "tags": ["fraud", "prod"],
    "logic": {">": [{"var": "risk_score"}, 0.8]},
})

trace = json.loads(te.execute_explain(rule, '{"risk_score": 0.92}'))

print(f"Decision:  {trace['result']}")
print(f"Evaluated: {trace['evaluated_at']}")
print(f"Engine:    {trace['engine']}")
```

## Use Cases

- **Audit logs** — Store `ExplainResult` as structured JSON alongside transaction records.
- **GDPR Article 22** — Provide individuals an explanation of automated decisions.
- **Debugging** — Compare `context_snapshot` and `logic_snapshot` side-by-side when a decision seems wrong.
- **A/B testing** — Capture traces for both rule versions and compare offline.
