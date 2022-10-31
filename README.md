# gist-book

Rust, Wasm, TLDR;

## TODO

- [x] `User::Guest` can see `hello` section.
- [x] `User::Guest` can see `bye` section.
- [x] `User::Guest` can see `R5` section.
- [x] `User::Guest` can see and able to run `editor` section.
- [ ] `User::Guest` can read as step by step.
- [ ] `User::Guest` can see `rust-by-practice` section.
- [ ] `User::Guest` can see `rustling` section.
- [ ] `User::Guest` can `auth` with `web3`.
- [ ] `User::Guest` can't see `freemium` section.
- [ ] `User::FREEMIUM` can see `freemium` section after `auth`.
- [ ] `User::FREEMIUM` can see `R4` section.
- [ ] `User::FREEMIUM` can't see `premium` section.
- [ ] `User::PREMIUM` can see `premium` section if has `NFT`.
- [ ] `User::PREMIUM` can see `R3` section.
- [ ] `User::PREMIUM` can see `R2` section.
- [ ] `User::PREMIUM` can see `R1` section.

## POC

> Project structure

```
🦀 Project
├─ 📂 src
│  ├─ 📄 main.rs
│  └─ 📄 lib.rs
└─ 📦 Cargo.toml
```

```
* Project
+ src
  - main.rs
  - lib.rs
# Cargo.toml
```

```json
[
  {
    "id": "0",
    "type": "folder",
    "label": "src",
    "children": ["0_0", "0_1"]
  },
  {
    "id": "0_0",
    "type": "file",
    "label": "main.rs"
  },
  {
    "id": "0_1",
    "type": "file",
    "label": "lib.rs"
  },
  {
    "id": "1",
    "type": "file",
    "label": "Cargo.toml"
  }
]
```
