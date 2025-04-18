# Enum

## Beware!

```rust,editable
enum Foo {
    Bar = 43114
}

fn main() {
    // Happy case
    println!("{:?}", Foo::Bar as u64); // 43114
    println!("{:?}", 43114 as u64);    // 43114

    // BEWARE! This get 106 not 43114 nor error.
    println!("{:?}", Foo::Bar as u8);  // 106 😱

    // This will throw an error.
    // println!("{:?}", 43114 as u8);
}
```

## How to take value and convert from `Enum Variants`.

> Refer to [`Rust Design Patterns`](https://rust-unofficial.github.io/patterns/idioms/mem-replace.html)

```rust,editable
use std::mem;

#[derive(Debug)]
enum FooBarEnum {
    Foo { name: String, x: u8 },
    Bar { name: String },
}

// 1️⃣ Convert Foo → Bar with if
fn foo_to_bar_with_if(e: &mut FooBarEnum) {
    if let FooBarEnum::Foo { name, x: 0 } = e {
        // Takes out `name` and assigned to `*e`.
        *e = FooBarEnum::Bar {
            name: mem::take(name),
        };
    }
}

// 2️⃣ Convert Bar → Foo with match.
fn bar_to_foo_with_match(e: &mut FooBarEnum, x: u8) {
    match e {
        FooBarEnum::Bar { name } => {
            // Takes out `name` and assigned to `*e`.
            *e = FooBarEnum::Foo {
                name: mem::take(name),
                x,
            };
        }
        _ => panic!("Not supported."),
    }
}

fn main() {
    // *️⃣ Create some Foo.
    let foo = &mut FooBarEnum::Foo {
        name: "foo".to_owned(),
        x: 0,
    };
    println!("foo: {:?}", foo);

    // 1️⃣ Convert Foo → Bar with if
    foo_to_bar_with_if(foo);
    println!("foo_to_bar_with_if: {:?}", foo);

    // 2️⃣ Convert Bar → Foo with match.
    bar_to_foo_with_match(foo, 1);
    println!("bar_to_foo_with_match: {:?}", foo);
}
```
