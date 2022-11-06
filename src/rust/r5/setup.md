# Setup

## 🛠 Install `IDE` and tools.

- [Rust in Visual Studio Code](https://code.visualstudio.com/docs/languages/rust)

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
```

## 🦀 App

```shell
cargo init foo
```

```yml
📂 foo
├─ 📂 src
│  └─ 📄 main.rs     # 👈 app entrypoint.
└─ 📦 Cargo.toml
```

## 🦀 Rust file

```rust,editable
// 👇 main function as an entrypoint.
fn main(){
  // 👇 macro to print something out.
  println!("hello world!"); // 👈 end with ; suffix.
}
```

---

## 🦀 App + File Module

```yml
📂 foo
├─ 📂 src
│  ├─ 📄 utils.rs    # 👈 module as a file.
│  └─ 📄 main.rs     # 👈 `mod utils;` then `use utils;`
└─ 📦 Cargo.toml
```

## 🦀 App + Folder Module

```yml
📂 foo
├─ 📂 src
│  │
│  ├─ 🗂 bar            # 👈 module as a folder.
│  │  ├─ 📄 mod.rs      # 👈 entrypoint.
│  │  ├─ 📄 hello.rs    # 👈 some file.
│  │  └─ 📄 world.rs    # 👈 other file.
│  │
│  └─ 📄 main.rs        # 👈 `mod bar;` then `use bar::hello;`
│
└─ 📦 Cargo.toml
```

## 🦀 Lib

```shell
cargo init bar --lib
```

```yml
🗂 bar
├─ 📂 src
│  └─ 📄 lib.rs    # 👈 lib entrypoint.
└─ 📦 Cargo.toml
```

## 🦀 Mono-repo

```yml
📂 mono-repo-example
│
├─ 🗂 bar           # 👈 lib.
├─ 🗂 baz           # 👈 other lib.
├─ 📁 foo           # 👈 app.
│
└─ 📦 Cargo.toml    # 👈 Another Cargo.
```

## 📦 Cargo.toml

```yml
[package]
name = "foo"         # 👈 App name.
version = "0.1.0"    # 👈 App version.
edition = "2021"     # 👈 Rust edition.

[dependencies]
tokio = "1.21.2"     # 👈 Added by `cargo add tokio`.
```

[Develop ➠](./develop.md)
