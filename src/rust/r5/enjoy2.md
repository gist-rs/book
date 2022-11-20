# Enjoy Day 2

![](/assets/kat.png) Hey! Nice to see you here. Previously we use `HashMap` which is fine but `Struct` is way more better, Let's get grab some coffee ☕️ and getting start.

## Struct

```rust,editable
fn main() {
    // 😭 Before: use `Tuple`.
    let animal = (("name", "foo"), ("age", 42));
    println!("{0:?}: {1:?}", animal.0 .0, animal.0 .1); // 😳 So hard to access tuple!
    println!("{0:?}: {1:?}", animal.1 .0, animal.1 .1); // 😭 Stop this!

    // 😘 After: use `Struct`.
    struct Animal {
        name: String, // We use `String` here not &str (will talk about this later).
        age: u8,      // 😳 `u8` mean unsigned integer (2^8 − 1) = 255
    }

    // Create animal
    let animal = Animal {
        name: "foo".to_owned(), // 😳 You can use `to_string()` here.
        age: 42u8,              // 😳 Shorthand for casting `42 as u8` or `42_u8`.
    };

    println!("name: {:?}", animal.name); // 😚 So easy to use!
    println!("age: {:?}", animal.age);
}
```

![](/assets/duck.png) Cool `Struct` seem handy, gimme more.

```rust,editable
// 👇 Let's move struct out from `fn main`.
#[derive(Debug)] // 😳 derive Debug so we can print later.
struct Animal {
    name: String,
    age: u8,
}

// 😳 We will implement some method for Animal.
impl Animal {
    // 😳 `new` constructor return 👇 itself call `Self`.
    fn new(name: &str, age: u8) -> Self {
        Animal {
            name: name.to_owned(),
            age,
        }
    }
}

fn main() {
    // New `new` style, so cool!
    let animal = Animal::new("foo", 42u8);

    // We can print here because of derive Debug.
    println!("{:#?}", animal);
}
```

> 💡 You can read more about `struct` [here](https://doc.rust-lang.org/rust-by-example/custom_types/structs.html)

![](/assets/kat.png)
