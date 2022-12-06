# Trait

## Generic Trait Parameters

```rust,editable
use std::fmt::Display;

trait Logger<S: Display> {
    fn log(&self, value: S) {
        println!("{}", value);
    }
}

struct Console {}

impl<S: Display> Logger<S> for Console {}

fn main() {
    let foo = "Hello";
    let bar = 100u8;
    let console = Console {};

    console.log(foo);
    console.log(bar);
}
```

## Generic Trait for Generic Type

```rust,editable
use std::fmt::Display;

trait Console<S: Display> {
    fn log(&self, value: S) {
        println!("{value}");
    }
}

impl<S: Display, T> Console<S> for T {}

fn main() {
    let foo = "Hello world!";
    let bar = 100u8;

    bar.log(foo);
}
```

## How can I define a function with a parameter that can be multiple kinds of trait objects?

> 🤔 [refer to stack overflow](https://stackoverflow.com/questions/51247690/how-can-i-define-a-function-with-a-parameter-that-can-be-multiple-kinds-of-trait)

```rust,editable
trait Vehicle {}
trait Floating {}

struct Boat;
impl Vehicle for Boat {}
impl Floating for Boat {}

trait Shipyard {
    fn construct(boat: Boat) -> Box<Self>;
}

impl Shipyard for Boat {
    fn construct(boat: Boat) -> Box<Self> {
        Box::new(boat)
    }
}

impl Shipyard for dyn Vehicle {
    fn construct(boat: Boat) -> Box<dyn Vehicle> {
        Box::new(boat) as Box<dyn Vehicle>
    }
}

impl Shipyard for dyn Floating {
    fn construct(boat: Boat) -> Box<dyn Floating> {
        Box::new(boat) as Box<dyn Floating>
    }
}

fn main() {
    let mut a: Vec<Box<dyn Vehicle>> = vec![];
    populate(&mut a);

    let mut b: Vec<Box<dyn Floating>> = vec![];
    populate(&mut b);
}

fn populate<T: ?Sized>(receiver: &mut Vec<Box<T>>)
where
    T: Shipyard,
{
    receiver.push(T::construct(Boat));
}
```

> 💡 Read more [here](https://kerkour.com/rust-generics-traits)
