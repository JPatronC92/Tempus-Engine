"""
Eligibility Check Example — Tempus Engine

Evaluates loan eligibility rules with versioned rule definitions.

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
    "name": "loan-eligibility",
    "version": "1.3.0",
    "tags": ["lending", "prod"],
    "description": "Determine loan eligibility based on credit score and income",
    "logic": {
        "if": [
            {"and": [
                {">=": [{"var": "credit_score"}, 680]},
                {">=": [{"var": "annual_income"}, 45000]}
            ]},
            "approved",
            {"if": [
                {"and": [
                    {">=": [{"var": "credit_score"}, 620]},
                    {">=": [{"var": "annual_income"}, 35000]}
                ]},
                "manual_review",
                "denied"
            ]}
        ]
    }
}

applicants = [
    {"id": "A001", "credit_score": 740, "annual_income": 72000},
    {"id": "A002", "credit_score": 650, "annual_income": 40000},
    {"id": "A003", "credit_score": 580, "annual_income": 28000},
    {"id": "A004", "credit_score": 700, "annual_income": 50000},
]

rule_json = json.dumps(rule)
contexts = [json.dumps(app) for app in applicants]

print(f"Rule: {rule['name']} v{rule['version']}")
print()
print(f"{'ID':<6} {'Score':>6} {'Income':>8} {'Decision':<15}")
print("-" * 40)

results = te.execute_batch(rule_json, contexts)
for app, result_json in zip(applicants, results):
    decision = json.loads(result_json)
    # Demo script: intentionally prints synthetic example data to stdout.
    print(f"{app['id']:<6} {app['credit_score']:>6} {app['annual_income']:>8} {decision:<15}")
