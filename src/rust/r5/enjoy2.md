# Enjoy Day 2

![](/assets/kat.png) We will need `vec` and `array` to keep things organized as index.

### Vec, iter, map, collect, into_iter, enumerate

```rust,editable
fn main() {
    // Create new `vec` and `array` of `&str`.
    let mut vec_of_foo = vec!["foo", "bar"]; // ✨ Say hi to vec! macro.
    let array_of_foo = ["foo", "bar"]; // Array of &str.

    println!("vec_of_foo: {vec_of_foo:#?}");
    println!("array_of_foo: {array_of_foo:#?}");

    // The different?
    vec_of_foo.push("baz"); // You can push more to Vec

    // 😱 Uncomment to see an error "no method named `push` found for array `[&str; 2]`".
    // FYI: `[&str; 2]` mean fixed array of &str usize 2.
    // 👍 Anyway fixed size is actually good for memory management, don't hate it!
    // array_of_foo.push("baz"); // ✨ You can't to fixed Array [&str; 2]

    // 1️⃣ Back to Vec, Let's iterate them.
    let hello_vec = vec_of_foo
        .iter() // ✨ Must `iter()` before you can map, filter,...
        .map(|e| format!("hello {e}")) // ✨ Say hi to `closure` |e| aka (e)=> in js.
        .collect::<Vec<_>>(); // ✨ `collect` any result from iterate.
        //             👆 `_` mean any.

    println!("hello_vec: {hello_vec:#?}");

    // 2️⃣ Do it again but with index.
    let indexed_vec = vec_of_foo
        .iter()
        .enumerate() // ✨ To access index we need `enumerate`.
        .map(|(i, e)| (i, e)) // ✨ Say hi to `Tuple` type.
        .collect::<Vec<(usize, &&str)>>(); // ✨ i is `usize`, e is &&str.

    println!("indexed_vec: {indexed_vec:#?}");

    // 3️⃣ Do it again but `into_iter`.
    let into_iter_indexed_vec = vec_of_foo
        .into_iter() // ✨ `into_iter` instead of `iter` for `deref` (Wait what?).
        .enumerate()
        .map(|(i, e)| (i, e))
        .collect::<Vec<(usize, &str)>>(); // ✨ e is just &str not &&str.
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
use std::collections::HashMap; // ✨ `use` aka `import` in js.
// We talk about :: 👆 already, it's just a separator.

fn main() {
    // Create new mutable hashmap
    let mut foo_hashmap = HashMap::new(); // ✨ Yet another :: here.

    // It's mutable so we can update it
    foo_hashmap.insert("name", "foo");
    foo_hashmap.insert("age", "42");

    // Now use it
    let maybe_name: Option<&&str> = foo_hashmap.get("name"); // ✨ Will return `Option<&&str>`

    // ✨ `match` aka `switch` in js.
    // Let's handle `Option<&&str>` which can be `Some` or `None`.
    match maybe_name {
        Some(name) => println!("1️⃣ hello {name}"), // ✨ Will print "hello foo".
        None => panic!("who!?"),                   // ✨ Will throw error with `panic!` macro.
    };

    // Or handle with `unwrap_or`.
    let unwrapped_name = maybe_name.unwrap_or(&"who!?");

    // And assign back by return after matched.
    let hi = match unwrapped_name {
        &"foo" => format!("2️⃣ hi! {unwrapped_name}"), // ✨ Will return unwrapped_name.
        _ => panic!("who!?"),                         // ✨ `_` aka `default` in js.
    };

    println!("{hi}");

    // Let's iterate and print it out.
    foo_hashmap
        .iter()                             // iter as usual, will use `for_each`.
        .for_each(|e| println!("{:?}", e)); // ✨ Just print, No need to collect.
}
```

> 💡 This will take you sometime to get used to `Option<T>`→ `Some<T>`/`None`.  
> To read more about this try [read more](https://doc.rust-lang.org/rust-by-example/error/option_unwrap.html).

![](/assets/duck.png) Don't be surprise if you found all this confusing. I did! But don't give up just yet!

[Continue to Day 3 ➠](./enjoy3.md)
