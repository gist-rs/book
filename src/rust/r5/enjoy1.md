# Enjoy Day 1

![](/assets/kat.png) <span class="speech-bubble">Here's shortest way to learning `Rust`, Let's do it!</span>

> 💡 for more examples see 👉 [rust-by-example](https://doc.rust-lang.org/rust-by-example/index.html)

## Variables, println, assert_eq, for

```rust,editable
fn main() {
    // Define immutable variable.
    let count = 0;

    // {} mean param(0).
    println!("count = {}", count);

    // Define mutable variable.
    let mut count = 1;

    // So we can change it
    count += 1;

    // {0} mean param(0), {1:#?} mean param(1) with pretty print(#) for Debug(?).
    println!("{0} = {1:#?}", "count", count);

    // Let's make some condition.
    if count == 2 {
        // String literal {count} mean variable count for Display.
        println!("count = {count}");
    }

    // Assert that count is equal 2.
    assert_eq!(count, 2);

    // As base 16 hexadecimal by adding 👇.
    println!("count = {count} = 0x{count:x}");
}
```

> 💡 More `println` pattern 👉 [here](https://doc.rust-lang.org/rust-by-example/hello/print.html)

## for, while, loop, break

```rust,editable
fn main() {
    let mut count = 0;

    // This .. 👇 mean range i from 0 to 7.
    for _i in 0..8 { // _i mean we won't use i
        count += 1;
    }

    println!("count = {count}");

    // This .. 👇 mean range i from 0 to 8.
    for i in 0..=8 {
        count += i;
    }

    println!("count = {count}", count = count);

    // 👇 This is how we loop element (e).
    for e in ["a","b","c"] {
        println!("{e}");
    }

    //  👇 This is index (i) can be use by 👇 call enumerate fn.
    for (i, e) in ["a","b","c"].iter().enumerate() {
        println!("{i} = {e}");
    }

    // while
    while count < 50 {
        count += 1;
    }

    println!("count = {0}", count);

    // loop
    loop {
        count += 1;
        if count >= 100 {
            break;
        }
    }

    println!("count = {}", count);

    // loop and break
    'outer: loop {
        count += 1;

        // Break at 200
        if count >= 200 {
            // Never readch here.
            break;
        } else {
            // Inner loop
            loop {
                count += 1;
                // Because thi s break first.
                if count >= 150 {
                    break 'outer;
                }
            }
        }
    }

    println!("count = {}", count);
}
```

## fn, const, static, return, format

```rust,editable
// We previously use a lot of "count", let's DRY it as a constant.
const COUNT: &str = "count"; // Say hi to referenced string slice &str

// And maybe we want static footgun 💥 that can mutate.
static mut total: u32 = 0;

// Define "add" as a function
fn add(a: i32, b: i32) -> i32 {
    // i32 = integer 32
    a + b // This mean return a + b, hence no semicolon ;
}

fn main() {
    // We can use assert instead of assert_eq for test.
    assert!(add(1, 2) == 3);

    // Try use COUNT with format!
    let result = format!("{COUNT} = {}", add(1, 9));
    println!("{result}");

    // We will need unsafe to mutate static (fyi: bad practice).
    unsafe {
        // Try mutate and 👇 cast i32 to u32 (unsigned integer 32)
        total = add(3, 4) as u32;

        // Try assert_eq.
        assert_eq!(total, 7);
    }
}
```

> 💡 There's lot more [Primitives](https://doc.rust-lang.org/rust-by-example/primitives.html) we didn't cover here, feel free to take a look!

![](/assets/kat.png) We just use [unsafe](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html) mutate because `static mut` can be mutate and access by any function at anytime globally so that make sense to make as `unsafe`. But no worry! we won't do that again until we really need it.

## String, Vec, Tuple

### `String` ⚔️ `&str`

![](/assets/kat.png) We will need both `&str`and `String` for entire our `Rust` journey.
You will know when and which to use it later. Let's just use it for now.

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

    // 😱 You can try uncomment 👇 this to experience an error `value borrowed here after move`.
    // println!("foo_string:{foo_string}");

    // But if you really want to keep access `foo_string`.
    // Just don't move in the first place! See below👇

    // 1️⃣ let other borrow `&` instead of move.
    let borrowed_foo_string = &another_foo_string;
    println!("another_foo_string: {another_foo_string}"); // Still can access.
    println!("borrowed_foo_string: {borrowed_foo_string}"); // Also here.

    // 2️⃣ or make a clone/copy instead of move.
    let borrowed_foo_string = another_foo_string.clone();
    println!("another_foo_string: {another_foo_string}"); // Still can access.
    println!("borrowed_foo_string: {borrowed_foo_string}"); // Also here.
}
```

![](/assets/duck.png) Now we know why we need `&` to borrow some value from some variable instead of move.  
And we don't like to `clone` (or `copy`) that much because of more memory will need for doing that.

---

[Continue to Day 2 ➠](./enjoy2.md)
