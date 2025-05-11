# Hello Cloudflare Github

![](/assets/kat.png) <span class="speech-bubble">Here's [how](https://github.com/gist-rs/hello-world-cloudflare) we release `Rust` as `Wasm` to `Cloudflare` via `Github` integration.</span>

## Setup (once)
```shell
rustup target add wasm32-unknown-unknown
cargo install cargo-generate
```

## Setup new project (optional)
```shell
# From template
cargo generate cloudflare/workers-rs

# Or
npx wrangler init
```

## Or from existing source
```shell
git clone https://github.com/gist-rs/hello-world-cloudflare
cd hello-world-cloudflare
```

## Dev local

```shell
npx wrangler dev
```

## Deploy local

```shell
npx wrangler login
npx wrangler deploy
```

## Deploy command (`Github` Integration)
> 💡 Set this in `Cloudflare` build setting via [Cloudflare Dashboard](https://dash.cloudflare.com/) ref [DOCS](https://developers.cloudflare.com/workers/ci-cd/builds/configuration/)

```shell
npx wrangler deploy -e production
```

## Config

<tabs>
<tab label="wrangler.toml">

```toml
{{#include ../../../examples/r2/hello-world-cloudflare/wrangler.toml}}
```

</tab>
<tab label="Cargo.toml">

```toml
{{#include ../../../examples/r2/hello-world-cloudflare/Cargo.toml}}
```

</tab>
</tabs>

## Code

<tabs>
</tab>
<tab label="lib.rs">

```rust
{{#include ../../../examples/r2/hello-world-cloudflare/src/lib.rs}}
```

</tab>
<tab label="solana.rs">

```rust,editable
{{#include ../../../examples/r2/hello-world-cloudflare/src/solana.rs}}
```

</tab>
</tabs>
