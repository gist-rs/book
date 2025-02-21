# Fastly Forward Request

## How to forward request to other server with Fastly

![](/assets/kat.png) <span class="speech-bubble">Let's Rust!</span>

## Setup Fastly CLI

```
# Install Homebrew
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install Fastly
brew install fastly/tap/fastly
```

## Init Fastly

```
mkdir fastly-forward-request
cd fastly-forward-request
fastly compute init
```

---

![](/assets/duck.png) <span class="speech-bubble">Coding time!</span>

## Code

<tabs>
<tab label="main.rs">

```rust,no_run
{{#include ../../examples/wasm/fastly-forward-request/src/main.rs}}
```

</tab>
<tab label="Cargo.toml">

```toml
{{#include ../../examples/wasm/fastly-forward-request/Cargo.toml}}
```

</tab>
</tabs>

## Dev

```
fastly compute serve

```

## Deploy

```
fastly compute publish
```
