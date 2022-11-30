# Trait

## How can I define a function with a parameter that can be multiple kinds of trait objects?

> 🤔 [refer to stack overflow](https://stackoverflow.com/questions/51247690/how-can-i-define-a-function-with-a-parameter-that-can-be-multiple-kinds-of-trait)

```rust,editable
#![feature(unsize)]

use std::marker::Unsize;

trait Shipyard {
    fn construct(boat: Boat) -> Box<Self>;
}

impl Shipyard for Boat {
    fn construct(boat: Boat) -> Box<Self> {
        Box::new(boat)
    }
}

impl<U: ?Sized> Shipyard for U
where
    Boat: Unsize<U>,
{
    fn construct(boat: Boat) -> Box<Self> {
        Box::new(boat) as Box<U>
    }
}

fn populate<T: ?Sized>(receiver: &mut Vec<Box<T>>)
where
    T: Shipyard,
{
    receiver.push(T::construct(Boat));
}
```

> 💡 Read more [here](https://kerkour.com/rust-generics-traits)
