# How to parse json with serde

> 💡 full source code is on [<i id="git-repository-button" class="fa fa-github"></i> github](https://github.com/gist-rs/book/tree/main/examples/r4/10-parse-json-serde)

#### `Cargo.toml`

```toml
{{#include ../../../examples/r4/10-parse-json-serde/Cargo.toml}}
```

#### `main.rs`

```rust,no_run
{{#include ../../../examples/r4/10-parse-json-serde/src/main.rs}}
```

> 🤷‍♂️ `serde_json` is not runnable via Rust Playground so output is shown below.

<details>
<summary>Run</summary>

```
1️⃣ foo_json = [
    Object {
        "id": String("foo"),
        "img_url": String("http://localhost:3000/assets/kat.png"),
        "type": String("Cat"),
    },
    Object {
        "id": String("bar"),
        "img_url": String("http://localhost:3000/assets/duck.png"),
        "type": String("Duck"),
    },
]
2️⃣ filter_and_map_foo_json = [
    Object {
        "id": String("foo"),
        "img_url": String("http://localhost:3000/assets/kat.png"),
        "type": String("Cat"),
    },
]
3️⃣ filter_map_foo_json = [
    Object {
        "id": String("foo"),
        "img_url": String("http://localhost:3000/assets/kat.png"),
        "type": String("Cat"),
    },
]
4️⃣ filtered_foo_value_json = [
    "foo",
]
5️⃣ foo_struct = [
    AnimalData {
        id: "foo",
        type: Cat,
        img_url: Url {
            scheme: "http",
            cannot_be_a_base: false,
            username: "",
            password: None,
            host: Some(
                Domain(
                    "localhost",
                ),
            ),
            port: Some(
                3000,
            ),
            path: "/assets/kat.png",
            query: None,
            fragment: None,
        },
    },
    AnimalData {
        id: "bar",
        type: Duck,
        img_url: Url {
            scheme: "http",
            cannot_be_a_base: false,
            username: "",
            password: None,
            host: Some(
                Domain(
                    "localhost",
                ),
            ),
            port: Some(
                3000,
            ),
            path: "/assets/duck.png",
            query: None,
            fragment: None,
        },
    },
]
6️⃣ bar_struct = AnimalData {
    id: "bar",
    type: Duck,
    img_url: Url {
        scheme: "http",
        cannot_be_a_base: false,
        username: "",
        password: None,
        host: Some(
            Domain(
                "localhost",
            ),
        ),
        port: Some(
            3000,
        ),
        path: "/assets/duck.png",
        query: None,
        fragment: None,
    },
}
```

</details>

## Need More?

> 💡 Read more about `Url` [here](https://rust-lang-nursery.github.io/rust-cookbook/web/url.html)
