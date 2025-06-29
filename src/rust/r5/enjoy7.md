# Enjoy Day 7

![](/assets/kat.png) <span class="speech-bubble">Sometime compiler didn't know what size (and type) we will return so `Box` and `dyn` is here to help.</span>

![](/assets/duck.png) <span class="speech-bubble">Oh, it's just like we put something in the 📦 so mister postman(compiler) can manage them properly!</span>

```rust,editable
let rust = "Rust".to_string()
let us = &rust[1..=2];
```

```bob

Stack                              Heap
.- - - - - - - - - - - - - -.      .- - - - - - - - - - - - - -.
:                           :      :                           :
:    String "Rust"          :      :    str                    :
:   +-----------+------+    : owns :   +----+----+----+----+   :
:   | ptr       |   o--+----+------+-->| R  | u  | s  | t  |   :
:   | len       |   4  |    :      :   +----+----+----+----+   :
:   | capacity  |   4  |    :      :           ^               :
:   +-----------+------+    :      :           |               :
:                           :      :           |               :
`- - - - - - - - - - - - - -'      `- - - - - -+- - - - - - - -'
                                               |
Stack                                          |
.- - - - - - - - - - - - - -.                  |
:                           :                  |
:    &str "us"              :                  |
:   +-----------+------+    : borrows          |
:   | ptr       |   o--+----+------------------+
:   | len       |   2  |    :
:   +-----------+------+    :
:                           :
`- - - - - - - - - - - - - -'
```

![](/assets/kat.png) <span class="speech-bubble">Imagine each `R` `u` `s` `t` is super car...</span>

![](/assets/duck.png) <span class="speech-bubble">It would be nice if we can borrow `&` instead of buying a new one!</span>

## Dynamic Dispatch with `Box`, `dyn`

```rust,editable
#[derive(Debug, Clone)]
struct Animal {}
struct Human {}

trait Sayable {
    fn say(&self) -> String;
}

impl Sayable for Animal {
    fn say(&self) -> String {
        "meow!".to_owned()
    }
}

impl Sayable for Human {
    fn say(&self) -> String {
        "hi!".to_owned()
    }
}

// Compiler'll need this👇 Box to know its size (Box's size).
fn animal_or_human() -> Box<dyn Sayable> {
    // Compiler'll need this 👆 dyn to know it'll be dynamic (Animal or Human)

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

![](/assets/kat.png) <span class="speech-bubble">We call this `-> Box<dyn Sayable>` as [Dynamic Dispatch](https://doc.rust-lang.org/book/ch17-02-trait-objects.html#trait-objects-perform-dynamic-dispatch) which have some little overhead compare to `-> impl Sayable` (static dispatch).</span>

---

![](/assets/kat.png) <span class="speech-bubble">Let's try avoid `Dynamic Dispatch` with `Static Dispatch` by [enum_dispatch](https://crates.io/crates/enum_dispatch)</span>

## Static dispatch with [enum_dispatch](https://crates.io/crates/enum_dispatch)

```rust,no_run
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

![](/assets/kat.png) <span class="speech-bubble">Or avoid `Dynamic Dispatch` with `Static Dispatch` by [either](https://crates.io/crates/either)</span>

## Static Dispatch with [either](https://crates.io/crates/either)

```rust,no_run
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

![](/assets/kat.png) <span class="speech-bubble">At this point you should able to read a lot of `Rust` code out there, let's [teardown ➠](./teardown.md).</span>
