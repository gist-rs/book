# Enjoy Day 3

![](/assets/kat.png) Hey! Nice to see you here. Previously we use `HashMap` which is fine but `Struct` is way more better, Let's get grab some coffee ☕️ and getting start.

## Struct

```rust,editable
fn main() {
    // 😭 Before: use `Tuple`.
    let animal = (("name", "foo"), ("age", 42));
    println!("{0:?}: {1:?}", animal.0 .0, animal.0 .1); // 😭 So hard to access tuple!
    println!("{0:?}: {1:?}", animal.1 .0, animal.1 .1); // 😭 Stop this!

    // 😚 After: use `Struct`.
    struct Animal {
        name: String, // We use `String` here not &str (will talk about this later).
        age: u8,      // ✨ `u8` mean unsigned integer (2^8 − 1) = 255
    }

    // Create animal
    let animal = Animal {
        name: "foo".to_owned(), // ✨ You can also use `to_string()` here.
        age: 42u8,              // ✨ Shorthand for casting `42 as u8` or `42_u8`.
    };

    println!("name: {:?}", animal.name); // 😚 So easy to use!
    println!("age: {:?}", animal.age);
}
```

![](/assets/duck.png) Cool `Struct` seem handy, gimme more.

## derive, impl, Self, &self

```rust,editable
// 👇 Let's move struct out from `fn main`.
#[derive(Debug, Clone)] // ✨ Derive Debug so we can print later.
struct Animal {
    #[allow(dead_code)] // ✨ Allow dead code.
    name: String,

    #[allow(dead_code)]
    age: u8,

    // 👇 `type` is reserved word but we still can use it.
    r#type: String, // ✨ r# mean raw string.
}

// ✨ We will implement some method for Animal.
impl Animal {
    // ✨ `new` constructor return 👇 itself call `Self`.
    fn new(name: &str, age: u8) -> Self {
        Animal {
            name: name.to_owned(),
            age,
            r#type: "duck".to_owned(),
        }
    }

    // ✨ `new_cat` alternative constructor with default type.
    fn new_cat(name: &str, age: u8) -> Self {
        Animal {
            name: name.to_owned(),
            age,
            r#type: "cat".to_owned(),
        }
    }

    // ✨ Define static method.
    pub fn static_say(animal_type: &str) -> &str {
        match animal_type {
            // 👇 This &str is bad practice, we need enum here.
            "cat" => "meaowww",
            "duck" => "quackkk",
            _ => "wat!?",
        }
    }

    // ✨ Define self 👇 method.
    pub fn say(&self) -> &str {
        let animal_type = self.r#type.as_str();
        Animal::static_say(animal_type)
    }
}

fn main() {
    // ✨ Now we can call 👇 new like this.
    let animal = Animal::new("foo", 42u8);
    println!("animal: {:#?}", animal);

    // ✨ Call say via static method.
    let static_say_str = Animal::static_say("duck");
    println!("static_say_str: {:#?}", static_say_str);

    // ✨ We can create new cat 👇 like this.
    let cat = Animal::new_cat("bar", 24u8);
    println!("cat: {:#?}", cat);

    // ✨ Call say via method itself.
    let say_str = cat.say();
    println!("say_str: {:#?}", say_str);

    // ✨ Or via Animal 😳
    println!("Animal::say: {:#?}", Animal::say(&cat));

    // ✨ You can also clone after derive Clone above 👆
    let mut duck = cat.clone();
    duck.name = "duck the duck".to_owned();
    duck.age = 13;

    //  ✨ Destructing from struct.
    let Animal { age, .. } = cat;
    println!("age: {:#?}", age);

    //  ✨ Match struct where animal
    match &duck {
        // ✨ Match age at 24
        Animal { age: 24, .. } => println!("match age at 24 : {:#?}", age),

        // ✨ Match age between 30-50 range.
        Animal { age: 30..=50, .. } => println!("match age between 30-50 : {:#?}", age),

        // Guard name equal to "foo"
        Animal { name, .. } if name == "duck the duck" => println!("animal.name: {:#?}", name),

        // Other age.
        _ => println!("age not in range"),
    }
}
```

> 💡 You can derive more than one e.g. `#[derive(Debug,Display,Clone,Copy)]` read more about `derive` [here](https://doc.rust-lang.org/rust-by-example/trait/derive.html) and `struct` [here](https://doc.rust-lang.org/rust-by-example/custom_types/structs.html).

[Continue to Day 4 ➠](./enjoy4.md)
