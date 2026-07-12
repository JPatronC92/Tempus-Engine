.PHONY: setup test clippy fmt audit deny ci-local test-python

setup:
	rustup component add clippy rustfmt
	cargo install cargo-deny || true
	cargo install cargo-audit || true

fmt:
	cargo fmt --all -- --check

clippy:
	cargo clippy --workspace --all-targets -- -D warnings

test:
	cargo test --workspace

audit:
	cargo audit

deny:
	cargo deny check

test-python:
	cd engine && uv run --with maturin --with pytest bash -c \
		"maturin develop --release && pytest ../tests/python/ -v"

ci-local: fmt clippy test audit deny
	@echo "All checks passed."
