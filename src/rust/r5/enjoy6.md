# Enjoy Day 6

## Supertraits

```rust,editable
trait Human {
    fn name(&self) -> String;
}

trait Learner: Human {
    fn is_enjoy(&self) -> bool;
}

trait Coder {
    fn language(&self) -> String;
}

trait Rustaceans: Coder + Learner {
    fn blog(&self) -> String;
}

struct Me {}
impl Human for Me {
    fn name(&self) -> String {
        "katopz".to_owned()
    }
}
impl Learner for Me {
    fn is_enjoy(&self) -> bool {
        true
    }
}
impl Coder for Me {
    fn language(&self) -> String {
        "rust".to_owned()
    }
}
impl Rustaceans for Me {
    fn blog(&self) -> String {
        "https://katopz.medium.com/".to_owned()
    }
}

fn greeting_rustaceans(someone: &dyn Rustaceans) -> String {
    format!(
        "My name is {}, I {} coding in {}, you can visit my blog at {}.",
        someone.name(),
        someone.is_enjoy().then(|| "enjoy").unwrap_or("sad"),
        someone.language(),
        someone.blog(),
    )
}

fn main() {
    println!("{}", greeting_rustaceans(&Me {}));
}
```

> 💡 More about [Supertraits](https://doc.rust-lang.org/rust-by-example/trait/supertraits.html)

## Generic Bounds

```rust,editable
#[derive(Debug)]
struct Animal {
    weight: f64,
}

impl Animal {
    fn add_weight(&mut self, weight: f64) {
        self.weight += weight
    }
}

trait Feedable {
    fn feed(&mut self, food_amount: f64);
}

impl Feedable for Animal {
    fn feed(&mut self, food_amount: f64) {
        self.add_weight(food_amount);
    }
}

// Bounds only Feedable allowed here.
fn feed<T: Feedable>(t: &mut T) {
    t.feed(1f64)
}

// Cat
trait Cat {}

impl Cat for Animal {}

// ✨ Multiple bounds
fn feed_cat_with_amount<T: Feedable + Cat>(t: &mut T, amount: f64) {
    t.feed(amount)
}

// Duck
trait Duck {}

#[allow(dead_code)]
fn feed_duck_100<T: Feedable + Duck>(t: &mut T) {
    t.feed(100f64)
}

// ✨ Or use traits as parameters like this
fn feed_duck_200(t: &mut (impl Feedable + Cat)) {
    t.feed(200f64)
}

fn main() {
    let mut animal = Animal { weight: 100f64 };
    animal.feed(1f64);
    println!("weight: {}", animal.weight);

    feed(&mut animal);
    println!("weight: {}", animal.weight);

    // Cat
    let mut animal = Animal { weight: 100f64 };
    feed_cat_with_amount(&mut animal, 10f64);
    animal.feed(10f64);

    println!("weight: {}", animal.weight);

    // Duck
    // 😱 Uncomment below to see an error `the trait `Duck` is not implemented for `Animal``.

    // feed_duck_100(&mut animal);
    // println!("weight: {}", animal.weight);

    // 💁 To solve this error try add `impl Duck for Animal {}`

    // 🤔 But this traits as parameters won't need `impl`, like feed_duck_100 above.
    feed_duck_200(&mut animal);
    println!("weight: {}", animal.weight);
}
```

> 💡 More about [Generics-Bounds](https://doc.rust-lang.org/rust-by-example/generics/bounds.html)

![](/assets/kat.png) At this point you should able to read a lot of `Rust` code out there, let's [teardown ➠](./teardown.md).

## TODO

> parameterize traits

```rust
trait Combiner<A, B, C> {
    fn combine(a: &A, b: &B) -> C;
}
```
