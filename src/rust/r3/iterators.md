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
