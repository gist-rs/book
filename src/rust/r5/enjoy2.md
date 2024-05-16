# Enjoy Day 2

![](/assets/kat.png) <span class="speech-bubble">We will need `vec` and `array` to keep things organized as index.</span>

### Vec, iter, map, collect, into_iter, enumerate, unwrap, Tuple, HashSet

```rust,editable
fn main() {
    // Create new `array` of `&str` and `vec`.
    let array_of_foo = ["foo", "bar"]; // Array of &str.
    let mut vec_of_foo = vec!["foo", "bar"]; // Say hi to vec! macro.

    println!("array_of_foo: {array_of_foo:#?}");
    println!("vec_of_foo: {vec_of_foo:#?}");

    // The different?
    vec_of_foo.push("baz"); // You can push more to Vec

    // 😱 Uncomment to see an error "no method named `push` found for array `[&str; 2]`".
    // FYI: `[&str; 2]` mean fixed array of &str usize 2.
    // 👍 Anyway fixed size is good for memory management, so don't hate it!
    // array_of_foo.push("baz"); // You can't to fixed Array [&str; 2]

    // 1️⃣ Back to Vec, Let's iterate them.
    let hello_vec = vec_of_foo
        .iter() // Must `iter()` before you can map, filter,...
        .map(|e| format!("hello {e}")) // Say hi to `closure` |e| aka (e)=> in js.
        .collect::<Vec<_>>(); // `collect` inferred type from iterate.
        //             👆 `_` is inferred type (let compiler desire).

    println!("hello_vec: {hello_vec:#?}");

    // 2️⃣ Do it again but with index.
    let indexed_vec = vec_of_foo
        .iter()
        .enumerate() // To access index we need `enumerate`.
        .map(|(i, e)| (i, e)) // Say hi to `Tuple` type.
        .collect::<Vec<(usize, &&str)>>(); // i is `usize`, e is &&str.

    println!("indexed_vec: {indexed_vec:#?}");

    // 3️⃣ Do it again but `into_iter`.
    let into_iter_indexed_vec = vec_of_foo
        .into_iter() // `into_iter` instead of `iter` for `deref` (Wait what?).
        .enumerate()
        .map(|(i, e)| (i, e))
        .collect::<Vec<(usize, &str)>>(); // e is just &str not &&str.
                                          // Or just `<Vec<_>>` if you lazy.

    println!("into_iter_indexed_vec: {into_iter_indexed_vec:#?}");

    // `into_iter` is handy to pass value without borrow
    // but it can be problematic sometime if it has been borrowed by 1️⃣ and 2️⃣.

    // 😱 Uncomment this to see an error.
    // assert_eq!(
    //     indexed_vec.first().unwrap().1,
    //     &into_iter_indexed_vec.first().unwrap().1
    // );

    // You can also define type 👇 here
    use std::collections::HashSet;
    let binding = vec!["foo", "bar"];
    let iter_hashset: HashSet<_> = binding
        .iter()
        .map(|e|e)
        .collect();

    println!("iter_hashset: {iter_hashset:#?}");

    // Or even shorter with 👇 FromIterator
    use std::iter::FromIterator;
    let binding = vec!["foo", "bar"];
    let hashset_from: HashSet<_> = HashSet::from_iter(binding);

    println!("hashset_from: {:#?}", hashset_from);
}
```

