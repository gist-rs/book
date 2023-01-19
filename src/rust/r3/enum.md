# Enum

## Beware!

```rust,editable
enum Foo {
    Bar = 43114
}

fn main() {
    // Happy case
    println!("{:?}", Foo::Bar as u64); // 43114
    println!("{:?}", 43114 as u64);    // 43114

    // BEWARE! This get 106 not 43114 nor error.
    println!("{:?}", Foo::Bar as u8);  // 106 😱

    // This will throw an error.
    // println!("{:?}", 43114 as u8);
}
```
