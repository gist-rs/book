# Associated Type Bounds

```rust,editable
// Define a trait `AnimalTrait` with an associated type `A`.
trait AnimalTrait {
    // Associated type for the animal's type (e.g., Cat).
    type A;

    // A method to create an instance of the animal.
    fn create() -> Self;

    // A method to get the name of the animal.
    fn name(&self) -> &str;
}

// Define a struct `Cat` that implements the `AnimalTrait` trait.
struct Cat;

impl AnimalTrait for Cat {
    type A = Cat;

    fn create() -> Self {
        Cat
    }

    fn name(&self) -> &str {
        "Cat"
    }
}

// Define a struct `Dog` that implements the `AnimalTrait` trait.
struct Dog;

impl AnimalTrait for Dog {
    type A = Dog;

    fn create() -> Self {
        Dog
    }

    fn name(&self) -> &str {
        "Dog"
    }
}

// This will look fancy a bit.
fn adopt_cat<T>(cat: T)
where
    // Here it is an 👇 Associated Type Bounds.
    T: AnimalTrait<A = Cat>,
{
    let cat_instance = T::create();
    let cat_name = cat_instance.name();

    println!("Adopted a {} named {}", cat.name(), cat_name);
}

fn main() {
    // Call the `adopt_cat` function, passing a `Cat` instance.
    adopt_cat(Cat);

    // 😱 Attempt to call the `adopt_cat` function with a `Dog` instance.
    // 👇 This will result in a compilation error.
    // adopt_cat(Dog);
    // --------- ^^^ type mismatch resolving `<Dog as AnimalTrait>::A == Cat`
}
```
