# Enjoy Day 1

![](/assets/kat.png) <span class="speech-bubble">Glad to see you here! Let's `Rust`!</span>

> 💡 for more examples see 👉 [rust-by-example](https://doc.rust-lang.org/rust-by-example/index.html)

## Variables, println, assert_eq

```rust,editable
fn main() {
    // Define immutable variable.
    let count = 0;

    // {} mean param_0.
    println!("1. count = {}", count);

    // Define mutable variable.
    let mut count = 1;

    // So we can change it.
    count += 1;

    // {0} mean param_0.
    // {1} mean param_1.
    //            ╭────────────────╮
    println!("2. {0} = {1:#?}", "count", count);
    //                  ╰──────────────────╯
    // # mean pretty print.
    // ? mean debug.

    // Let's make some condition.
    if count == 2 {
        // String literal {count} mean variable count for Display.
        println!("3. count = {count}");
    }

    // Assert that count is equal 2.
    assert_eq!(count, 2);

    // As base 16 hexadecimal by adding    👇.
    println!("4. count = {count} = 0x{count:x}");
}
```

> 💡 More `println` pattern 👉 [here](https://doc.rust-lang.org/rust-by-example/hello/print.html)

## for, while, loop, break

```rust,editable
fn main() {
    //  👇 Mutable so we change the value later.
    let mut count = 0;

    // This .. 👇 mean range i from 0 to 7.
    for _i in 0..8 { // _i mean we won't use i
        count += 1;
    }

    println!("1. count = {count}");

    // This .. 👇 mean range i from 0 to 8.
    for i in 0..=8 {
        count += i;
    }

    println!("2. count = {count}", count = count);

    // 👇 This is how we loop element (e).
    for e in ["a","b","c"] {
        println!("3. {e}");
    }

    //  👇 This is index (i) can be use by 👇 call enumerate fn.
    for (i, e) in ["a","b","c"].iter().enumerate() {
        println!("4. {i} = {e}");
    }

    // while
    while count < 50 {
        count += 1;
    }

    println!("5. count = {0}", count);

    // loop
    loop {
        count += 1;
        if count >= 100 {
            break;
        }
    }

    println!("6. count = {}", count);

    // loop and break
    'outer: loop {
        count += 1;

        // Break at 200
        if count >= 200 {
            // Never reach here because 👇.
            break;
        } else {
            // Inner loop
            loop {
                count += 1;
                // Because this will break first.
                if count >= 150 {
                    break 'outer;
                }
            }
        }
    }

    println!("7. count = {}", count);
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
    println!("1. {result}");

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

![](/assets/kat.png) <span class="speech-bubble">We (rarely) use [unsafe](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html) mutate because `static mut` can be mutate and access by any function at anytime globally so that make sense to make as `unsafe`.</span>

![](/assets/duck.png) <span class="speech-bubble">No worry! we won't do `unsafe` things anytime soon.</span>

## String, &str

![](/assets/kat.png) <span class="speech-bubble">We will need both `&str`and `String` for entire our `Rust` journey.
You will know the other [fancy type of string](https://www.youtube.com/watch?v=CpvzeyzgQdw) in Rust later.</span>

### Without reference

- `str` = string slice, <u>immutable</u> view into a sequence of UTF-8 bytes, reference to a portion of a string or an array of bytes and has no ownership.
- `String` = owned, heap-allocated string, <u>mutable</u>, growable, and dynamic string type that you can modify at runtime.

### With reference &

- `&str` = reference to a `str`.
- `&String` = reference to a `String`.

```rust,editable
fn main() {
    // Start with str
    let foo_str = "foo"; // &str 👈 Reference to a string slice.

    // Try move str
    let bar_str = foo_str;
    println!("1. bar_str: {bar_str}");
    println!("2. foo_str: {foo_str}");

    // Now let's try String
    let foo_string = foo_str.to_string(); // String 👈 So we can move it.

    // Try move String.
    let bar_string = foo_string;
    println!("3. bar_string: {bar_string}");

    // But foo_string is already moved. 💀
    // 😱 You can try uncomment 👇 this to see an error.
    // println!("foo_string:{foo_string}");
    //                      ^^^^^^^^^^^^ value borrowed here after move

    // So we need & to make a reference.
    // 1️⃣ let other borrow `&` instead of move.
    let borrowed_bar_string = &bar_string;
    println!("4. bar_string: {bar_string}"); // Still can access.
    println!("5. borrowed_bar_string: {borrowed_bar_string}"); // Also here.

    // 2️⃣ or make a clone/copy instead of move.
    let borrowed_bar_string = bar_string.clone();
    println!("6. bar_string: {bar_string}"); // Still can access.
    println!("7. borrowed_bar_string: {borrowed_bar_string}"); // Also here.
}
```

![](/assets/kat.png) <span class="speech-bubble">So we need `&` to borrow the instead of moving it.
Anyway we tend to avoid `clone`/`copy` to reduce overhead aka [zero-copy](https://swatinem.de/blog/magic-zerocopy) as possible.</span>

![](/assets/duck.png) <span class="speech-bubble">Oh, We also can convert between `&str` and `String`.</span>

```rust,editable
fn main() {
    // String → &str
    let bar_str = bar_string.as_str();

    println!("bar_string: {bar_string}");
    println!("bar_str: {bar_str}");

    // &str → String
    assert_eq!(bar_string, foo_str.to_string());
}
```

![](/assets/kat.png) <span class="speech-bubble">One more thing!</span>

- `'static str` = `str` with a `'static` lifetime. It's a reference to a string that is valid for the entire duration of the program (which is both good and bad).

![](/assets/duck.png) <span class="speech-bubble">It's `OK` to confuse about `str`, `String` and `&str`, but no worries! we will get use to it in the end.</span>

[Continue to Day 2 ➠](./enjoy2.md)
