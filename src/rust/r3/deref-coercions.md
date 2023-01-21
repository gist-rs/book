# Deref coercions

> 🤔 [refer to ](https://twitter.com/wcrichton/status/1597318042244915201)

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
    let foo = Foo { bar: 'a' };

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

## Read more

- Official: https://doc.rust-lang.org/book/ch15-02-deref.html
- MIT (1st edition): https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/deref-coercions.html
- Rust Reference: https://doc.rust-lang.org/reference/type-coercions.html
- Rust RFC Book: https://rust-lang.github.io/rfcs/0241-deref-conversions.html
