# Enjoy

## Variables

```rust,editable
fn main() {
  // Define immutable variable.
  let count = 0;

  // {:#?} mean format pretty for Debug.
  println!("count = {:#?}", count);

  // Define mutable variable.
  let mut count = 1;

   // {0} mean first param for Display.
  println!("{0} = {1}", "count", count);

  // So we can change it
  count = count + 1;

  // Let's make some condition.
  if count == 2 {
    // {count} mean variable count for Display.
    println!("count = {count}");
  }

  // Then loop.
  for i in 0..8 {
    count = count + 1;
  }

  // Assert that count is 10.
  assert_eq!(count, 10);

  // As base 16 hexadecimal.
  println!("count = {count} = 0x{count:x}");
}
```

> 💡 More `println` pattern here 👉 https://doc.rust-lang.org/rust-by-example/hello/print.html

##
