# Derive More

> 🤔 Refer to [The Newtype Pattern In Rust](https://www.worthe-it.co.za/blog/2020-10-31-newtype-pattern-in-rust.html)

## Do

- Implement `From` not `Into`.
- Use `TryFrom` for validation + `From`.
- Implementing `Newtype` Traits with `Derive More`.

## Setup

```bash
cargo add derive_more --features "add display from from_str"
```

## Code

<tabs>
<tab label="main.rs">

```rust,edition2021
{{#include ../../../examples/r3/hello-derive-more/src/main.rs}}
```

</tab>
<tab label="Cargo.toml">

```toml
{{#include ../../../examples/r3/hello-derive-more/Cargo.toml}}
```

</tab>
</tabs>

## Output

```
1️⃣ Phone number is 123-4567
2️⃣ Phone number is 123-4567
3️⃣ 5 years + 2 years = 7 years
```
