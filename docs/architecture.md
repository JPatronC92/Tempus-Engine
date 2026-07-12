# Architecture

## Two-Layer Design

Tempus Engine follows a strict two-layer separation:

```
┌───────────────────────────┐
│      Tempus Engine        │  ← Metadata, governance, batch orchestration
│      (AGPL-3.0)           │
├───────────────────────────┤
│      jsonlogic-fast       │  ← Pure JSON-Logic evaluation
│      (MIT / Apache-2.0)   │
└───────────────────────────┘
```

### Layer 1: jsonlogic-fast (Evaluator)

A neutral, portable JSON-Logic evaluation library. It has no opinion on how rules are stored, named, or managed. It simply takes a JSON-Logic rule and a JSON context, and returns a result.

- **Crate:** [jsonlogic-fast](https://github.com/JPatronC92/First-T-Engine)
- **License:** MIT OR Apache-2.0
- **Targets:** Native (x86/ARM), Python (PyO3), WebAssembly (wasm-bindgen)

### Layer 2: Tempus Engine (Decision Layer)

Wraps the evaluator with everything needed for production decision systems:

- **`RuleDefinition`** — named, versioned, tagged rule container
- **`execute()` / `execute_batch()`** — metadata-aware execution
- **Detailed error reporting** — per-context success/failure in batch mode
- **Engine introspection** — `get_engine_info()` returns version + evaluator info

## Data Flow

```
RuleDefinition { name, logic, version, tags }
        │
        ▼
   execute(&rule, context_json)
        │
        ├─► serialize rule.logic → JSON string
        │
        ├─► jsonlogic_fast::evaluate(rule_json, context_json)
        │
        └─► return Result<Value>
```

## Why This Split?

1. **License clarity** — The evaluator is permissive (MIT/Apache-2.0). The engine layer is copyleft (AGPL-3.0). Users who only need evaluation can use jsonlogic-fast without AGPL obligations.

2. **Focused testing** — Each layer has its own test suite. The evaluator is tested against JSON-Logic semantics. The engine is tested against governance and orchestration behavior.

3. **Independent evolution** — The evaluator can gain new operators or optimizations without changing engine APIs. The engine can add features (explain mode, rule chaining) without touching evaluation logic.
