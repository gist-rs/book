# Generic Array

## Problem

- We can't do this.

```rust,no_run
struct Foo<T, N> {
    data: [T; N]
}
```

## Solution

- Use [generic_array](https://docs.rs/generic-array/latest/generic_array/).

```rust,editable,edition2021
use generic_array::{ArrayLength, GenericArray};
use generic_array::typenum::U5;
use generic_array::*;

fn main() {
  #[derive(Debug)]
  struct Foo<T, N: ArrayLength<T>> {
      // We need this cause👇 is usually can't be done.
      data: GenericArray<T, N>
  }

  // By fn
  let foo = Foo::<i32, U5>{data: GenericArray::default()};

  // By macro
  let bar_data = arr![i32; 0, 0, 0, 0, 0];

  // Proof
  println!("{foo:?}");
  assert_eq!(foo.data, bar_data);
}
```
