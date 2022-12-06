# Enjoy Day 7

![](/assets/kat.png) Sometime compiler didn't know what size (and type) we will return so `Box` and `dyn` is here to help.

![](/assets/duck.png) Oh, it's just like we put something in the 📦 so mister postman(compiler) can manage them properly!

![](/assets/kat.png) Yes!, same idea for `String` 👉 `str`, `Vec` 👉 `array` which is `Heap` 👉 `Stack`.

## Dynamic Dispatch with `Box`, `dyn`

```rust
// ...Continue from previous example.
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

![](/assets/kat.png) We call this `-> Box<dyn Sayable>` as [Dynamic Dispatch](https://doc.rust-lang.org/book/ch17-02-trait-objects.html#trait-objects-perform-dynamic-dispatch) which have some little overhead compare to `-> impl Sayable` (static dispatch).

---

![](/assets/kat.png) Let's try avoid `Dynamic Dispatch` with `Static Dispatch` by [enum_dispatch](https://crates.io/crates/enum_dispatch)

## Static dispatch with [enum_dispatch](https://crates.io/crates/enum_dispatch)

```rust,no_run
// ...Continue from previous example.
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

![](/assets/kat.png) Or avoid `Dynamic Dispatch` with `Static Dispatch` by [either](https://crates.io/crates/either)

## Static Dispatch with [either](https://crates.io/crates/either)

```rust,no_run
// ...Continue from previous example.
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

---

![](/assets/kat.png) At this point you should able to read a lot of `Rust` code out there, let's [teardown ➠](./teardown.md).
