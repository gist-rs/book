# Enjoy Day 5

![](/assets/kat.png) <span class="speech-bubble">We `impl` (implement) some `trait` (aka skill) for our `struct` so it can have any desired skill.</span>

## trait, impl

![](/assets/duck.png) <span class="speech-bubble">You also can `impl` any `trait` for `struct` 👇.</span>

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

![](/assets/duck.png) <span class="speech-bubble">That's look like [`impl`](enjoy3.md) to `struct`, but this time we can implement that `trait` to any `struct` we want!</span>

![](/assets/kat.png) <span class="speech-bubble">Take a way</span>

- `trait` is a skill.
- `impl` is how we implement skill.
- So `impl Sayable for Animal` littery mean `implement sayable skill to animal`. 🐈💬

---

## Async with Tokio

![](/assets/kat.png) <span class="speech-bubble">We will use `tokio` crate for now.</span>

<tabs>
<tab label="main.rs">

```rust,editable,edition2021
use std::thread::sleep;
use std::time::{Duration, SystemTime};

async fn sleep_2secs() {
    // Will sleep for 2 seconds.
    sleep(Duration::new(2, 0));
}

// This `async fn main` need `tokio::main`.
#[tokio::main]
async fn main() {
    // Wait for for 2 sec.
    let now = SystemTime::now();
    sleep_2secs().await;

    // Ensure it's 2 sec.
    let now_sec = now.elapsed().ok().unwrap().as_secs();
    assert_eq!(now_sec, 2);
    println!("We have been asleep for {} seconds.", now_sec);
}
```

</tab>
<tab label="Cargo.toml">

```toml
[package]
name = "foo"
version = "0.1.0"
edition = "2021"

[dependencies]
async-trait = "0.1.59"
tokio = { version ="1.22", features = ["full"] }
```

</tab>
</tabs>

---

## Async Traits

![](/assets/kat.png) <span class="speech-bubble">And let's use `tokio` with `trait`.</span>

<tabs>
<tab label="main.rs">

```rust,editable,edition2021
use async_trait::async_trait;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

// 👇 We need this.
#[async_trait]
trait Animal {
    // 👇 Because of this async.
    async fn sleep(&self);
}

struct Cat;

// 👇 Also here.
#[async_trait]
impl Animal for Cat {
    async fn sleep(&self) {
        // Will sleep for 2 seconds.
        sleep(Duration::new(2, 0));
    }
}

// This `async fn main` need `tokio::main`.
#[tokio::main]
async fn main() {
    // Wait for sleepy cat for 2 sec.
    let now = SystemTime::now();
    Cat {}.sleep().await;

    // Ensure it's 2 sec.
    let now_sec = now.elapsed().ok().unwrap().as_secs();
    assert_eq!(now_sec, 2);
    println!("Cat has been asleep for {} seconds.", now_sec);
}
```

</tab>
<tab label="Cargo.toml">

```toml
[package]
name = "foo"
version = "0.1.0"
edition = "2021"

[dependencies]
async-trait = "0.1.59"
tokio = { version ="1.22", features = ["full"] }
```

</tab>
</tabs>

