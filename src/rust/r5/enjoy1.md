# Enjoy Day 1

![](/assets/kat.png) Here's shortest way to learning `Rust`, Let's do it!

> 💡 for more examples see 👉 [rust-by-example](https://doc.rust-lang.org/rust-by-example/index.html)

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
    count += 1;

    // Let's make some condition.
    if count == 2 {
        // String literal {count} mean variable count for Display.
        println!("count = {count}");
    }

    // This .. 👇 here's mean range i from 0 to 7.
    for _i in 0..8 { // 😳 _i mean we won't use i
        count += 1;
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
// We use a lot of "count", let's DRY it as a constant.
const COUNT: &str = "count"; // 😳 Say hi to referenced string slice &str

// "add" as a function
fn add(a: i32, b: i32) -> i32 {
    // 😳 i32 = integer 32
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

### Vec, iter, map, collect, into_iter, enumerate

![](/assets/kat.png) We will need `vec` and `array` to keep values organized as index.

```rust,editable
fn main() {
    // Create new `vec` and `array` of `&str`.
    let mut vec_of_foo = vec!["foo", "bar"]; // 😳 Say hi to vec! macro.
    let array_of_foo = ["foo", "bar"]; // Array of &str.

    println!("vec_of_foo: {vec_of_foo:#?}");
    println!("array_of_foo: {array_of_foo:#?}");

    // The different?
    vec_of_foo.push("baz"); // You can push more to Vec

    // 😱 Uncomment to see an error "no method named `push` found for array `[&str; 2]`".
    // FYI: `[&str; 2]` mean fixed array of &str usize 2.
    // 👍 Anyway fixed size is actually good for memory management, don't hate it!
    // array_of_foo.push("baz"); // 😳 You can't to fixed Array [&str; 2]

    // 1️⃣ Back to Vec, Let's iterate them.
    let hello_vec = vec_of_foo
        .iter() // 😳 Must `iter()` before you can map, filter,...
        .map(|e| format!("hello {e}")) // 😳 Say hi to `closure` |e| aka (e)=> in js.
        .collect::<Vec<_>>(); // 😳 `collect` any result from iterate.
        //             👆 `_` mean any.

    println!("hello_vec: {hello_vec:#?}");

    // 2️⃣ Do it again but with index.
    let indexed_vec = vec_of_foo
        .iter()
        .enumerate() // 😳 To access index we need `enumerate`.
        .map(|(i, e)| (i, e)) // 😳 Say hi to `Tuple` type.
        .collect::<Vec<(usize, &&str)>>(); // 😳 i is `usize`, e is &&str.

    println!("indexed_vec: {indexed_vec:#?}");

    // 3️⃣ Do it again but `into_iter`.
    let into_iter_indexed_vec = vec_of_foo
        .into_iter() // 😳 `into_iter` instead of `iter` for `deref` (Wait what?).
        .enumerate()
        .map(|(i, e)| (i, e))
        .collect::<Vec<(usize, &str)>>(); // 😳 e is just &str not &&str.
                                          // Or just `<Vec<_>>` if you lazy.

    println!("into_iter_indexed_vec: {into_iter_indexed_vec:#?}");

    // `into_iter` is handy to pass value without borrow
    // but it can be problematic sometime if it has been borrowed by 1️⃣ and 2️⃣.

    // 😱 Uncomment this to see an error.
    // assert_eq!(
    //     indexed_vec.first().unwrap().1,
    //     &into_iter_indexed_vec.first().unwrap().1
    // );
}
```

![](/assets/duck.png) So `iter` will make an auto borrow `&` for us which is handy.  
Anyway we can use `into_iter` if we need to move instead of borrow with caution.  
And also `<Vec<_>>` is for lazy crab like us, nice!

> 🏂 Fun fact!  
> `String` and `Vec` is on `heap`.  
> `str` and `array` is on `stack`.

### HashMap, match, Some, None

![](/assets/kat.png) `HashMap` is like Key/Value pair.

```rust,editable
use std::collections::HashMap; // 😳 `use` aka `import` in js.
// We talk about :: 👆 already, it's just a separator.

fn main() {
    // Create new mutable hashmap
    let mut foo_hashmap = HashMap::new(); // 😳 Yet another :: here.

    // It's mutable so we can update it
    foo_hashmap.insert("name", "foo");
    foo_hashmap.insert("age", "42");

    // Now use it
    let maybe_name: Option<&&str> = foo_hashmap.get("name"); // 😳 Will return `Option<&&str>`

    // 😳 `match` aka `switch` in js.
    // Let's handle `Option<&&str>` which can be `Some` or `None`.
    match maybe_name {
        Some(name) => println!("1️⃣ hello {name}"), // 😳 Will print "hello foo".
        None => panic!("who!?"),                   // 😳 Will throw error with `panic!` macro.
    };

    // Or handle with `unwrap_or`.
    let unwrapped_name = maybe_name.unwrap_or(&"who!?");

    // And assign back by return after matched.
    let hi = match unwrapped_name {
        &"foo" => format!("2️⃣ hi! {unwrapped_name}"), // 😳 Will return unwrapped_name.
        _ => panic!("who!?"),                         // 😳 `_` aka `default` in js.
    };

    println!("{hi}");

    // Let's iterate and print it out.
    foo_hashmap
        .iter()                             // iter as usual, will use `for_each`.
        .for_each(|e| println!("{:?}", e)); // 😳 Just print, No need to collect.
}
```

> 💡 This will take you sometime to get used to `Option<T>`→ `Some<T>`/`None`.  
> To read more about this try [read more](https://doc.rust-lang.org/rust-by-example/error/option_unwrap.html).

![](/assets/duck.png) Don't be surprise if you found all this confusing. I did! But don't give up just yet!

[Continue to Day 2 ➠](./enjoy2.md)
