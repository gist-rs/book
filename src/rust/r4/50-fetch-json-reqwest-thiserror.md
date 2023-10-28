# How to fetch json with reqwest and handle with thiserror

> 💡 full source code is on [<i id="git-repository-button" class="fa fa-github"></i> github](https://github.com/gist-rs/book/blob/main/examples/r4/50-fetch-json-reqwest-thiserror)

#### `foo.json`

```json
{{#include ../../../examples/r4/50-fetch-json-reqwest-thiserror/src/foo.json}}
```

#### `Cargo.toml`

```toml
{{#include ../../../examples/r4/50-fetch-json-reqwest-thiserror/Cargo.toml}}
```

#### `main.rs`

```rust,edition2021
{{#include ../../../examples/r4/50-fetch-json-reqwest-thiserror/src/main.rs}}
```

> 🤷 `reqwest`+`TLS` is not runnable via Rust Playground so output is shown below.

<details>
<summary>Run</summary>

```
Ok(
    [
        AnimalData {
            id: "foo",
            weight: 123.45,
            created_at: "2022-09-01",
        },
        AnimalData {
            id: "bar",
            weight: 42.2424,
            created_at: "2022-08-01",
        },
    ],
)
```

</details>
