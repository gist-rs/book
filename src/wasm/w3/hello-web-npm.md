# Hello web+npm

## How to publish `wasm` to `npm` and use via `web`.

> 💡 This example use `no bundler` way to keep it simple.
> If you use webpack or other bundler go [here](https://rustwasm.github.io/docs/wasm-bindgen/examples/hello-world.html)

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

> To use with bundler (webpack) see 👉

---
