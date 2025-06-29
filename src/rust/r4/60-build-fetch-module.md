# Build a fetch module

![](/assets/kat.png) <span class="speech-bubble">We able to fetch and handle errors from previously examples, it's time to reuse it as a module!</span>

```
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

> 🚧 You can do this as a homework, no need to wait for me!
