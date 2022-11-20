# Enjoy Day 1

![](/assets/kat.png) Here's shortest way to learning `Rust`, Let's do it!

> 💡 for more examples see 👉 [rust-by-example](https://doc.rust-lang.org/rust-by-example/index.html)

## Variables, println, assert_eq

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
    count += 1;

    // Let's make some condition.
    if count == 2 {
        // String literal {count} mean variable count for Display.
        println!("count = {count}");
    }

    // This .. 👇 here's mean range i from 0 to 7.
    for _i in 0..8 { // ✨ _i mean we won't use i
        count += 1;
    }

    // Assert that count is 10.
    assert_eq!(count, 10);

    // As base 16 hexadecimal.
    println!("count = {count} = 0x{count:x}");
}
```

> 💡 More `println` pattern here 👉 https://doc.rust-lang.org/rust-by-example/hello/print.html

## fn, const, return, format

```rust,editable
// We use a lot of "count", let's DRY it as a constant.
const COUNT: &str = "count"; // ✨ Say hi to referenced string slice &str

// "add" as a function
fn add(a: i32, b: i32) -> i32 {
    // ✨ i32 = integer 32
    a + b // ✨ This mean return a + b, hence no semicolon ;
}

fn main() {
    // Or better use assert_eq.
    assert!(add(1, 2) == 3);

    // Use const and fn
    let result = format!("{COUNT} = {}", add(1, 9));
    println!("{result}");
}
```

> 💡 There's lot more [Primitives](https://doc.rust-lang.org/rust-by-example/primitives.html) we didn't cover here, feel free to take a look!

## String, Vec, Tuple, HashMap

### `String` ⚔️ `&str`

![](/assets/kat.png) We will need both `&str`and `String` for entire our `Rust` journey.
You will know when and which to use it later. Just use it for now.

```rust,editable
fn main() {
    // Convert &str to String
    let foo_str = "foo"; // &str
    let foo_string = foo_str.to_string(); // String

    println!("foo_str: {foo_str}");
    println!("foo_string:  {foo_string}");

    // Convert String to &str
    let another_foo_string = foo_string; // Move foo_string to another_foo_string. 👋
    let another_foo_str = another_foo_string.as_str();

    println!("another_foo_string: {another_foo_string}");
    println!("another_foo_str: {another_foo_str}");

    assert_eq!(another_foo_string, foo_str.to_string());

    // 😱 You can try uncomment this to experience an error `value borrowed here after move`.
    // 💡 It's has been move 👋 = you can't use it = memory are now free = good.

    // println!("foo_string:{foo_string}");

    // But if you really want to keep access `foo_string` how?
    // Just don't move at the first place! 👇

    // 1. let other borrow `&` instead of move.
    let borrowed_foo_string = &another_foo_string;
    println!("another_foo_string: {another_foo_string}"); // Still can access.
    println!("borrowed_foo_string: {borrowed_foo_string}"); // Also here.

    // 2. or make a clone/copy instead of move.
    let borrowed_foo_string = another_foo_string.clone();
    println!("another_foo_string: {another_foo_string}"); // Still can access.
    println!("borrowed_foo_string: {borrowed_foo_string}"); // Also here.
}
```

![](/assets/duck.png) Now we know why we need `&` to borrow some value from some variable instead of move.  
And we don't like to `clone` (or `copy`) that much because of more memory will need for doing that.

[Continue to Day 2 ➠](./enjoy2.md)
