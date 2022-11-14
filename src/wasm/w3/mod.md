# W3 - Advanced

# Goals

- Able to deploy and use `wasm` as `node_modules`

## How to publish `wasm` to `npm` and use via `web`.

### 1️⃣ Prerequisites

- [node](https://nodejs.org/en/download/package-manager/#macos)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- [npm](https://www.npmjs.com/)

### 2️⃣ Build/Deploy

```shell
# Get example
git clone https://github.com/rustwasm/wasm-bindgen/tree/main/examples/hello_world
cd hello_world

# Build lib `@foo/bar` target `web`
# Change `foo` and `bar` as you desire.
wasm-pack build --scope foo --target web

# Or maybe no organize by remove `--scope foo`.
wasm-pack build --target web

# Publish pkg folder
wasm-pack login
npm publish pkg --access=public
```

### 3️⃣ Use `wasm` via `web`.

```js
import init, { greet } from '@foo/bar'
init().then({
  greet('World!')
}).catch(console.error);
```
