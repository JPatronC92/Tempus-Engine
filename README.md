
# Tempus Engine

[![crates.io](https://img.shields.io/crates/v/tempus-engine.svg)](https://crates.io/crates/tempus-engine)
[![docs.rs](https://docs.rs/tempus-engine/badge.svg)](https://docs.rs/tempus-engine)
[![CI](https://github.com/JPatronC92/Tempus-Engine/actions/workflows/ci.yml/badge.svg)](https://github.com/JPatronC92/Tempus-Engine/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/JPatronC92/Tempus-Engine/graph/badge.svg)](https://codecov.io/gh/JPatronC92/Tempus-Engine)

**Deterministic rule execution for decision systems.**

Built on [jsonlogic-fast](https://crates.io/crates/jsonlogic-fast) for portable, high-performance JSON-Logic evaluation.

## What Tempus adds over raw JSON-Logic

| jsonlogic-fast                  | Tempus Engine                              |
| ------------------------------- | ------------------------------------------ |
| Evaluate a rule against context | Execute rules with metadata and validation |
| Raw `serde_json::Value` output  | Structured decision results                |
| Performance-first               | Decision-systems-first                     |
| Stateless evaluation            | Foundation for audit, explain, governance  |

## Use cases

- **Fraud scoring** — block, review, or allow transactions based on risk signals
- **Credit eligibility** — approve/deny with deterministic, auditable logic
- **Dynamic pricing** — compute fees and discounts from rule definitions
- **Policy enforcement** — evaluate compliance rules across regulated workflows
- **Feature flags** — gate features with structured rule logic

## Quick start

### Prerequisites

- Rust 1.75+
- Python 3.10+
- [`uv`](https://docs.astral.sh/uv/)

### Build and test

```bash
make setup        # Build Python bindings
make test         # Run Rust core tests
make test-python  # Run Python e2e tests (29 tests)
make test-wasm    # Run WASM runtime tests (15 tests)
make ci-local     # Full quality gate
```

## Architecture

```text
┌──────────────────────────────────────────────────┐
│              Tempus Engine (this repo)            │
│  Rule metadata · Rich validation · Domain demos  │
│  Foundation for: explain · audit · time layers   │
└────────────────────┬─────────────────────────────┘
                     │ depends on
┌────────────────────▼─────────────────────────────┐
│           jsonlogic-fast (separate repo)          │
│  Fast, embeddable, cross-runtime JSON-Logic eval  │
│  Rust core · Python bindings · WASM bindings      │
└──────────────────────────────────────────────────┘
```

## Repository structure

```text
Tempus-Engine/
├── engine/               # Rust crate: tempus-engine
│   └── src/
│       ├── lib.rs        # Re-exports jsonlogic-fast + Tempus extensions
│       └── metadata.rs   # Rule metadata (name, version, tags)
├── examples/             # Domain-specific usage examples
│   └── python/
│       ├── fraud_scoring.py
│       └── eligibility.py
├── docs/                 # Product documentation
├── tests/
│   └── python/           # Python e2e tests
└── Makefile
```

## Roadmap

| Phase | What                                          | Status      |
| ----- | --------------------------------------------- | ----------- |
| 1     | Rule metadata, rich validation, domain demos  | **Done** ✅ |
| 2     | Explain mode (decision trace / audit trail)   | **Done** ✅ |
| 3     | Rule chaining & persistence (JSON/YAML)       | **Done** ✅ |
| 4     | Python & WASM bindings                        | **Done** ✅ |
| 5     | Time layer (effective-date rule selection)     | Planned     |
| 6     | Decision database (receipts, replay, hashing) | Planned     |

## License

Tempus Engine is licensed under **[AGPL-3.0-or-later](LICENSE)**.

### What AGPL means for you

| Your use case | AGPL obligation |
|---|---|
| Internal tooling, not network-exposed | No source disclosure required |
| SaaS / API that calls Tempus Engine | Must release **your entire application** under AGPL |
| Embedding in a closed-source product | Must release product source under AGPL |
| Open-source project (compatible license) | Use freely; keep attribution |

If the AGPL does not fit your business model, [contact us](https://github.com/JPatronC92) to discuss a commercial license.

### Dependency licenses

The underlying evaluation engine ([jsonlogic-fast](https://github.com/JPatronC92/First-T-Engine)) is **MIT OR Apache-2.0** — permissive, no copyleft obligations. Only the Tempus Engine wrapper layer carries the AGPL.
