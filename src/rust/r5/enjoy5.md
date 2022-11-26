# Enjoy Day 5

![](/assets/kat.png) Glad you made it this far. We almost there!

## Trait, impl for

```rust,editable
// Just boring struct.
#[derive(Debug, Clone)]
struct Animal {}
struct Human {}

// ✨ New skill, look like interface.
trait Say {
    fn say(&self) -> String;
}

// ✨ Implement `Say` skill for `Animal`.
impl Say for Animal {
    // All animal wil say meow for now. 😆
    fn say(&self) -> String {
        "meow!".to_owned() // convert &str to String.
    }
}

// Implement `Say` skill for `Human`.
impl Say for Human {
    // All human kind say hi! 🤘
    fn say(&self) -> String {
        "hi!".to_owned() // convert &str to String.
    }
}

fn main() {
    let animal = Animal {};

    // So we can call like this.
    println!("{:?}", animal.say());

    // Or this.
    println!("{:?}", Animal::say(&animal));

    // Now human turn (with shorthand).
    println!("{:?}", Human::say(&Human {}));
}
```

![](/assets/duck.png) Hey! That's look like [`impl`](enjoy3.md) we learn before, but this time we can just apply some `trait` (aka skill) to some `struct`. So we didn't have any circular dependency problems!

## Box, dyn

![](/assets/kat.png) Sometime Rust didn't know what type (and size) we return so `Box` and `dyn` is here to help.

```rust,editable
// Same as previous example.
#[derive(Debug, Clone)]
struct Animal {}
struct Human {}

trait Say {
    fn say(&self) -> String;
}

impl Say for Animal {
    fn say(&self) -> String {
        "meow!".to_owned()
    }
}

impl Say for Human {
    fn say(&self) -> String {
        "hi!".to_owned()
    }
}

// ✨ Compiler need this 👇 to know it size.
fn animal_or_human() -> Box<dyn Say> {

    // ✨ How to get current time.
    let now = std::time::SystemTime::now();

    // ✨ New way to convert `Result` to `Option`
    let maybe_duration = now.elapsed().ok();

    match maybe_duration {
        Some(duration) => {
            // ✨ Modulo so we get 50% chance randomly by current time.
            if duration.as_micros() % 2 == 0 {
                Box::new(Animal {})
            } else {
                Box::new(Human {})
            }
        }
        // ✨ When you not finish implementation yet, try use todo.
        None => todo!(),
    }
}

fn main() {
    // Randomly say by animal or human.
    println!("{:?}", animal_or_human().say());
}
```

![](/assets/kat.png) At this point you should able to read 50% of `Rust` code out there, wait no more! let's go write something or continue to [next section](./r4/mod.md).

[Enjoy R4 ➠](../r4/mod.md)
