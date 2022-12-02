# Enjoy Day 4

![](/assets/kat.png) Now we need `enum` to replace "cat" and "duck" type from previous example.

## Enum

```rust,editable
#[derive(Debug)]
enum AnimalType {
    Cat,
    Duck,
}

// How to return string or &str from enum.
impl AnimalType {
    fn as_str(&self) -> &str {
        match self {
            AnimalType::Cat => "🐈",
            AnimalType::Duck => "🐥",
        }
    }
}

// How to use type as a parameters, hey!👇 what's this? 😳
fn say(animal_type: AnimalType) -> &'static str {
    // ✨ To survive from fn {}, we need 👆 'static to let is has program's lifetime.
    match animal_type {
        AnimalType::Cat => "meaowww",
        AnimalType::Duck => "quackkk",
    }
}

fn main() {
    println!(
        "{0:?} aka {1:?} say {2:?}",
        AnimalType::Cat,
        AnimalType::Cat.as_str(),
        say(AnimalType::Cat)
    );
    println!(
        "{0:?} aka {1:?} say {2:?}",
        AnimalType::Duck,
        AnimalType::Duck.as_str(),
        say(AnimalType::Duck)
    );
}
```

> 💡 There's more examples about [Enums](https://doc.rust-lang.org/rust-by-example/custom_types/enum.html), and [match enums](https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring/destructure_enum.html).

![](/assets/kat.png) Don't worry about `&'static str` or `lifetimes` just yet, compiler will let you know when need (usually out of `{ }` scope ) and we will talk about it later. Let's continue on other topics.

## strum, Result, Ok, Err

```rust,editable
use std::str::FromStr;
use strum_macros::{Display, EnumString};

#[derive(Debug, Eq, PartialEq, EnumString, Display)]
enum AnimalType {
    #[strum(serialize = "cat", to_string = "catty")]
    Cat,
    #[strum(serialize = "duck", to_string = "ducky")]
    Duck,
    Unknown,
    #[strum(disabled)]
    Pet(String),
}

#[derive(Debug, Eq, PartialEq, EnumString, Display)]
enum AnimalSound {
    #[strum(serialize = "cat", to_string = "meaowww")]
    Cat,
    #[strum(serialize = "duck", to_string = "quackkk")]
    Duck,
}

fn main() {
    // ✨ Get AnimalType from &str.
    let animal_type = AnimalType::from_str("cat");
    println!("1️⃣ animal_type: {animal_type:?}");

    // ✨ Unwrap or assign as Unknown.
    let animal_type = animal_type.unwrap_or(AnimalType::Unknown).to_string();
    println!("2️⃣ animal_type: {animal_type:?}");

    // ✨ Get AnimalSound from str.
    let cat_sound_result = AnimalSound::from_str("cat");
    println!("3️⃣ cat_sound_result: {:?}", cat_sound_result);

    // ✨ Handle cat_sound Result.
    let cat_sound_string = match cat_sound_result {
        // ✨ Handle happy case.
        Ok(animal_sound) => animal_sound.to_string(),

        // ✨ Handle error case.
        Err(err) => panic!("{:?}", err),
    };

    println!("4️⃣ cat_sound_string: {cat_sound_string:?}");

    // 😱 Uncomment this to experience an error and try to fix it by add Clone, Copy to AnimalSound
    // println!("4️⃣ cat_sound_result: {cat_sound_result:?}");

    // Match
    let animals = vec![AnimalType::Cat, AnimalType::Pet("snoopy".to_owned())];
    let my_pet = animals
        .into_iter()
        .filter_map(|e| match e {
            AnimalType::Pet(name) => Some(name),
            AnimalType::Cat => None,
            AnimalType::Duck => None,
            AnimalType::Unknown => None,
        })
        .collect::<Vec<_>>();

    println!("5️⃣ my_pet: {:?}", my_pet.join(","));
}
```

> 🤷‍♂️ `strum` is not runnable via Rust Playground so output is shown below.

<details>
<summary>Run</summary>

```
1️⃣ animal_type: Ok(Cat)
2️⃣ animal_type: "catty"
3️⃣ cat_sound_result: Ok(Cat)
4️⃣ cat_sound_string: "meaowww"
5️⃣ my_pet: "snoopy"
```

</details>

> 💡 `Result<T, E>`⎯⎯ unwrap → `Ok<T>`/`Err(E)` which `T`,`E` are generic.  
> Read more about how to handle `Result` [here](https://doc.rust-lang.org/rust-by-example/error/result.html)

[Continue to Day 5 ➠](./enjoy5.md)
