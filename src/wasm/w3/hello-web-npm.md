# Hello web+npm

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
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- [npm](https://www.npmjs.com/)

### 2️⃣ Build/Deploy

```shell
# Get example
git clone https://github.com/rustwasm/wasm-bindgen/tree/main/examples/hello_world
cd hello_world

# Build lib `@foo/bar` target `web`.
# Change `foo` and `bar` as you desire.
wasm-pack build --scope foo --target web

# Or maybe no organize by remove `--scope foo`.
wasm-pack build --target web

# Login to npm.
wasm-pack login

# Publish pkg folder.
npm publish pkg --access=public
```

### 3️⃣ Use `wasm` via `web`.

```js
import init, { greet } from '@foo/bar'
init().then({
  greet('World!')
}).catch(console.error);
```