> 💡 `::<>` is [turbo fish](https://turbo.fish/) <span style="transform: scaleX(-1);"><span>🐟💨</span></span> .
> 💡 Read more about `iterate` [here](https://doc.rust-lang.org/rust-by-example/trait/iter.html).
> 💡 If you crazy about `iterate` do try [Rust Iterator Cheat Sheet](https://danielkeep.github.io/itercheat_baked.html)

![](/assets/kat.png) <span class="speech-bubble">So `iter` will make an auto borrow `&` for us which is handy.
And we can use `into_iter` if we need.</span>

![](/assets/duck.png) <span class="speech-bubble">Also `<Vec<_>>` is for lazy crab like us, nice!</span>

### Option, Some, None, use, HashMap, match, expect, unwrap_or, panic

![](/assets/kat.png) <span class="speech-bubble">`HashMap` is like Key/Value pair.</span>

```rust,editable
use std::collections::HashMap; // `use` aka `import` in js.
// We talk about :: 👆 already, it's just a separator.

fn main() {
    // Create new mutable hashmap
    let mut foo_hashmap = HashMap::new(); // Yet another :: here.

    // It's mutable so we can update it
    foo_hashmap.insert("name", "foo");
    foo_hashmap.insert("age", "42");

    // Or rather use HashMap::from for batch insert.
    let mut foo_hashmap = HashMap::from([
        ("name", "foo"),
        ("age", "42"),
    ]);

    // 1️⃣ And when we tend to throw an error if not exist.
    let name_or_error = foo_hashmap.get("name").expect("Expect name");        // Will return &&str
    let name_or_error = foo_hashmap.get::<str>("name").expect("Expect name"); // Will return &&str

    println!("1️⃣ name_or_error:{name_or_error:?}");

    // 2️⃣ Now use it in varies style.
    let maybe_name = foo_hashmap.get("name");          // Will return `Option<&&str>`.
    let maybe_name = foo_hashmap.get("name").copied(); // Will return `Option<&str>`.
    let maybe_name = foo_hashmap.get::<str>("name");   // Will return `Option<&&str>`.

    // `match` aka `switch` in js.
    // Let's handle `Option<&&str>` which can be `Some` or `None`.
    match maybe_name {
        Some(name) => println!("2️⃣ hello {name}"), // Will print "hello foo".
        None => panic!("who!?"),                   // Will throw error with `panic!` macro.
    };

    // 3️⃣ Or handle with `unwrap_or`.
    let unwrapped_name = maybe_name.unwrap_or(&"who!?");

    // And assign back by return after matched.
    let hi = match unwrapped_name {
        &"foo" => format!("3️⃣ unwrapped_name:{unwrapped_name}"), // Will return unwrapped_name.
        _ => panic!("who!?"),                                    // `_` aka `default` in js.
    };

    println!("{hi}");

    // 4️⃣ Let's iterate and print it out.
    foo_hashmap
        .iter()                               // iter as usual, will use `for_each`.
        .for_each(|e| println!("4️⃣ {:?}", e)); // Just print, No need to collect.

    // 5️⃣ Then we will use get👇 to borrow the value.
    let name = foo_hashmap.get("name").unwrap();
    println!("5️⃣ unwrap_name:{name}");

    // 6️⃣ Or take it by remove 👇.
    let age = foo_hashmap.remove("age").unwrap();
    println!("6️⃣ remove_age:{age}");

    // 😱 So this will fail because we already remove it above.
    // let age = foo_hashmap.remove("age").unwrap();
}
```

![](/assets/kat.png) <span class="speech-bubble">Recap: `unwrap`, `expect`, `unwrap_or`</span>

> 💡 `T` is generic Type, `None` is nothing, `unwrap` always has 2 outputs.

```
                         ╭─ Some(T) ──▶︎ T
Option<T> ── unwrap ─────┤
                         ╰─ None ─────▶︎ 💥 Panic
```

```
                         ╭─ Some(T) ──▶︎ T
Option<T> ── expect ─────┤
                         ╰─ None ─────▶︎ 💥 Panic with reason.
```

```
                         ╭─ Some(T) ──▶︎ T
Option<T> ── unwrap_or ──┤
                         ╰─ None ─────▶︎ Some fallback value you desired.
```

![](/assets/kat.png) <span class="speech-bubble">To know more about this try [read more](https://doc.rust-lang.org/rust-by-example/error/option_unwrap.html).</span>

![](/assets/duck.png) <span class="speech-bubble">We tend to avoid `unwrap` and use `unwrap_or` or `expect` instead.</span>

### Result, Ok, Err, SystemTime

```rust,editable
use std::time::{SystemTime, UNIX_EPOCH, Duration};

fn main() {
    // Get current system time.
    let now = SystemTime::now();

    // And it will return a Result.
    let duration_since_result = now.duration_since(UNIX_EPOCH);
    println!("duration_since_result:{:?}", duration_since_result);

    // 1️⃣ We can unwrap it to get inner value. 😃
    let duration_since:Duration = duration_since_result.unwrap();
    println!("1️⃣ duration_since:{:?}", duration_since);

    // 2️⃣ Or use match to handle Result<Ok(Duration),Err(())>
    let duration = match now.duration_since(UNIX_EPOCH) {
        // Handle happy case.
        Ok(duration) => duration,

        // Handle error case.
        Err(err) => panic!("{:?}", err),
    };
    println!("2️⃣ duration:{:?}", duration);

    // But what if result is error? 😱
    let duration_since_result = Err(());

    // // 💥 👇 It will panic and crash with no reason. 😭
    // let duration_since:Duration = duration_since_result.unwrap();
    // println!("💥 duration_since:{:?}", duration_since);

    // 3️⃣ You can fallback with unwrap_or.
    let duration:Duration = duration_since_result.unwrap_or(Duration::new(0u64,0u32));
    println!("3️⃣ duration:{:?}", duration);

    // 4️⃣ Or panic with a reason, we will use `expect` instead. 🫣
    let _duration:Duration = duration_since_result.expect("4️⃣ 🔥 Expect some number.");
}
```

> 💡 `T` is generic Type, `E` is generic Error, `unwrap` always has 2 outputs.

```
                            ╭─ Ok(T) ──▶︎ T
Result<T, E> ── unwrap ─────┤
                            ╰─ Err(E) ─▶︎ 💥 Panic
```

```
                            ╭─ Ok(T) ──▶︎ T
Result<T, E> ── expect ─────┤
                            ╰─ Err(E) ─▶︎ 💥 Panic with reason.
```

```
                            ╭─ Ok(T) ──▶︎ T
Result<T, E> ── unwrap_or ──┤
                            ╰─ Err(E) ─▶︎ Some fallback value you desired.
```

> Read more about how to handle `Result` [here](https://doc.rust-lang.org/rust-by-example/error/result.html)

![](/assets/duck.png) <span class="speech-bubble">Don't be surprise if you found this confusing. I did! But don't give up just yet!</span>

![](/assets/kat.png) <span class="speech-bubble">We will use `Option` and `Result` a lot from now on, so deal with it!</span>

[Continue to Day 3 ➠](./enjoy3.md)
