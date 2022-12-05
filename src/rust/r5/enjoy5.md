# Enjoy Day 5

![](/assets/kat.png) We `impl` (implement) some `trait` (aka skill) for `struct` so that `struct` can have that skill.

## trait, impl

```rust,editable
// Just boring struct.
#[derive(Debug, Clone)]
struct Animal {}
struct Human {}

// New skill. Wanna to say something?
trait Sayable {
    // This nearly like interface.
    fn say(&self) -> String; // We use String instead of &str here for no reason.
}

// Implement `Sayable` skill for `Animal`.
impl Sayable for Animal {
    // All animal will say meow for now. 😆
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

![](/assets/duck.png) That's look like [`impl`](enjoy3.md) to `struct`, but this time we can implement that `trait` to any `struct` we want!

---

![](/assets/kat.png) Sometime Rust didn't know what size (and type) we return so `Box` and `dyn` is here to help.

## Dynamic Dispatch with `Box`, `dyn`

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

// Compiler'll need this👇 Box to know a size (Box size BTW).
fn animal_or_human() -> Box<dyn Sayable> {
    // Compiler'll need this 👆 dyn to know it's dynamic (Animal or Human)

    // How to get current time.
    let now = std::time::SystemTime::now();

    // How to get duration since UNIX_EPOCH.
    let result_duration = now.duration_since(std::time::UNIX_EPOCH);

    // How to convert `Result` to `Option`.
    let maybe_duration = result_duration.ok();

    match maybe_duration {
        Some(duration) => {
            // Take secs
            let sec = duration.as_secs();

            // Modulo so we get 50% chance randomly by current time.
            if sec % 2 == 0 {
                Box::new(Animal {})
            } else {
                Box::new(Human {})
            }
        }
        // When you not finish implementation yet, try use todo!.
        None => todo!(),
    }
}

fn main() {
    // Randomly say by animal or human.
    println!("{:?}", animal_or_human().say());
}
```

![](/assets/kat.png) This `-> Box<dyn Sayable>` is [Dynamic Dispatch](https://doc.rust-lang.org/book/ch17-02-trait-objects.html#trait-objects-perform-dynamic-dispatch) which have some overhead compare to `-> impl Sayable` (static dispatch).

---

![](/assets/kat.png) Let's try avoid `Dynamic Dispatch` with `Static Dispatch` by [either](https://crates.io/crates/either)

## Static Dispatch with [either](https://crates.io/crates/either)

```rust,no_run
// ...Continue from example above.
# #[derive(Debug, Clone)]
# struct Human {}
# struct Animal {}
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

use either::Either;
use rand::Rng;

// Can be either Sayable or Sayable.
fn animal_or_human() -> Either<impl Sayable, impl Sayable> {
    // Use random number instead of time, just for fun.
    if rand::thread_rng().gen_range(0u8..9u8) % 2 == 0 {
        Either::Left(Animal {})
    } else {
        Either::Right(Human {})
    }
}

fn main() {
    // Randomly say by animal or human.
    let either_animal_or_human = animal_or_human();
    println!(
        "{:?}",
        match either_animal_or_human.is_left() {
            true => either_animal_or_human.left().unwrap().say(),
            false => either_animal_or_human.right().unwrap().say(),
        }
    );
}
```

![](/assets/kat.png) Let's try avoid `Dynamic Dispatch` with `Static Dispatch` by [enum_dispatch](https://crates.io/crates/enum_dispatch)

## Static dispatch with [enum_dispatch](https://crates.io/crates/enum_dispatch)

```rust,no_run
// ...Continue from example above.
# #[derive(Debug, Clone)]
# struct Human {}
# struct Animal {}
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

use enum_dispatch::enum_dispatch;
use rand::Rng;

#[enum_dispatch]
enum SayableEnum {
    Animal,
    Human,
}

fn animal_or_human() -> SayableEnum {
    if rand::thread_rng().gen_range(0u8..9u8) % 2 == 0 {
        Animal {}.into()
    } else {
        Human {}.into()
    }
}

fn main() {
    // Randomly say by animal or human.
    println!(
        "{:?}",
        match animal_or_human() {
            SayableEnum::Animal(e) => e.say(),
            SayableEnum::Human(e) => e.say(),
        }
    );
}
```

---

![](/assets/kat.png) We also have `async trait` supported by `async-trait` lib. [Official support](https://blog.rust-lang.org/inside-rust/2022/11/17/async-fn-in-trait-nightly.html) is almost there.

## Async Traits

### `Cargo.toml`

```toml
[package]
name = "foo"
version = "0.1.0"
edition = "2021"

[dependencies]
async-trait = "0.1.59"
tokio = { version ="1.22", features = ["full"] }
```

### `main.rs`

```rust,editable,edition2021
use async_trait::async_trait;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

#[async_trait]
trait Animal {
    async fn sleep(&self);
}

struct Cat;

#[async_trait]
impl Animal for Cat {
    async fn sleep(&self) {
        // will sleep for 2 seconds
        sleep(Duration::new(2, 0));
    }
}

// This `async fn main` need `tokio::main`
#[tokio::main]
async fn main() {
    let now = SystemTime::now();
    Cat {}.sleep().await;

    let now_sec = now.elapsed().ok().unwrap().as_secs();
    assert_eq!(now_sec, 2);
    println!("Cat has been sleep for {} sec.", now_sec);
}
```

[Continue to Day 6 ➠](./enjoy6.md)
