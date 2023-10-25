# WIP Bevy

```
ld: cannot find -lshlwapi: No such file or directory
```

```
# ⚠️ Not working
rustup toolchain install stable-x86_64-pc-windows-gnu

# Switch to msvc
rustup default stable-x86_64-pc-windows-msvc
```

## Note

- [x] Hot reload: https://github.com/lee-orr/dexterous_developer // Not seem to work and will need `XCode` to be installed for faster reload.
- [x] Game template: https://github.com/NiklasEi/bevy_game_template
- [x] Load scene from Blender: https://github.com/kaosat-dev/Blender_bevy_components_workflow // Work but animation will failed.
- [x] Renet: Renet is a network library for Server/Client game: https://github.com/lucaspoffo/renet
- [x] bevy_xpbd: 2D and 3D physics engine based on Extended Position Based Dynamics for Bevy. https://github.com/Jondolf/bevy_xpbd // Not ready for prod.
- [x] Sea : https://github.com/claudijo/pirate-sea-jam // Work `0.11` but weird firing ammo.
- [x] Oxidized Navigation: Tiled Runtime Nav-mesh generation for 3D worlds https://github.com/TheGrimsey/oxidized_navigation
- [x] Picking and Pointer Events for Bevy: https://github.com/aevyrie/bevy_mod_picking
- [ ] 💾 Moonshine Save: A save/load framework for Bevy game engine. https://github.com/Zeenobit/moonshine_save
- [ ] Bevy Replicon: Write the same logic that works for both multiplayer and single-player. https://github.com/lifescapegame/bevy_replicon

## Windows

> Use `dynamic`

```toml
[profile.dev]
opt-level = 1

[profile.dev.package."*"]
opt-level = 3

[dependencies]
bevy = {version = "0.11.3", features = ["dynamic"] }
```

---

## Mac intel

### Problem

> https://github.com/RustAudio/coreaudio-sys/issues/85

```
   Compiling coreaudio-sys v0.2.11
error: failed to run custom build command for `coreaudio-sys v0.2.11`

Caused by:
  process didn't exit successfully: `/Users/katopz/git/katopz/bevy-tower-defense-tutorial/target/debug/build/coreaudio-sys-8eda3e49206fc9e3/build-script-build` (exit status: 101)
  --- stdout
  cargo:rerun-if-env-changed=COREAUDIO_SDK_PATH
  cargo:rustc-link-lib=framework=AudioUnit
  cargo:rustc-link-lib=framework=CoreAudio
  cargo:rerun-if-env-changed=BINDGEN_EXTRA_CLANG_ARGS

  --- stderr
  thread 'main' panicked at /Users/katopz/.cargo/registry/src/index.crates.io-6f17d22bba15001f/proc-macro2-1.0.47/src/fallback.rs:756:9:
  "enum_(unnamed_at_/Library/Developer/CommandLineTools/SDKs/MacOSX_sdk/usr/include/MacTypes_h_382_1)" is not a valid Ident
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### Solution

```
brew install llvm@15
cargo update
```

---

### Problem

> https://github.com/michaeleisel/homebrew-zld/issues?q=is%3Aissue+is%3Aclosed

### Solution

Install `Xcode`
