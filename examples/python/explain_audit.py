"""
Explain Mode Example — Tempus Engine

Demonstrates the explain mode: each decision is logged as a structured
JSON trace containing rule metadata, context snapshot, and timestamp.

Useful for:
  - Audit logs
  - GDPR Article 22 explanations
  - Debugging unexpected decisions

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

# ─────────────────────────────────────────────────────────────────────────────
# Rule definition
# ─────────────────────────────────────────────────────────────────────────────

FRAUD_RULE = {
    "name": "fraud-block",
    "version": "3.1.0",
    "tags": ["fraud", "payments", "prod"],
    "description": "Block or review transactions above risk thresholds",
    "required_context_keys": ["risk_score", "tx_id"],
    "logic": {
        "if": [
            {">": [{"var": "risk_score"}, 0.8]}, "block",
            {"if": [
                {">": [{"var": "risk_score"}, 0.5]}, "review",
                "allow",
            ]},
        ]
    },
}

# ─────────────────────────────────────────────────────────────────────────────
# Transaction data
# ─────────────────────────────────────────────────────────────────────────────

transactions = [
    {"tx_id": "TX-001", "risk_score": 0.92, "amount": 5000, "merchant": "HighRisk Ltd"},
    {"tx_id": "TX-002", "risk_score": 0.45, "amount": 120,  "merchant": "Supermarket"},
    {"tx_id": "TX-003", "risk_score": 0.67, "amount": 890,  "merchant": "Electronics Co"},
    {"tx_id": "TX-004", "risk_score": 0.12, "amount": 50,   "merchant": "Coffee Shop"},
    {"tx_id": "TX-005", "risk_score": 0.88, "amount": 2200, "merchant": "Unknown Vendor"},
]

# ─────────────────────────────────────────────────────────────────────────────
# Output
# ─────────────────────────────────────────────────────────────────────────────

rule_json = json.dumps(FRAUD_RULE)

print(f"Rule: {FRAUD_RULE['name']} v{FRAUD_RULE['version']}")
print()

audit_log = []
for tx in transactions:
    trace = json.loads(te.execute_explain(rule_json, json.dumps(tx)))
    audit_log.append(trace)
    print(f"  TX {tx['tx_id']:>7}  risk={tx['risk_score']:.2f}  "
          f"amount={tx['amount']:>5}  → {trace['result']:<8}  "
          f"[{trace['evaluated_at']}]")

print()
print("─── Audit log (JSON) ───────────────────────────────")
print(json.dumps(audit_log[0], indent=2))  # show first trace as sample