> 💡 We also have `async trait` supported by `async-trait` lib. [Official support](https://blog.rust-lang.org/inside-rust/2022/11/17/async-fn-in-trait-nightly.html) is almost there.

---

![](/assets/kat.png) <span class="speech-bubble">Now, Let's play with `State Machine`🚀</span>

![](/assets/duck.png) <span class="speech-bubble">States? Like... awake state, sleepy state? 😴 <br>What's a `State Machine`? Sounds complicated!</span>

![](/assets/kat.png) <span class="speech-bubble">Not really! It's just a way to manage something that can be in different conditions (`states`) and how it moves between them. Like our cat, who can be asleep or awake! 😼</span>

## Simple State Machines: Cat Naps (and Snacks!)

![](/assets/kat.png) <span class="speech-bubble">Rust's type system itself can represent the state and prevent bugs! Let's make it impossible to wake up a cat that's already awake!</span>

> 💡 Inspired by [Hoverbear's State Machine Pattern](https://hoverbear.org/blog/rust-state-machine-pattern/)

```rust,editable
use std::marker::PhantomData;

// --- Marker Types for States ---
// These don't hold data, they just act like labels for our states.
#[derive(Debug, Clone, Copy)] // So we can print and copy them easily
struct Sleeping;

#[derive(Debug, Clone, Copy)]
struct Awake;

// --- Our Cat Struct ---
// It's generic! The `State` tells us what condition the cat is in.
// PhantomData tells Rust "we care about this `State` type, even if we don't store data of that type".
#[derive(Debug)]
struct Cat<State> {
    name: String,
    _state: PhantomData<State>, // We use PhantomData because State is just a marker
}

// --- Transitions ---

// Implementation block ONLY for Cats that are Sleeping
impl Cat<Sleeping> {
    // This function takes a Cat<Sleeping>...
    fn wake_up(self) -> Cat<Awake> {
        // ...and returns a NEW Cat<Awake>!
        println!("{} stretches and yawns... now awake! ☀️", self.name);
        Cat {
            name: self.name, // Keep the same name
            _state: PhantomData, // Mark it as Awake now
        }
    }
    // Notice: There's no `lull_to_sleep` or `eat` function here! Sleeping cats don't eat!
}

// Implementation block ONLY for Cats that are Awake
impl Cat<Awake> {
    // This function takes a Cat<Awake>...
    fn lull_to_sleep(self) -> Cat<Sleeping> {
        // ...and returns a NEW Cat<Sleeping>!
        println!("{} curls up and starts snoozing... 😴", self.name);
        Cat {
            name: self.name, // Keep the same name
            _state: PhantomData, // Mark it as Sleeping now
        }
    }

    // *** NEW *** Let the awake cat eat!
    // It takes any type that can be turned 'Into' a String for the food.
    // It consumes the Cat<Awake> and returns it, still Awake.
    fn eat(self, food: impl Into<String>) -> Self {
        // Here we convert the input 'food' into a real String
        let food_item: String = food.into();
        println!("{} happily munches on some {}! Yum! 🐟", self.name, food_item);
        // The cat ate, but is still awake, so we return the same cat (type).
        self
    }
    // Notice: No `wake_up` function here! Can't wake up an awake cat!
}

// --- How to create our first Cat ---
impl Cat<Sleeping> {
    // A function to create a new cat, starting in the Sleeping state.
    fn new_sleeping_cat(name: String) -> Self {
         println!("A new cat, {}, appears, already asleep...", name);
        Cat { name, _state: PhantomData }
    }
}

fn main() {
    // Mittens starts asleep
    let mittens = Cat::new_sleeping_cat("Mittens".to_string());
    println!("*️⃣ Initial state: {:?}", mittens);

    // We can ONLY call wake_up because mittens is Cat<Sleeping>
    let mittens = mittens.wake_up();
    println!("➡️ After waking up: {:?}", mittens);

    // *** NEW *** Now that Mittens is Awake, she can eat!
    // We pass a &str, but `eat` turns it into a String using .into()
    let mittens = mittens.eat("delicious tuna");
    println!("➡️ After eating: {:?}", mittens); // Still Awake

    // We could even pass a String directly if we wanted!
    let mittens = mittens.eat("fancy salmon".to_string());
    println!("➡️ After eating again: {:?}", mittens); // Still Awake


    // Now we can ONLY call lull_to_sleep because mittens is Cat<Awake>
    let mittens = mittens.lull_to_sleep();
    println!("➡️ After falling asleep again: {:?}", mittens);

    // Try uncommenting this - it won't compile! Mittens is Sleeping!
    // let mittens = mittens.eat("a midnight snack");

    // Try uncommenting this - it won't compile! Mittens is Sleeping!
    // let mittens = mittens.lull_to_sleep();
}
```

[Continue to Day 6 ➠](./enjoy6.md)
