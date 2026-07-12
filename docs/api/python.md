# Python Bindings

Tempus Engine exposes its full API to Python via [PyO3](https://pyo3.rs/).

## Installation

```bash
pip install maturin
cd bindings/python
maturin develop   # development build
# or: maturin build --release  (wheel)
```

## Functions

### `execute(rule_json, context_json) → str`

Execute a rule against a context. Returns the result as a JSON string.

```python
import json
import tempus_engine as te

rule = json.dumps({
    "name": "credit-check",
    "version": "1.0.0",
    "logic": {"if": [{">": [{"var": "score"}, 700]}, "approve", "review"]},
})

result = json.loads(te.execute(rule, '{"score": 800}'))
# → "approve"
```

---

### `execute_batch(rule_json, contexts_json) → list[str]`

```python
contexts = ['{"score": 800}', '{"score": 600}', '{"score": 742}']
results = [json.loads(r) for r in te.execute_batch(rule, contexts)]
# → ["approve", "review", "approve"]
```

---

### `execute_chain(rules_json, initial_context_json) → tuple[str, str]`

Returns `(result_json, final_context_json)`. Each rule's output is injected as `"decision"` into the next rule's context.

```python
rules = json.dumps([rule1_dict, rule2_dict])
result_json, ctx_json = te.execute_chain(rules, '{"score": 800}')
```

---

### `execute_explain(rule_json, context_json) → str`

Returns a JSON `ExplainResult` with full trace information.

```python
trace = json.loads(te.execute_explain(rule, '{"score": 800}'))
print(trace["rule_name"])     # "credit-check"
print(trace["result"])        # "approve"
print(trace["evaluated_at"])  # "2026-04-04T..."
```

---

### `get_engine_info() → str`

```python
info = json.loads(te.get_engine_info())
# {"engine": "tempus-engine", "version": "0.1.0", ...}
```
