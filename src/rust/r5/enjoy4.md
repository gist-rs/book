# Enjoy Day 4

![](/assets/kat.png) Now we need `enum` to replace "cat" and "duck" type by previous example.

## Enum

```rust,editable
#[derive(Debug)]
enum AnimalType {
    Cat,
    Duck,
}

fn sound_of(animal_type: AnimalType) -> &'static str {
    match animal_type {
        AnimalType::Cat => "meaowww",
        AnimalType::Duck => "quackkk",
    }
}

fn main() {
    println!("{0:?}: {1:?}", AnimalType::Cat, sound_of(AnimalType::Cat));
    println!("{0:?}: {1:?}", AnimalType::Duck, sound_of(AnimalType::Duck));
}
```

## strum, Result, Ok, Err

```rust,no_run
use std::str::FromStr;
use strum_macros::{Display, EnumString};

#[derive(Debug, Eq, PartialEq, EnumString, Display)]
enum AnimalType {
    #[strum(serialize = "cat", to_string = "catty")]
    Cat,
    #[strum(serialize = "duck", to_string = "ducky")]
    Duck,
    Unknown,
}

#[derive(Debug, Eq, PartialEq, EnumString, Display)]
enum AnimalSound {
    #[strum(serialize = "cat", to_string = "meaowww")]
    Cat,
    #[strum(serialize = "duck", to_string = "quackkk")]
    Duck,
}

fn main() {
    // ✨ Get AnimalType from str.
    let animal_type = AnimalType::from_str("cat");
    println!("1️⃣ animal_type: {animal_type:?}");

    println!(
        "2️⃣ animal_type: {:?}",
        animal_type.unwrap_or(AnimalType::Unknown).to_string()
    );

    // ✨ Get AnimalSound from str.
    let cat_sound = AnimalSound::from_str("cat");
    println!("3️⃣ cat_sound: {cat_sound:?}");

    // ✨ Handle cat_sound Result.
    let cat_sound_string = match cat_sound {
        // ✨ Handle happy case.
        Ok(animal_sound) => animal_sound.to_string(),

        // ✨ Handle error case.
        Err(err) => panic!("{:?}", err),
    };

    println!("4️⃣ cat_sound_string: {cat_sound_string:?}");
}
```

> 🤷‍♂️ `strum` is not runnable via Rust Playground so output is shown below.

```
1️⃣ animal_type: Ok(Cat)
2️⃣ animal_type: "catty"
3️⃣ cat_sound: Ok(Cat)
4️⃣ cat_sound_string: "meaowww"
```

> 💡 Like an `Option`, but this time `Result<T, Error>`⎯⎯ unwrap → `Ok<T>`/`Err`.  
> Read more about how to handle `Result` [here](https://doc.rust-lang.org/rust-by-example/error/result.html)
