# Contributing to Tempus Engine

Thank you for your interest in contributing!

## Getting Started

```bash
git clone https://github.com/JPatronC92/Tempus-Engine.git
cd Tempus-Engine
make setup
make ci-local
```

## Development Workflow

1. Fork the repository.
2. Create a feature branch: `git checkout -b feat/your-feature`.
3. Make your changes and add tests.
4. Run `make ci-local` to validate everything passes.
5. Open a pull request against `main`.

## Code Style

- Rust: `cargo fmt` and `cargo clippy` must pass with no warnings.
- Python: follow standard PEP 8 conventions.
- All public APIs must have doc comments.

## Testing

```bash
make test          # Rust unit tests
make test-python   # Python integration tests
```

## License

By contributing, you agree that your contributions will be licensed under the **AGPL-3.0-or-later** license.
