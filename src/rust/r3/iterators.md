# Iterators

## How to sum value by key in object list?

```rust,editable
use std::collections::HashMap;

fn main() {
    println!(
        "{:?}",
        vec![
            ("key1", 10),
            ("key2", 20),
            ("key1", 5),
            ("key3", 15),
            ("key2", 25),
        ]
        .into_iter()
        .fold(HashMap::new(), |mut acc, (key, value)| {
            *acc.entry(key).or_insert(0) += value;
            acc
        })
    );
}
```

## How's the different between `iter` and `into_iter`?

```rust,editable
fn main() {
    // iter will borrow.
    let numbers = vec![0,1,2,3,4,5];
    let sum = numbers.iter().sum::<u32>();
    println!("Sum of {numbers:?} is {sum}");

    //into_iter will take ownership.
    let numbers = vec![0,1,2,3,4,5];
    let sum = numbers.into_iter().sum::<u32>();

    // 😱 uncomment this line 👇 will get an error below.
    // println!("Sum of {numbers:?} is {sum}");
    //                   ^^^^^^^^^^^ value borrowed here after move
}
```
