# Rule Store

`RuleStore` provides a named collection of `RuleDefinition` objects loaded from disk or strings at startup.

## Load from JSON

**File format** (`rules/credit.json`):

```json
[
  {
    "name": "credit-check",
    "version": "1.3.0",
    "tags": ["credit", "prod"],
    "logic": { "if": [{ ">=": [{ "var": "score" }, 680] }, "approve", "deny"] }
  },
  {
    "name": "income-check",
    "version": "1.0.0",
    "required_context_keys": ["annual_income"],
    "logic": { ">=": [{ "var": "annual_income" }, 35000] }
  }
]
```

```rust
use tempus_engine::RuleStore;

let store = RuleStore::from_json_file("rules/credit.json").unwrap();

if let Some(rule) = store.get("credit-check") {
    let result = tempus_engine::execute(rule, r#"{"score": 720}"#).unwrap();
    println!("{result}");
}
```

## Load from YAML (feature: `yaml`)

Enable in `Cargo.toml`:

```toml
tempus-engine = { ..., features = ["yaml"] }
```

**File format** (`rules/credit.yaml`):

```yaml
- name: credit-check
  version: "1.3.0"
  tags: [credit, prod]
  logic:
    if:
      - ">=": [{var: score}, 680]
      - approve
      - deny
```

```rust
let store = tempus_engine::RuleStore::from_yaml_file("rules/credit.yaml").unwrap();
```

## Runtime Insertion

```rust
use tempus_engine::{RuleStore, metadata::RuleDefinition};
use serde_json::json;

let mut store = RuleStore::new();
store.insert(
    RuleDefinition::new("dynamic-rule", json!({"==": [1, 1]})).with_version("0.1.0"),
);
```

## Context Validation

`RuleDefinition` can declare which context keys it requires:

```rust
let rule = RuleDefinition::new("income-check", json!({">=": [{"var": "income"}, 35000]}))
    .with_required_keys(vec!["income".into()]);

let ctx = serde_json::json!({"score": 700});  // missing "income"
match rule.validate_context(&ctx) {
    Ok(())   => println!("context is valid"),
    Err(e)   => println!("validation failed: {e}"),
}
```
