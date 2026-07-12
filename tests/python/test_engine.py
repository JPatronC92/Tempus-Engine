"""
End-to-end pytest suite for the tempus_engine Python bindings.

Run after building the extension with:
    cd bindings/python && maturin develop
Then:
    pytest tests/python/
"""
import json
import pytest

try:
    import tempus_engine as te
    BINDINGS_AVAILABLE = True
except ImportError:
    BINDINGS_AVAILABLE = False

pytestmark = pytest.mark.skipif(
    not BINDINGS_AVAILABLE,
    reason="tempus_engine native extension not built — run `maturin develop` first",
)

# ─────────────────────────────────────────────────────────────────────────────
# Fixtures
# ─────────────────────────────────────────────────────────────────────────────

SCORE_RULE = json.dumps({
    "name": "credit-check",
    "version": "1.0.0",
    "tags": ["credit", "prod"],
    "description": "Approve high scores, review others",
    "logic": {
        "if": [{">": [{"var": "score"}, 700]}, "approve", "review"]
    },
})

FRAUD_RULE = json.dumps({
    "name": "fraud-block",
    "version": "2.0.0",
    "tags": ["fraud"],
    "logic": {
        "if": [
            {">": [{"var": "risk_score"}, 0.8]}, "block",
            {"if": [{">": [{"var": "risk_score"}, 0.5]}, "review", "allow"]}
        ]
    },
})

# ─────────────────────────────────────────────────────────────────────────────
# execute
# ─────────────────────────────────────────────────────────────────────────────

class TestExecute:
    def test_approve_high_score(self):
        result = json.loads(te.execute(SCORE_RULE, '{"score": 800}'))
        assert result == "approve"

    def test_review_low_score(self):
        result = json.loads(te.execute(SCORE_RULE, '{"score": 600}'))
        assert result == "review"

    def test_boundary_score_702_approves(self):
        result = json.loads(te.execute(SCORE_RULE, '{"score": 702}'))
        assert result == "approve"

    def test_invalid_context_raises(self):
        with pytest.raises(ValueError):
            te.execute(SCORE_RULE, "{bad-json}")

    def test_invalid_rule_raises(self):
        with pytest.raises(ValueError):
            te.execute("{bad}", '{"score": 700}')


# ─────────────────────────────────────────────────────────────────────────────
# execute_batch
# ─────────────────────────────────────────────────────────────────────────────

class TestExecuteBatch:
    def test_batch_returns_correct_decisions(self):
        contexts = [
            '{"score": 800}',
            '{"score": 600}',
            '{"score": 742}',
        ]
        results = [json.loads(r) for r in te.execute_batch(SCORE_RULE, contexts)]
        assert results == ["approve", "review", "approve"]

    def test_batch_fraud_scoring(self):
        contexts = [
            '{"risk_score": 0.92}',
            '{"risk_score": 0.45}',
            '{"risk_score": 0.67}',
        ]
        results = [json.loads(r) for r in te.execute_batch(FRAUD_RULE, contexts)]
        assert results == ["block", "allow", "review"]

    def test_empty_batch_returns_empty(self):
        results = te.execute_batch(SCORE_RULE, [])
        assert results == []


# ─────────────────────────────────────────────────────────────────────────────
# execute_chain
# ─────────────────────────────────────────────────────────────────────────────

class TestExecuteChain:
    def test_single_rule_chain(self):
        rules = json.dumps([json.loads(SCORE_RULE)])
        result_json, ctx_json = te.execute_chain(rules, '{"score": 800}')
        assert json.loads(result_json) == "approve"

    def test_two_rule_chain_pipes_decision(self):
        rule1 = {
            "name": "score-gate",
            "logic": {"if": [{">": [{"var": "score"}, 700]}, "approve", "review"]},
        }
        rule2 = {
            "name": "echo-decision",
            "logic": {"var": "decision"},
        }
        rules = json.dumps([rule1, rule2])
        result_json, ctx_json = te.execute_chain(rules, '{"score": 800}')
        result = json.loads(result_json)
        ctx = json.loads(ctx_json)
        assert result == "approve"
        assert ctx["decision"] == "approve"

    def test_empty_chain_raises(self):
        with pytest.raises(ValueError):
            te.execute_chain("[]", '{"score": 800}')


# ─────────────────────────────────────────────────────────────────────────────
# execute_explain
# ─────────────────────────────────────────────────────────────────────────────

class TestExecuteExplain:
    def test_explain_contains_required_fields(self):
        trace = json.loads(te.execute_explain(SCORE_RULE, '{"score": 800}'))
        assert trace["rule_name"] == "credit-check"
        assert trace["rule_version"] == "1.0.0"
        assert trace["result"] == "approve"
        assert trace["context_snapshot"]["score"] == 800
        assert "evaluated_at" in trace
        assert "engine" in trace

    def test_explain_logic_snapshot(self):
        trace = json.loads(te.execute_explain(SCORE_RULE, '{"score": 800}'))
        assert "if" in trace["logic_snapshot"]

    def test_explain_tags(self):
        trace = json.loads(te.execute_explain(SCORE_RULE, '{"score": 800}'))
        assert "credit" in trace["rule_tags"]


# ─────────────────────────────────────────────────────────────────────────────
# get_engine_info
# ─────────────────────────────────────────────────────────────────────────────

class TestGetEngineInfo:
    def test_returns_engine_name(self):
        info = json.loads(te.get_engine_info())
        assert info["engine"] == "tempus-engine"

    def test_returns_version(self):
        info = json.loads(te.get_engine_info())
        assert isinstance(info["version"], str)
        assert len(info["version"]) > 0

    def test_returns_evaluator_info(self):
        info = json.loads(te.get_engine_info())
        assert "evaluator" in info
