# Deref coercions

> 🤔 Refer to [Rust Design Patterns](https://rust-unofficial.github.io/patterns/idioms/coercion-arguments.html)

## Do and Don't

- Use `&str` over `&String`.
- Use `&[T]` over `&Vec<T>`.
- Use `&T` over `&Box<T>`.

Because...

```rust,editable
// Do this
fn good(foo: &str) {
    println!("{foo}");
}

// Not this
fn bad(foo: &String) {
    println!("{foo}");
}

fn main() {
    // 🤩 Deref coercion happen here.
    println!("{:?}", good(&"Ferris".to_string())); // &String → &str
    println!("{:?}", good(&"Ferris"));             // &&str → &str
    println!("{:?}", good("Ferris"));              // &str → &str

    println!("{:?}", bad(&"Ferris".to_string()));  // &String → &String

    // 😱 No coercion here! Uncomment to see an errors.
    println!("{:?}", bad(&"Ferris")); // expected reference `&String` found reference `&&'static str`
    println!("{:?}", bad("Ferris"));  // expected reference `&String` found reference `&'static str`
}
```

![](/assets/kat.png) <span class="speech-bubble">That should show why `Deref coercions` is handy, without it we will have to do `&&str → &str` ourself and that's no joy. You can enjoy it now or continue reading to dig deeper. 👇</span>

## Deref

```rust,editable
use std::ops::Deref;

#[derive(Debug)]
struct Foo<T> {
    bar: T,
}

impl<T> Deref for Foo<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.bar
    }
}

fn main() {
    let foo = Foo { bar: "bar" };

    // Without Deref.
    println!("{:?}", foo.bar);

    // With Deref.
    println!("{:?}", *foo);

    // Same thing.
    assert_eq!(*foo, foo.bar);
}
```

## Deref auto coercions

```rust,editable
fn foo(bar: &str) {
    println!("{bar}");
}

fn main() {
    // Because String implements Deref<Target=str>.
    let owned_hello = "Hello".to_string();

    // So this work because &String → &str
    foo(&owned_hello);
    foo(&&owned_hello);
    foo(&*&owned_hello); // Why god why? Stop!

    // Or even this because &Rc<String> → &String → &str
    foo(&std::rc::Rc::new(owned_hello));
}
```

## Deref method

```rust,editable
struct Foo;

impl Foo {
    fn bar(&self) {
        println!("FooBar");
    }
}

fn main() {
    // Same thing.
    Foo.bar();
    (&Foo).bar();
    (&&Foo).bar();
    (&&&Foo).bar();
}
```

## More about `Deref`

> - [Official Rust Book](https://doc.rust-lang.org/book/ch15-02-deref.html)
> - [MIT (1st edition)](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/deref-coercions.html)
> - [Rust Reference](https://doc.rust-lang.org/reference/type-coercions.html)
> - [Rust RFC Book](https://rust-lang.github.io/rfcs/0241-deref-conversions.html)
