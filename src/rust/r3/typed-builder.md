# Typed Builder

> 🤔 Refer to [Builder with typestate in Rust](https://www.greyblake.com/blog/builder-with-typestate-in-rust/)

## Do

- Use [Typed Builder](https://crates.io/crates/typed-builder)

```rust
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
struct User {
    id: String,
    email: String,
    #[builder(default)]
    first_name: Option<String>,
    #[builder(default)]
    last_name: Option<String>,
}

fn main() {
    let katopz = User::builder()
        .id("13".into())
        .email("katopz@gmail.com".into())
        .first_name(Some("Kat".into()))
        .build();

    println!(katopz);
}
```

<a href="https://codesandbox.io/p/sandbox/typed-builder-lp6yxk" class="button">▢ CodeSandbox</a>
