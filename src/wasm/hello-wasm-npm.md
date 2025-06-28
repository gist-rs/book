# Hello Wasm NPM

## How to publish `wasm` to `npm` and use via `web`.

<details>
<summary>🚥 We will use <code>no-bundler</code> style.</summary>

### You have 2 options here

- `bundler` (default, support `webpack`)
- `no-bundler` (init before use).

We use [vite](https://vitejs.dev/guide/why.html) which is still has [Wasm bundle issue](https://github.com/vitejs/vite/issues/4551) so we tend to use `no-bundler` in the meantime to avoid its and to keep it simple (bundler free).

> 💡 For `bundler` via webpack go [here](https://rustwasm.github.io/docs/wasm-bindgen/examples/hello-world.html)

</details>

### 1️⃣ Prerequisites

- [node](https://nodejs.org/en/download/package-manager/#macos)
- [npm](https://www.npmjs.com/)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
  ```bash
  cargo install wasm-pack
  ```

### 2️⃣ Build/Deploy
```bash
# Get example
git clone https://github.com/rustwasm/wasm-bindgen/tree/main/examples/hello_world
cd hello_world

# Build lib `@foo/bar` target `web`.
# Change `foo` and `bar` as you desire.
wasm-pack build --scope foo --target nodejs

# Or maybe no organize by remove `--scope foo` and target web.
wasm-pack build --target web

# Login to npm.
wasm-pack login

# Publish pkg folder.
wasm-pack publish --access=public
```

#### Optional: no wasm-pack

```bash
# Build
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/hello_world.wasm --out-dir pkg --nodejs

# Release
npm publish pkg --access=public
```

### 3️⃣ Use `wasm`

#### via `web`.
```js
import init, { greet } from '@foo/bar'
init().then({
  greet('World!');
}).catch(console.error);
```

#### via `nodejs`.
```js
import { greet } from '@foo/bar'
greet('World!');
```
