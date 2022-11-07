# Setup

## 1️⃣ 🛠 Install `IDE` and tools.

- [Rust in Visual Studio Code](https://code.visualstudio.com/docs/languages/rust)

## 2️⃣ Use `Cargo`.
> 💡 📦 [`Cargo`](https://doc.rust-lang.org/cargo/index.html) ≈ `NPM` = Package Manager.

```shell
cargo init foo           # 👈 Will init app name `foo`.

cargo run                # 👈 Build and Run.
cargo watch              # 👈 Will watch file change and rebuild.
cargo test               # 👈 Test the tests if has.

cargo build              # 👈 Just build.
cargo build --release    # 👈 No debug = Smaller/Faster.

cargo add tokio          # 👈 add package named `tokio`.
cargo remove tokio       # 👈 remove package named `tokio`.
```
> 💡 [`tokio`](https://tokio.rs/) crate make `async` easier.

## 3️⃣ Try `hello world`.
> 👩🏻‍💻 enter `cargo init hello-world` via command line.
```yml
📂 hello-world
├─ 📂 src            # 👈 keep source code in here.
│  └─ 📄 main.rs     # 👈 app entrypoint.
└─ 📦 Cargo.toml
```

└─ 📄 main.rs
```rust,editable
// 👇 main function as an entrypoint.
fn main(){
  // 👇 macro to print something out.
  println!("hello world!"); // 👈 end with ; suffix.
}
```
└─ 📦 Cargo.toml
```yml
[package]
name = "foo"         # 👈 App name.
version = "0.1.0"    # 👈 App version.
edition = "2021"     # 👈 Rust edition.

[dependencies]
tokio = "1.21.2"     # 👈 Added by `cargo add tokio`.
```

> You can now skip reading and go [enjoy](./enjoy.md) coding or keep digging below. 👇

---

## 🦀 App + File Module
> Separation of concern to each file.

```yml
📂 foo
├─ 📂 src
│  ├─ 📄 utils.rs    # 👈 module as a file.
│  └─ 📄 main.rs     # 👈 `mod utils;` then `use utils;`
└─ 📦 Cargo.toml
```

## 🦀 App + Folder Module
> Separation of concern to each folder.

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
> Separation of concern to each lib as crate.

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
> 1 folder = 1 package.

```yml
📂 mono-repo-example
│
├─ 🗂 bar           # 👈 lib.
├─ 🗂 baz           # 👈 other lib.
├─ 📁 foo           # 👈 app.
│
└─ 📦 Cargo.toml    # 👈 Another Cargo.
```

---

[Enjoy ➠](./enjoy.md)
