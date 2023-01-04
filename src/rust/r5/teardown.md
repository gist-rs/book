# Tear down

## Recap

1. 🦀`Rust` like to move it move it 🎵, borrow `&` when need.
1. 🦀`Rust` variable are `immutable` by default, `mut` when need.
1. 🦀`Rust` variable will drop when out of scope `{ }`, consider borrow `&` when need.
1. No `null`, only `Option`, Use `match` (or [other ways](https://doc.rust-lang.org/rust-by-example/std/option.html)) to handle it; `unwrap` will `panic`.

   ```rust,no_run
   enum Option<T> {
      Some(T),
      None,
   }
   ```

1. When `fn` return `Result`, Use `match` (or [other ways](https://doc.rust-lang.org/rust-by-example/std/result.html)) to handle it; `unwrap` will `panic`.

   ```rust,no_run
   enum Result<T, E> {
      Ok(T),
      Err(E),
   }
   ```

1. Both `Option`, `Result` are `enum` so eat that frog 🐸!
1. Generic `T` and `E` nearly like generic in `TypeScript` so it should be easy there.
1. Use `iter`, `iter_into`, `collect` wisely, but no worry [`clippy`](https://doc.rust-lang.org/clippy/) will got your back anyway.
1. Choose `composition` over `inheritance`, learn to love `struct`, `impl`, `trait`, `derive` instead.
1. We `impl` (implement) some `trait` (aka skill) for `struct` so that `struct` can have that skill.
1. `String`,`Vec`,`Box` are smart pointer allocated on `heap`.
1. `str`, `array`, `struct`, and other primitives type are allocated on `stack`.
1. Compiler will ask to add `dyn` when need e.g. `Box`, `Supertraits`.
1. `Dynamic Dispatch` (Box) can be replace with `Static Dispatch` if need.
1. Don't over thinking! Do trust `clippy` and 🦀[`Rustacean`](https://rustacean-principles.netlify.app/) and you will be fine.
1. Did we forget `Some(thing)`? 🤔

---

![](/assets/kat.png) Easy right? We're not done yet. Let's dig deeper! 👉 [Continue to R4 ➠](../r4/mod.md)
