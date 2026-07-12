# WebAssembly Bindings

Tempus Engine can run in any JavaScript environment via [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/).

## Build

```bash
# Install wasm-pack
cargo install wasm-pack

# Web target (ES modules)
cd bindings/wasm
wasm-pack build --target web --out-dir pkg

# Node.js target
wasm-pack build --target nodejs --out-dir pkg-node
```

## Usage (Browser / ESM)

```js
import init, { execute, executeBatch, executeChain, executeExplain, getEngineInfo }
    from './pkg/tempus_engine_wasm.js';

await init();

const rule = JSON.stringify({
  name: "credit-check",
  version: "1.0.0",
  logic: { "if": [{ ">": [{ "var": "score" }, 700] }, "approve", "review"] },
});

const result = JSON.parse(execute(rule, '{"score": 800}'));
// → "approve"
```

## Usage (Node.js)

```js
const { execute, getEngineInfo } = require('./pkg-node/tempus_engine_wasm.js');

const result = JSON.parse(execute(ruleJson, contextJson));
```

## API

| Function | Signature | Notes |
|----------|-----------|-------|
| `execute` | `(ruleJson, contextJson) → string` | Main evaluation |
| `executeBatch` | `(ruleJson, contextsArray) → string[]` | Batch evaluation |
| `executeChain` | `(rulesJson, contextJson) → {result, context}` | Rule pipeline |
| `executeExplain` | `(ruleJson, contextJson) → string` | Explain trace |
| `getEngineInfo` | `() → string` | Engine metadata |

All functions throw `Error` on invalid JSON or evaluation failure.
