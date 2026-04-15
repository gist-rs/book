# Setup

## 1️⃣ `IDE` and tools

- [Rust in Visual Studio Code](https://code.visualstudio.com/docs/languages/rust)
- [Rust with Zed](https://zed.dev/)
- [Zed Monokai+Charcoal theme](https://zed-themes.com/?search=Monokai+Charcoal)
- [GLM](https://z.ai/subscribe?ic=VRXWPTEP5N): Smart enough, not cheap anymore but coding plan is good enough(for now)

 - 🎛️ Zed settings that's look like VSCdode with GLM
    ```json
    // Zed settings
    //
    // For information on how to configure Zed, see the Zed
    // documentation: https://zed.dev/docs/configuring-zed
    //
    // To see all of Zed's default settings without changing your
    // custom settings, run the `open default settings` command
    // from the command palette or from `Zed` application menu.
    {
      "agent_buffer_font_size": 12.0,
      "active_pane_modifiers": {
        "border_size": 0.0,
      },
      "agent_ui_font_size": 12.0,
      "terminal": {
        "font_size": 10.0,
      },
      "language_models": {
        "openai_compatible": {
          "GLM": {
            "api_url": "https://api.z.ai/api/coding/paas/v4",
            "available_models": [
              {
                "name": "glm-4.7",
                "display_name": "GLM 4.7",
                "max_tokens": 200000,
                "max_output_tokens": 32000,
                "max_completion_tokens": 200000,
                "capabilities": {
                  "tools": true,
                  "images": true,
                  "parallel_tool_calls": false,
                  "prompt_cache_key": true,
                },
              },
              {
                "name": "glm-5.1",
                "display_name": "GLM 5.1",
                "max_tokens": 200000,
                "max_output_tokens": 32000,
                "max_completion_tokens": 200000,
                "capabilities": {
                  "tools": true,
                  "images": true,
                  "parallel_tool_calls": false,
                  "prompt_cache_key": true,
                },
              },
            ],
          },
          "gemma-4-26b-a4b": {
            "api_url": "http://localhost:1234/v1",
            "available_models": [
              {
                "name": "google/gemma-4-26b-a4b",
                "max_tokens": 200000,
                "max_output_tokens": 32000,
                "max_completion_tokens": 200000,
                "capabilities": {
                  "tools": true,
                  "images": true,
                  "parallel_tool_calls": false,
                  "prompt_cache_key": true,
                },
              },
            ],
          },
          "qwen3-coder-next": {
            "api_url": "http://localhost:1234/v1",
            "available_models": [
              {
                "name": "qwen/qwen3-coder-next",
                "max_tokens": 200000,
                "max_output_tokens": 32000,
                "max_completion_tokens": 200000,
                "capabilities": {
                  "tools": true,
                  "images": true,
                  "parallel_tool_calls": false,
                  "prompt_cache_key": true,
                },
              },
            ],
          },
        },
      },
      "experimental.theme_overrides": {
        "warning.background": "#736e01",
      },
      "show_edit_predictions": false,
      "theme": "Monokai Charcoal (gray)",
      "telemetry": {
        "diagnostics": false,
        "metrics": false,
      },
      "ui_font_size": 12.0,
      "buffer_font_size": 11.0,
      "buffer_font_family": "Menlo",
      "agent": {
        "message_editor_min_lines": 8,
        "inline_assistant_model": {
          "provider": "GLM",
          "model": "glm-4.7",
        },
        "always_allow_tool_actions": true,
        "default_profile": "write",
        "default_model": {
          "provider": "GLM",
          "model": "glm-4.7",
        },
      },
      "lsp": {
        "rust-analyzer": {
          "initialization_options": {
            "check": {
              "command": "clippy",
            },
          },
        },
      },
    }
    ```

### Install extra tools.👇
```bash
cargo install cargo-edit    # 👈 = cargo add, cargo upgrade.
cargo install cargo-watch   # 👈 = cargo watch.
cargo install cargo-audit   # 👈 = cargo audit.
```

## 2️⃣ Use `Cargo`

> 💡 📦 [`Cargo`](https://doc.rust-lang.org/cargo/index.html) ≈ `NPM` = Package Manager.

```bash
cargo init foo           # 👈 Will init app name `foo`.

cargo run                # 👈 Build and Run.
cargo watch              # 👈 Watch for file change and rebuild.
cargo test               # 👈 Test the tests if has.

cargo build --release    # 👈 No debug = Smaller/Faster.

cargo add tokio          # 👈 add package named `tokio`
cargo rm tokio           # 👈 remove package named `tokio`.

cargo upgrade            # 👈 Upgrade current dependencies.
```

> 💡 [`tokio`](https://tokio.rs/) crate make `async` easier.

## 3️⃣ Hello World

> 👩🏻‍💻 enter `cargo init hello-world` via command line.

```yml
📂 hello-world
├─ 📂 src           # 👈 keep source code in here.
│  └─ 📄 main.rs    # 👈 app entrypoint.
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

> ⚡️ You can now skip to 👉 [enjoy](./enjoy1.md) coding or continue reading 4️⃣ below. 👇

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

```rust,no_run,editable
pub fn hello() {    // 👈 make it public, or just pub(crate) for internal use.
  println!("hello world!");
}
```

│ └─ 📄 main.rs

```rust,no_run
mod utils;          // 👈 include utils file.
use utils;          // 👈 and use it.

fn main () {
  utils::hello();    // 👈 call hello function.
}
```

> Now you have too many files and want to group it into folder as a module. See below how to👇

</details>

## 🗂 App + Folder Module

<details>
<summary>Group related files to each folder.</summary>

> See [setup4 example](https://github.com/gist-rs/book/blob/867cb1b1953d475153d58d8f4376c14e62af31d6/examples/r5/setup4)

```yml
📂 foo
├─ 📂 src
│  │
│  ├─ 📂 utils
│  │  ├─ 📄 mod.rs     # 👈 entrypoint (similar to index.js in JS).
│  │  ├─ 📄 say.rs     # 👈 Contain hello function.
│  │  └─ 📄 cast.rs    # 👈 will able to use say.
│  │
│  └─ 📄 main.rs       # 👈 `mod utils;` then `use utils::say;`
│
└─ 📦 Cargo.toml
```

│ │ ├─ 📄 mod.rs

```rust,no_run,editable
pub mod say;        // 👈 import "say" and export.

// 👇 It's look like this in JS.
// export * from say;
```

│ │ ├─ 📄 say.rs

```rust,no_run,editable
pub fn hello() {    // 👈 make it public, or just pub(crate) for internal use.
  println!("hello world!");
}
```

│ │ └─ 📄 cast.rs

```rust,no_run,editable
use super::say      // 👈 just use. (no mod need because of super)

pub fn cast() {
  say::hello();      // 👈 then call hello function.
}
```

│ └─ 📄 main.rs

```rust,no_run
mod utils;          // 👈 include utils file.
use utils::say;     // 👈 and use.

fn main() {
  say::hello();      // 👈 then call hello function.
}
```

> This is better but now you want to reuse that module with other project. Let's make a library then 👇

</details>

## 🗂 Lib

<details>
<summary>Separate each lib as crate.</summary>

```bash
cargo init bar --lib
```

```yml
🗂 utils
├─ 📂 src
│  └─ 📄 lib.rs     # 👈 lib entrypoint.
└─ 📦 Cargo.toml
```

│ └─ 📄 lib.rs

```rust,no_run,editable
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
<summary>aka Monorepo.</summary>

```yml
📂 workspace-example
│
├─ 🗂 utils
│  ├─ 📂 src
│  │  └─ 📄 lib.rs     # 👈 lib entrypoint.
│  └─ 📦 Cargo.toml    # 1️⃣ utils's cargo.
│
├─ 📂 foo
│  ├─ 📂 src
│  │  └─ 📄 main.rs    # 👈 app entrypoint.
│  └─ 📦 Cargo.toml    # 2️⃣ foo's cargo.
│
└─ 📦 Cargo.toml       # 3️⃣ Workspace's cargo.
```

│ └─ 📦 Cargo.toml

```yaml
[dependencies]
utils = { path="../utils" }  # 👈 2️⃣ foo's cargo. refer to utils via path
```

└─ 📦 Cargo.toml

```yaml
# 👇 3️⃣ Workspace's cargo.
[workspace]
members = [
  "utils",
  "foo",
]
```

> See [worksapce example](https://github.com/gist-rs/book/tree/8347d4a8280f87e908e2fec13ea190782c0f3a4b/examples/r5/workspace-example)

</details>

---

## Next

Let's continue to [Enjoy ➠](./enjoy1.md)
