# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] — 2026-04-04

### Added

#### Core engine
- `RuleDefinition` struct with builder pattern for metadata-rich rule management (name, version, tags, required context keys).
- `execute`, `execute_numeric`, `execute_batch`, `execute_batch_detailed` — metadata-aware wrappers over `jsonlogic-fast`.
- `execute_chain` — pipe output of one rule as `"decision"` into the next rule's context.
- `execute_explain` — full decision trace with timestamps for audit trails.
- `RuleStore` — load named rule sets from JSON or YAML files.
- `validate_context` — fail fast when required context keys are missing.
- `get_engine_info()` — engine version and evaluator introspection.
- Full re-export of `jsonlogic-fast` public API for zero-friction adoption.

#### Bindings
- **Python** (PyO3 + maturin): `execute`, `execute_batch`, `execute_chain`, `execute_explain`, `get_engine_info`.
- **WASM** (wasm-bindgen): `execute`, `execute_batch`, `execute_chain`, `get_engine_info`.

#### Infrastructure
- CI pipeline: fmt, clippy, test, cargo-audit, cargo-deny.
- Python bindings CI: build + pytest across Python 3.10–3.12.
- MkDocs documentation with Material theme, deployed to GitHub Pages.
- Domain examples: fraud scoring, credit eligibility (Python).
- 13 Rust unit tests, comprehensive Python e2e test suite.
- AGPL-3.0-or-later license.
