# Init

## 📦 Cargo

`Cargo` ≈ `NPM` = Package Manager.

### Init

> How to init app name `foo`.

```shell
cargo init foo
```

### Dev

```shell
cargo build
cargo run
cargo test
```

### Release

```shell
cargo build --release
```

### Add/Remove package (crate)

> How to add, remove package named `tokio`.

```shell
cargo add tokio
cargo remove tokio

# Something else
cargo help
```

## 🦀 App

```shell
cargo init foo
```

```
📂 foo
├─ 📂 src
│  └─ 📄 main.rs # 👈 Your app entrypoint.
└─ 📦 Cargo.toml
```

## 🦀 Lib

```shell
cargo init bar --lib
```

```
🗂 bar
├─ 📂 src
│  └─ 📄 lib.rs # 👈 Your lib entrypoint.
└─ 📦 Cargo.toml
```

## 🦀 Mono-repo

```
📂 mono-example
├─ 🗂 bar # 👈 Your lib.
├─ 🗂 baz # 👈 Your other lib.
├─ 📁 foo # 👈 Your app.
└─ 📦 Cargo.toml
```

## 📦 Cargo.toml

```yml
[package]
name = "foo" # 👈 Your app name.
version = "0.1.0" # 👈 Your app version.
edition = "2021" # 👈 Rust edition.

[dependencies]
tokio = "1.21.2" # 👈 Added by `cargo add tokio`.
```

## 🦀 Rust file

```rust,editable
// 👇 main function as entrypoint.
fn main(){
  // 👇 macro to print something out.
  println!("hello world!"); // 👈 end with ; suffix.
}
```
