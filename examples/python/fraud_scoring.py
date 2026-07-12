"""
Fraud Scoring Example — Tempus Engine

Evaluates a fraud-risk rule against a batch of transaction contexts.
Demonstrates RuleDefinition with metadata and batch execution.

Prerequisites:
    cd bindings/python && maturin develop
"""
import json

try:
    import tempus_engine as te
except ImportError as exc:
    raise SystemExit(
        "tempus_engine is not installed.\n"
        "Build it first with:\n"
        "  cd bindings/python && maturin develop"
    ) from exc

rule = {
    "name": "fraud-block",
    "version": "2.0.0",
    "tags": ["fraud", "prod", "payments"],
    "description": "Block transactions with risk score above 0.8",
    "logic": {
        "if": [
            {">": [{"var": "risk_score"}, 0.8]},
            "block",
            {"if": [
                {">": [{"var": "risk_score"}, 0.5]},
                "review",
                "allow"
            ]}
        ]
    }
}

transactions = [
    {"tx_id": "TX001", "risk_score": 0.92, "amount": 5000},
    {"tx_id": "TX002", "risk_score": 0.45, "amount": 120},
    {"tx_id": "TX003", "risk_score": 0.67, "amount": 890},
    {"tx_id": "TX004", "risk_score": 0.12, "amount": 50},
]

rule_json = json.dumps(rule)
contexts = [json.dumps(tx) for tx in transactions]

print(f"Rule: {rule['name']} v{rule['version']}")
print(f"Tags: {', '.join(rule['tags'])}")
print(f"Description: {rule['description']}")
print()
print(f"{'TX ID':<8} {'Risk':>6} {'Amount':>8} {'Decision':<10}")
print("-" * 36)

results = te.execute_batch(rule_json, contexts)
for tx, result_json in zip(transactions, results):
    decision = json.loads(result_json)
    print(f"{tx['tx_id']:<8} {tx['risk_score']:>6.2f} {tx['amount']:>8} {decision:<10}")
