# Rule Chaining

Rule chaining allows you to build decision pipelines where the output of one rule becomes available as `"decision"` in the context for the next rule.

## How It Works

```
Context:  { "score": 800 }
              ↓
Rule 1 (score-gate):  output → "approve"
              ↓ inject: { "score": 800, "decision": "approve" }
Rule 2 (limit-check): reads {"var": "decision"} → "approve"
              ↓ return ("approve", final_context)
```

## Rust Example

```rust
use tempus_engine::{execute_chain, metadata::RuleDefinition};
use serde_json::json;

let rule1 = RuleDefinition::new(
    "score-gate",
    json!({"if": [{">": [{"var": "score"}, 700]}, "approve", "review"]}),
);

let rule2 = RuleDefinition::new(
    "final-decision",
    // Only approve if score passed AND amount is under limit
    json!({"if": [
        {"and": [
            {"==": [{"var": "decision"}, "approve"]},
            {"<=": [{"var": "amount"}, 50000]}
        ]},
        "approved",
        "manual_review"
    ]}),
);

let (result, ctx) = execute_chain(
    &[rule1, rule2],
    r#"{"score": 800, "amount": 30000}"#,
).unwrap();

assert_eq!(result, json!("approved"));
assert_eq!(ctx["decision"], json!("approved"));
```

## Python Example

```python
import json, tempus_engine as te

rules = json.dumps([
    {"name": "score-gate",
     "logic": {"if": [{">": [{"var": "score"}, 700]}, "approve", "review"]}},
    {"name": "limit-check",
     "logic": {"if": [
         {"and": [{"==": [{"var": "decision"}, "approve"]},
                  {"<=": [{"var": "amount"}, 50000]}]},
         "approved", "manual_review"
     ]}},
])

result_json, ctx_json = te.execute_chain(rules, '{"score": 800, "amount": 30000}')
print(json.loads(result_json))  # "approved"
```

## Notes

- The result of each rule is injected as `"decision"` into the context consumed by the subsequent rule.
- The original context keys are preserved throughout the chain.
- Returns an error if the chain is empty or any rule evaluation fails.
