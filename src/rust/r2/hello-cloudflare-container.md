# Hello Cloudflare Container (Rust)

![](/assets/kat.png) <span class="speech-bubble">Here's an example for `Rust` on `Cloudflare` via `Container`.</span>

## Setup
> 💡 Refer to [https://developers.cloudflare.com/containers/]()

```bash
npm create cloudflare@latest -- --template=cloudflare/templates/containers-template
```

![](/assets/duck.png) <span class="speech-bubble">You will get `Golang` from template then just replace with `Rust` related below.</span>

## Config

<tabs>
<tab label="Dockerfile">

```docker
{{#include ../../../examples/r2/hello-cloudflare-container/Dockerfile}}
```

</tab>
<tab label="Cargo.toml">

```toml
{{#include ../../../examples/r2/hello-cloudflare-container/container_src/Cargo.toml}}
```

</tab>
</tabs>

## Code

<tabs>
<tab label="main.rs">

```rust
{{#include ../../../examples/r2/hello-cloudflare-container/container_src/src/main.rs}}
```

</tab>
<tab label="index.ts">

```js
{{#include ../../../examples/r2/hello-cloudflare-container/src/index.ts}}
```

</tab>
</tabs>

## Dev & Deploy
```bash
npm run dev
npm run deploy
```
