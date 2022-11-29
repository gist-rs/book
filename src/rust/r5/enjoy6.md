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

### Generic Bounds

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

fn feed<T: Feedable>(t: &mut T) {
    t.feed(1f64)
}

// Cat
trait Catty {}

impl Catty for Animal {}

fn feed_cat_with_amount<T: Feedable + Catty>(t: &mut T, amount: f64) {
    t.feed(amount)
}

// Duck
trait Ducky {}

#[allow(dead_code)]
fn feed_duck_100<T: Feedable + Ducky>(t: &mut T) {
    t.feed(100f64)
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
    // 😱 Uncomment below to see an error `the trait `Ducky` is not implemented for `Animal``.

    // feed_duck_100(&mut animal);
    // println!("weight: {}", animal.weight);

    // 💁‍♂️ To solve this error try add `impl Ducky for Animal {}`
}

```

![](/assets/kat.png) At this point you should able to read a lot of `Rust` code out there, let's [teardown ➠](./teardown.md).
