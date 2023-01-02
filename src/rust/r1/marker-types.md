# Marker Types

![](/assets/kat.png) Let's make use of `Marker Types` by a little help from [PhantomData](https://doc.rust-lang.org/nomicon/phantom-data.html) which use for make compiler happy.

```rust,editable
use std::marker::PhantomData;

// Unit types to represent each state.
struct Standby;
struct Running;

// 1️⃣ Standby state.
impl PlayStation<Standby> {
    // Can be turn on.
    pub fn turn_on(self) -> PlayStation<Running> {
        PlayStation::<Running> {
            state: PhantomData::<Running>,
        }
    }
}

// 2️⃣ Running state.
impl PlayStation<Running> {
    // Can be play.
    pub fn play(&mut self) {}
}

// *️⃣ PlayStation default to Standby state.
struct PlayStation<State = Standby> {
    state: PhantomData<State>,
}

impl Default for PlayStation<Standby> {
    fn default() -> Self {
        Self {
            state: Default::default(),
        }
    }
}

// This can be access in any state.
impl<State> PlayStation<State> {
    pub fn version(&self) -> u8 {
        5
    }
}

fn main() {
    // Cool! We just got new PS5!
    let ps5 = PlayStation::default();

    // Can access only version.
    println!("ps4_version:{}", ps5.version());

    // Can play after turn on.
    ps5.turn_on().play();
}
```
