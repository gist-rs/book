# Build a fetch module

![](/assets/kat.png) We able to fetch and handle things nicely, it's time to reuse it as a module.

```rust
📂 fetch-any-lib
│
├─ 🗂 fetch-any
│  ├─ 📂 src
│  │  └─ 📄 lib.rs        # 👈 lib entrypoint.
│  └─ 📦 Cargo.toml
│
├─ 📂 examples
│  ├─ 📂 foo
│  │  ├─ 📂 src
│  │  │  └─ 📄 main.rs    # 👈 app entrypoint.
│  │  └─ 📦 Cargo.toml
│
└─ 📦 Cargo.toml          # 👈 Workspace's Cargo.
```

> 🚧 UNDER CONSTRUCTION
