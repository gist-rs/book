# Lifetimes

![](/assets/kat.png) This is a short note of [this book](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)

## Lifetime

✅ borrowed value does live long enough.

```rust,editable
fn main() {
    let my_money;

    // This is fine.
    let your_money = 5;
    my_money = your_money;

    println!("my_money: {}", my_money);
    println!("your_money: {}", your_money);
}
```

❎ borrowed value does not live long enough.

```rust,editable
fn main() {
    let my_money;

    // 😱 This is not, because `your_money` is in your { } scope.
    {
        let your_money = 5;
        my_money = &your_money;
    }
    // 👆 `your_money` dropped here, it won't leave your { } scope.

    // borrow later used here 👇.
    println!("my_money: {}", my_money);
}
```

## Lifetime Annotations

### Lifetime Annotations in Function Signatures

```rust,no_run
fn main() {
    // Actually we need 'a lifetime annotations. 😱
    fn hello_with_lifetime<'a>(x: &'a str) -> &'a str {
        x
    }

    // Or this... 😱
    fn hello_with_any_lifetime(x: &'_ str) -> &'_ str {
        x
    }

    // Good news, we can do this instead (Thanks to compiler!) 🙏
    fn hello_str(x: &str) -> &str {
        x
    }

    // Anyway for multiple params, we not sure how long lifetime each one.
    // So this 👇 and  👇 also here 👇 and here 👇 will need. 😅
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        // This └───────┴───────────┴───────────┘ have same lifetime
        // which defined as 'a (can be any e.g. 'foo, 'lol).

        if x.len() > y.len() {
            x // Maybe return this
        } else {
            y // Maybe return this
        }
    }

    println!("1️⃣ {:?}", hello_with_lifetime("world"));
    println!("2️⃣ {:?}", hello_with_any_lifetime("world"));
    println!("3️⃣ {:?}", hello_str("world"));

    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("3️⃣ The longest string is {}", result);
    }
}
```

### Lifetime Annotations in Method Definitions

```rust,editable
// We need👇 <'a> here.
struct Me<'a> {
    name: &'a str, // Because of this 'a.
}

// So this will need <'a> here too! 🤷
impl<'a> Me<'a> {
    fn say_my_name(&self) -> &str {
        self.name
    }
}

// But this don't
struct You {
    name: String, // Because of no 'a here.
}

// So this no need <'a>.
impl You {
    fn say_my_name(&self) -> String {
        self.name.to_owned()
    }
}

// And this also don't need <'a>
struct Cat {
    name: &'static str, // Because of no 'a here.
}

// So this no need <'a>.
impl Cat {
    fn say_my_name(&self) -> &str {
        self.name
    }
}

fn main() {
    println!("{:?}", Me { name: "foo" }.say_my_name());

    println!("{:?}", You { name: "bar".to_owned() }.say_my_name());

    println!("{:?}", Cat { name: "baz" }.say_my_name());
}
```

### Fun facts

- `&'static str` = lives the entire lifetime of your program = book hotel for entire year = use it wisely.
- `String` = smart pointer = heap = a bit more allocation (not much).
- `&'a str` = lifetime annotations = more specific lifetime = good (but headache).

![](/assets/kat.png) Now we know that we need to add `<'a>` lifetime annotations to let compiler know its lifetime.
