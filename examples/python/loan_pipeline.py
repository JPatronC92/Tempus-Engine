"""
Loan Pipeline Example — Tempus Engine (Rule Chaining)

Demonstrates a two-stage decision pipeline using rule chaining:
  Stage 1 — Credit gate: approve / review / deny based on credit score
  Stage 2 — Limit check: approved loans must be within the income-based limit

Uses the rule chaining pattern (output → "decision" → next rule context).

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
# Rule definitions
# ─────────────────────────────────────────────────────────────────────────────

CREDIT_GATE = {
    "name": "credit-gate",
    "version": "1.2.0",
    "tags": ["credit", "prod"],
    "required_context_keys": ["credit_score", "annual_income"],
    "logic": {
        "if": [
            {"and": [
                {">=": [{"var": "credit_score"}, 680]},
                {">=": [{"var": "annual_income"}, 45000]},
            ]},
            "approved",
            {"if": [
                {"and": [
                    {">=": [{"var": "credit_score"}, 620]},
                    {">=": [{"var": "annual_income"}, 35000]},
                ]},
                "manual_review",
                "denied",
            ]},
        ]
    },
}

LIMIT_CHECK = {
    "name": "limit-check",
    "version": "1.0.0",
    "tags": ["credit", "prod"],
    "required_context_keys": ["requested_amount", "annual_income"],
    "logic": {
        "if": [
            {"!=": [{"var": "decision"}, "approved"]},
            {"var": "decision"},         # pass through non-approved decisions unchanged
            {"if": [
                {"<=": [{"var": "requested_amount"}, {"*": [{"var": "annual_income"}, 4]}]},
                "approved",
                "limit_exceeded",
            ]},
        ]
    },
}

# ─────────────────────────────────────────────────────────────────────────────
# Applicant data
# ─────────────────────────────────────────────────────────────────────────────

applicants = [
    {"id": "A001", "credit_score": 740, "annual_income": 72_000, "requested_amount": 200_000},
    {"id": "A002", "credit_score": 650, "annual_income": 40_000, "requested_amount": 90_000},
    {"id": "A003", "credit_score": 580, "annual_income": 28_000, "requested_amount": 50_000},
    {"id": "A004", "credit_score": 700, "annual_income": 55_000, "requested_amount": 300_000},  # limit exceeded
    {"id": "A005", "credit_score": 720, "annual_income": 90_000, "requested_amount": 250_000},
]

rules_json = json.dumps([CREDIT_GATE, LIMIT_CHECK])

# ─────────────────────────────────────────────────────────────────────────────
# Output
# ─────────────────────────────────────────────────────────────────────────────

print("Loan Pipeline — Two-stage rule chain")
print(f"Stage 1: {CREDIT_GATE['name']} v{CREDIT_GATE['version']}")
print(f"Stage 2: {LIMIT_CHECK['name']} v{LIMIT_CHECK['version']}")
print()
print(f"{'ID':<6} {'Score':>6} {'Income':>9} {'Amount':>10} {'Decision':<18}")
print("─" * 55)

for app in applicants:
    result_json, _ctx_json = te.execute_chain(rules_json, json.dumps(app))
    decision = json.loads(result_json)
    print(
        f"{app['id']:<6} {app['credit_score']:>6} "
        f"{app['annual_income']:>9,} {app['requested_amount']:>10,} "
        f"{decision:<18}"
    )
