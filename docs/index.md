# Tempus Engine

**Deterministic rule execution for decision systems.**

Tempus Engine wraps the [jsonlogic-fast](https://github.com/JPatronC92/First-T-Engine) evaluator and adds a metadata layer designed for production decision systems: named rules, versioning, tags, and batch execution with detailed error reporting.

## Quick Start (Rust)

```rust
use tempus_engine::{execute, metadata::RuleDefinition};
use serde_json::json;

let rule = RuleDefinition::new(
    "credit-check",
    json!({"if": [{">": [{"var": "score"}, 700]}, "approve", "review"]}),
)
.with_version("1.0.0")
.with_tags(vec!["credit".into(), "prod".into()]);

let result = execute(&rule, r#"{"score": 800}"#).unwrap();
assert_eq!(result, json!("approve"));
```

## Key Features

| Feature | Description |
|---------|-------------|
| **Rule Metadata** | Name, version, tags, and description attached to every rule |
| **Batch Execution** | Process thousands of contexts in one call with parallel evaluation |
| **Detailed Results** | Per-context success/error reporting for batch operations |
| **Full jsonlogic-fast API** | All evaluation functions re-exported for convenience |
| **Deterministic** | Same input → same output, always |

## License

AGPL-3.0-or-later — see [LICENSE](https://github.com/JPatronC92/Tempus-Engine/blob/main/LICENSE).
