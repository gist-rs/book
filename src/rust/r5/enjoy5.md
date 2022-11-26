# Enjoy Day 5

![](/assets/kat.png) Glad you made it this far. We almost there!

## Trait

```rust,editable
// Just boring struct
#[derive(Debug, Clone)]
struct Animal {}
struct Human {}

// ✨ New skill, look like interface.
trait Say {
    fn static_say() -> String;
    fn say(&self) -> String;
}

// ✨ Implement `Say` skill for `Animal`
impl Say for Animal {
    fn say(&self) -> String {
        "meow!".to_owned() // convert &str to String
    }

    // Remember static say?
    fn static_say() -> String {
        "meowwww!".to_owned()
    }
}

// ✨ Implement `Say` skill for `Human`
impl Say for Human {
    fn say(&self) -> String {
        "hi!".to_owned() // convert &str to String
    }

    // Remember static say?
    fn static_say() -> String {
        "hello!".to_owned()
    }
}

fn main() {
    let animal = Animal {};

    // So we can call like this
    println!("{:?}", animal.say());

    // Or this
    println!("{:?}", Animal::static_say());

    // Or even this
    println!("{:?}", Animal::say(&animal));

    // Now human
    println!("{:?}", Human::static_say());
}
```

![](/assets/duck.png) Hey! That's look like [`impl`](enjoy3.md) we learn before, but this time we can apply skill to something instead of include that skill permanently. So we didn't have any circular dependency!
