# Enjoy Day 5

![](/assets/kat.png) <span class="speech-bubble">We `impl` (implement) some `trait` (aka skill) for our `struct` so it can have any desired skill.</span>

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

![](/assets/duck.png) <span class="speech-bubble">That's look like [`impl`](enjoy3.md) to `struct`, but this time we can implement that `trait` to any `struct` we want!</span>

---

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
    println!("Cat has been sleep for {} sec.", now_sec);
}
```

> 💡 We also have `async trait` supported by `async-trait` lib. [Official support](https://blog.rust-lang.org/inside-rust/2022/11/17/async-fn-in-trait-nightly.html) is almost there.

---

[Continue to Day 6 ➠](./enjoy6.md)
