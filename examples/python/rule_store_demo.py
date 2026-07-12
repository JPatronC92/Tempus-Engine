"""
Rule Store Example — Tempus Engine

Demonstrates loading a named collection of rules from a JSON file and
evaluating them against different contexts.

Prerequisites:
    cd bindings/python && maturin develop
"""
import json
import os

try:
    import tempus_engine as te
except ImportError as exc:
    raise SystemExit(
        "tempus_engine is not installed.\n"
        "Build it first with:\n"
        "  cd bindings/python && maturin develop"
    ) from exc

# Rules file path (relative to workspace root)
RULES_PATH = os.path.join(os.path.dirname(__file__), "../../rules/credit.json")

with open(RULES_PATH) as f:
    rules = json.load(f)

store = {r["name"]: r for r in rules}

print(f"Rules loaded from: {os.path.relpath(RULES_PATH)}")
print(f"Loaded {len(rules)} rules from store.\n")

cases = [
    ("credit-check", {"credit_score": 720, "annual_income": 60_000}),
    ("credit-check", {"credit_score": 600, "annual_income": 30_000}),
    ("fraud-block",  {"risk_score": 0.92}),
    ("fraud-block",  {"risk_score": 0.3}),
    # Missing key — validation (the engine returns an error for missing required keys)
    ("credit-check", {"credit_score": 700}),
]

print(f"{'Rule':<15} {'Context':<40} {'Result':<15}")
print("─" * 72)
for rule_name, ctx in cases:
    rule_json = json.dumps(store[rule_name])
    try:
        result = json.loads(te.execute(rule_json, json.dumps(ctx)))
    except ValueError as exc:
        result = f"ERROR: {exc}"
    print(f"{rule_name:<15} {str(ctx):<40} {str(result):<15}")
