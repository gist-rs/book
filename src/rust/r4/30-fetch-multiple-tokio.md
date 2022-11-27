# How to fetch multiple with tokio

> 🚧 `#[tokio::main]` Not support `wasm`, do use `futures` instead.

#### `foo.json`

```json
{{#include ../../../examples/r4/30-fetch-multiple-tokio/src/foo.json}}
```

#### `bar.json`

```json
{{#include ../../../examples/r4/30-fetch-multiple-tokio/src/bar.json}}
```

#### `Cargo.toml`

```toml
{{#include ../../../examples/r4/30-fetch-multiple-tokio/Cargo.toml}}
```

#### `main.rs`

```rust,edition2021
{{#include ../../../examples/r4/30-fetch-multiple-tokio/src/main.rs}}
```

> 🤷‍♂️ `reqwest`+`TLS` is not runnable via Rust Playground so output is shown below.

<details>
<summary>Run</summary>

```

```

</details>
