# Enjoy

Frequency use gist for survival purpose, for more examples see 👉 [rust-by-example](https://doc.rust-lang.org/rust-by-example/index.html)

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
// We use a lot of "count", let's DRY it as a constant.
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

## String, Vec, Tuple, HashMap

### 1️⃣ String

> 💡 We will need both `&str`and `String` for entire our `Rust` journey.
> You will know when and which to use it later. Just use it for now.

```rust,editable
fn main() {

  // Convert &str to String
  let foo_str = "foo";                    // &str
  let foo_string = foo_str.to_string();   // String

  println!("foo_str: {foo_str}");
  println!("foo_string:  {foo_string}");

  // Convert String to &str
  let another_foo_string = foo_string;    // move foo_string to another_foo_string. 👋
  let another_foo_str = another_foo_string.as_str();

  println!("another_foo_string: {another_foo_string}");
  println!("another_foo_str: {another_foo_str}");

  assert_eq!(another_foo_str, another_foo_str);
  assert_eq!(another_foo_string, foo_str.to_string());

  // 😱 You can try uncomment this to experience an error `value borrowed here after move`.
  // 💡 It's has been move 👋 = you can't use it = memory are now free = good.
  // println!("foo_string:{foo_string}");
}
```

### 2️⃣ Vec, Iter

> 💡 Vec = Vector = nearly same as Array in js.

```rust,editable
fn main() {
  // Let's keep things in order.
  let vec_of_foo = vec!("foo", "bar");      // 😳 Say hi to vec! macro.

  println!("vec_of_foo: {vec_of_foo:#?}");

  // And iterate them
  let hello_vec = vec_of_foo
    .iter()                         // 😳 Must `iter()` before you can map, filter,...
    .map(|e|format!("hello {e}"))   // 😳 Say hi to `closure` |e| aka (e)=> in js
    .collect::<Vec<_>>();           // 😳 `collect` any result from iterate. *️⃣

  println!("hello_vec: {hello_vec:#?}");

  // Do it again but with index
  let indexed_vec = vec_of_foo
    .iter()
    .enumerate()                       // 😳 To access index we need `enumerate`.
    .map(|(i, e)|(i, e))               // 😳 Say hi to `Tuple` type.
    .collect::<Vec<(usize, &&str)>>(); // 😳 i is `usize`, e is &&str.

    println!("indexed_vec: {indexed_vec:#?}");

  // Do it again but with index
  let indexed_vec = vec_of_foo
    .into_iter()                       // 😳 `into_iter` instead of `iter` for `deref` (Wait what?).
    .enumerate()
    .map(|(i, e)|(i, e))
    .collect::<Vec<(usize, &str)>>();  // 😳 i is `usize`, e is &str.
  //.collect::<Vec<_>>();              // Or just `<Vec<_>>` if you lazy.

  println!("indexed_vec: {indexed_vec:#?}");
}
```

<details>
  <summary>*️⃣ collect</summary>

> 💡 `.collect::<Vec<_>>()`  
> ✨ `::` use as separator.  
> ✨ `<SomeType>` is like we use type in ts.  
> ✨ `_` mean any type.
>
> So `collect::<Vec<_>>` mean "just collect whatever iterate return".

</details>

### 3️⃣

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
