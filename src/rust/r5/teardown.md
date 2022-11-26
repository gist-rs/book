# Tear down

## Recap

1. 🦀`Rust` like to move it move it 🎵, borrow `&` when need.
1. 🦀`Rust` variable is `immutable` by default, `mut` when need.
1. 🦀`Rust` variable will drop when out of scope `{ }`, consider borrow `&` when need.
1. No `null`, only `Option`, Use `match` (or [other ways](https://doc.rust-lang.org/rust-by-example/std/option.html)) to handle it `unwrap` will panic.

   ```rust
   Option<T>
   ├─Some(T)
   └─None
   ```

1. When `fn` return `Result`, Use `match` (or [other ways](https://doc.rust-lang.org/rust-by-example/std/result.html)) to handle it `unwrap` will panic.

   ```rust,
   Result<T, E>
   ├─Ok(T)
   └─Err(E)
   ```

1. Both `Option`, `Result` is `enum` so eat that frog 🐸!
1. Use `iter`, `iter_into`, `collect` wisely, but no worry [`clippy`](https://doc.rust-lang.org/clippy/) will got your back anyway.
1. Choose `composition` over `inheritance`, learn to love `struct`, `impl`, `trait`, `derive` instead.
1. `String`,`Vec`,`Box` is smart pointer allocated on `heap` and ref to stack below.
1. `str`, `array`, `struct`, and other primitives type is allocated on `stack`.
1. `dyn` take case unsure on some type (`Box`, `Supertraits`) for us.
1. Don't over thinking, trust compiler, `clippy` and 🦀[`Rustacean`](https://rustacean-principles.netlify.app/), you will be fine.
1. Did we forget `Some(thing)`? 🤔

---

![](/assets/kat.png) Easy right? Let's go deeper! We're not done yet 👉 [Continue to R 4 ➠](../r4/mod.md)
