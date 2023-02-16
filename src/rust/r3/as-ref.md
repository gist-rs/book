# AsRef

> 🤔 [refer to Airat Galiullin](https://twitter.com/galiullin_airat/status/1625938494634827776)

```rust
// This 👇 did the trick.
fn hello<T: AsRef<str>>(planet: T) {
    println!("Hello, {}!", planet.as_ref());
}

fn main() {
    // String
    hello("Earth".to_string());

    // &str
    hello("Mars");
}
```
