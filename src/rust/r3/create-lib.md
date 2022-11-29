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
