# Create your own lib

## How to implement extra feature to `lib`.

- lib `hi.rs`

  ```rust,no_run
  // 👇 exclamation ! mean apply to whole file.
  #![cfg(feature = "hi")]
  pub fn hi(){
    println!(":#?", base64::encode("hi"));
  }
  ```

- lib `Cargo.toml`

  ```toml
  [package]
  name = "foo"

  [dependencies]
  # optional true = include only when need 👇
  base64 =  { version = "0.13", optional = true }

  [features]
  hi = ["dep:base64"] # 👈 related crate.
  ```

## How to use extra feature from `lib`.

- app `Cargo.toml`

  ```toml
  [package]
  name = "bar"

  [dependencies]
  # To include feature named "hi" 👇
  foo = { version = "1", features = ["hi"] }
  ```

## How to use your `lib` from `path`

```toml
[dependencies]
pyth-sdk-solana = { path = "../pyth-sdk-solana" }
```

## How to use your `lib` from `github`

```toml
[dependencies]
pyth-sdk-solana = { git = "https://github.com/pyth-network/pyth-sdk-rs", rev = "75e2742" }
```

---

## What common traits we should derive?

- `Debug`
- `Send` for `Mutex`
- `Sync` for `Arc`
- `Clone`
- `Default`
- `PartialEq`, `PartialOrd`, `Hash`, `Eq`, and `Ord` for comparison.
- `Serialize`, `Deserialize`

> 💡 You can read more about this at at [Rust for Rustaceans](https://nostarch.com/rust-rustaceans)

## Which function restrictions shall we use?

```rust
// Take owned String, return owned String.
fn hello(foo: String) -> String

// Take ref to str, return Copy on Write.
fn hello(foo: &str) -> Cow<'_, str>

// Take impl, return impl.
fn hello(foo: impl AsRef<str>) -> impl AsRef<str>
```

> 💡 You can read more about `Cow` here 👉 [6 things you can do with the Cow 🐄 in Rust 🦀](https://dev.to/kgrech/6-things-you-can-do-with-the-cow-in-rust-4l55)
