# Derive More

> 🤔 Refer to [The Newtype Pattern In Rust](https://www.worthe-it.co.za/blog/2020-10-31-newtype-pattern-in-rust.html)

## Do

- Implement `From` not `Into`.
- Use `TryFrom` for validation + `From`.
- Implementing `Newtype` Traits with `Derive More`.

```rust
use derive_more::{Add, Display, From, FromStr};
use std::str::FromStr;

#[derive(FromStr, Display)]
pub struct PhoneNumber(String);

#[derive(Clone, Copy, From, Display, Add)]
#[display(fmt = "{} years", _0)]
pub struct Years(u32);

fn main() {
    // Use parse.
    let num = "123-4567".parse::<PhoneNumber>().unwrap();
    println!("1️⃣ Phone number is {}", num);

    // Use from_str.
    let num = PhoneNumber::from_str("123-4567").unwrap();
    println!("2️⃣ Phone number is {}", num);

    // Use from and Add.
    let age_1 = Years::from(5);
    let age_2 = Years::from(2);
    println!("3️⃣ {} + {} = {}", age_1, age_2, age_1 + age_2);
}
```

<a href="https://codesandbox.io/p/sandbox/derive-more-852zqb" class="button">▢ CodeSandbox</a>
