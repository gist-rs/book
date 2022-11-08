# Setup

## 1️⃣ `IDE` and tools

- [Rust in Visual Studio Code](https://code.visualstudio.com/docs/languages/rust)

## 2️⃣ Use `Cargo`

> 💡 📦 [`Cargo`](https://doc.rust-lang.org/cargo/index.html) ≈ `NPM` = Package Manager.

```shell
cargo init foo           # 👈 Will init app name `foo`.

cargo run                # 👈 Build and Run.
cargo watch              # 👈 Watch for file change and rebuild.
cargo test               # 👈 Test the tests if has.

cargo build --release    # 👈 No debug = Smaller/Faster.

cargo add tokio          # 👈 add package named `tokio`
cargo remove tokio       # 👈 remove package named `tokio`.
```

> 💡 [`tokio`](https://tokio.rs/) crate make `async` easier.

## 3️⃣ Hello World

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
fn main() {
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

> ⚡️ You can now skip to 👉 [enjoy](./enjoy.md) coding or continue reading 4️⃣ below. 👇

---

## 4️⃣ Modules and Project structure.

> 🤔 What if `main.rs` has to many codes? Your should separate that concern to each file/folder.

## 🗂 App + File Module

<details>
<summary>Separate some function to each file.</summary>

```yml
📂 foo
├─ 📂 src
│  ├─ 📄 utils.rs    # 👈 module as a file.
│  └─ 📄 main.rs     # 👈 will need utils file.
└─ 📦 Cargo.toml
```

│ ├─ 📄 utils.rs

```rust
pub fn hello() { // 👈 make it public, or just pub(crate) for internal use.
  println!("hello world!");
}
```

│ └─ 📄 main.rs

```rust
mod utils;       // 👈 include utils file.
use utils;        // 👈 and use it.

fn main () {
  utils.hello(); // 👈 call hello function.
}
```

> Now you have too many files and want to group it into folder as a module. See below how to👇

</details>

## 🗂 App + Folder Module

<details>
<summary>Group related files to each folder.</summary>

```yml
📂 foo
├─ 📂 src
│  │
│  ├─ 🗂 utils
│  │  ├─ 📄 mod.rs      # 👈 entrypoint.
│  │  └─ 📄 say.rs      # 👈 Contain hello function.
│  │
│  └─ 📄 main.rs        # 👈 `mod bar;` then `use bar::hello;`
│
└─ 📦 Cargo.toml
```

│ │ ├─ 📄 mod.rs

```rust
pub mod sat;    // 👈 include say file and make it pub so main can use.
```

│ │ └─ 📄 say.rs

```rust
pub fn hello() { // 👈 make it public, or just pub(crate) for internal use.
  println!("hello world!");
}
```

│ └─ 📄 main.rs

```rust
mod utils;        // 👈 include utils file.
use utils::say;   // 👈 and use.

fn main(){
  say.hello();    // 👈 then call hello function.
}
```

> This is better but now you want to reuse that module with other project. Let's make a library then 👇

</details>

## 🗂 Lib

<details>
<summary>Separate each lib as crate.</summary>

```shell
cargo init bar --lib
```

```yml
🗂 utils
├─ 📂 src
│  └─ 📄 lib.rs    # 👈 lib entrypoint.
└─ 📦 Cargo.toml
```

│ └─ 📄 lib.rs

```rust
pub fn hello() {    // 👈  make it pub so other can use.
    println!("hello world!");
}
```

> 🤔 Now you have 3 options to use it.

- Push to github and [use it](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html?highlight=git#specifying-dependencies-from-git-repositories) like this in `Cargo.toml`.
  ```yaml
  [dependencies]
  foo = { git="https://YOU_GITHUB_REPO_URL"}
  ```
- [Publish](https://doc.rust-lang.org/cargo/reference/publishing.html) it to the internet and `cargo add foo` to use it.
- Use it in `Workspace` which is the next topic below.👇

</details>

## 🗂 Workspace

<details>
<summary>1 folder = 1 package.</summary>

```yml
📂 mono-repo-example
│
├─ 🗂 utils         # 👈 lib.
├─ 📂 foo           # 👈 app.
│  ├─ 📄 main.rs
│  └─ 📦 Cargo.toml # 👈 app Cargo.
│
└─ 📦 Cargo.toml    # 👈 Workspace Cargo.
```

│ └─ 📦 Cargo.toml

```yaml
[dependencies]
foo = { path="../utils"}  # 👈 refer to parent mod via path
```

└─ 📦 Cargo.toml

```yaml
[workspace]
members = [
    "seahorse-compile",
```

> Now you are happy! You even can

Let's continue to [Enjoy ➠](./enjoy.md)

</details>
