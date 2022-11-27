# How to parse json with serde

```rust
{{#include ../../../examples/r4/10-parse-json-serde/src/main.rs}}
```

> 🤷‍♂️ `serde_json` is not runnable via Rust Playground so output is shown below.

<details>
<summary>Run</summary>

```
1️⃣ foo_json = [
    Object {
        "id": String("foo"),
    },
    Object {
        "id": String("bar"),
    },
]
2️⃣ filter_and_map_foo_json = [
    Object {
        "id": String("foo"),
    },
]
3️⃣ filter_map_foo_json = [
    Object {
        "id": String("foo"),
    },
]
4️⃣ filtered_foo_value_json = [
    "foo",
]
```

</details>
