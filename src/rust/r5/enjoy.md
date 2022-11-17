# Enjoy

Frequency use gist for survival purpose, for more examples see 👉 https://doc.rust-lang.org/rust-by-example/index.html

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
    // String literal {count} mean variable count for Display.
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

## Function, Const

```rust,editable
  // We use a lot "count", let's DRY it as a constant.
  const COUNT: &str = "count"; // 😳 Say hi to referenced string slice &str

// "add" as a function
fn add(a:i32, b:i32) -> i32 { // 😳 i32 = integer 32
  a + b // 😳 This mean return a + b, hence no semicolon ;
}

fn main() {
  // Or better use assert_eq.
  assert!(add(1, 2) == 3);

  // Use const and fn
  let result = format!("{COUNT} = {}", add(1, 9));
  println!("{result}");
}
```

## Struct, Enum

```rust,editable
// No inheritance, do composition.
struct Animal {}
struct Cat {}
struct Dog {}

struct
fn main() {
  // TODO
}
```
