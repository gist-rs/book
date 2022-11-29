# Enjoy Day 5

![](/assets/kat.png) Glad you made it this far. We almost there!

## Trait, impl for

```rust,editable
// Just boring struct.
#[derive(Debug, Clone)]
struct Animal {}
struct Human {}

// ✨ New skill, look like interface.
trait Sayable {
    fn say(&self) -> String;
}

// ✨ Implement `Sayable` skill for `Animal`.
impl Sayable for Animal {
    // All animal wil say meow for now. 😆
    fn say(&self) -> String {
        "meow!".to_owned() // convert &str to String.
    }
}

// Implement `Sayable` skill for `Human`.
impl Sayable for Human {
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

![](/assets/kat.png) Sometime Rust didn't know what size (and type) we return so `Box` and `dyn` is here to help.

```rust
// ...Continue from example above.

# #[derive(Debug, Clone)]
# struct Animal {}
# struct Human {}
#
# trait Sayable {
#     fn say(&self) -> String;
# }
#
# impl Sayable for Animal {
#     fn say(&self) -> String {
#         "meow!".to_owned()
#     }
# }
#
# impl Sayable for Human {
#     fn say(&self) -> String {
#         "hi!".to_owned()
#     }
# }
#
// ✨ Compiler need this 👇 to know it size.
fn animal_or_human() -> Box<dyn Sayable> {
    // ✨ Compiler need this 👆 to know it's dynamic (Animal or Human)

    // ✨ How to get current time.
    let now = std::time::SystemTime::now();

    // ✨ How to get duration since UNIX_EPOCH.
    let result_duration = now.duration_since(std::time::UNIX_EPOCH);

    // ✨ How to convert `Result` to `Option`.
    let maybe_duration = result_duration.ok();

    match maybe_duration {
        Some(duration) => {
            // Take secs
            let sec = duration.as_secs();

            // ✨ Modulo so we get 50% chance randomly by current time.
            if sec % 2 == 0 {
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

[Continue to Day 6 ➠](./enjoy6.md)
