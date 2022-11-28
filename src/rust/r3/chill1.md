# Chill Part 1

## How to move captured variable into a closure within a closure.

> 🤔 [refer to stack overflow](https://stackoverflow.com/questions/28521637/how-can-i-move-a-captured-variable-into-a-closure-within-a-closure)

### With `RefCell`

```rust,editable
fn main() {
    let mut seen = vec![];
    let items = vec![vec![1i32, 2], vec![3], vec![1]];

    let a: Vec<_> = items
    .iter()
    .flat_map(|inner_numbers| inner_numbers.iter())
    .filter_map(move |&number| {
        if !seen.contains(&number) {
            seen.push(number);
            Some(number)
        } else {
            None
        }
    })
    .collect();

    println!("{:?}", a);
}
```

### Without `RefCell`

```rust,editable
use std::cell::RefCell;

fn main() {
    let seen = vec![];
    let items = vec![vec![1i32, 2], vec![3], vec![1]];

    let seen_cell = RefCell::new(seen);

    let a: Vec<_> = items
        .iter()
        .flat_map(|inner_numbers| {
            inner_numbers.iter().filter_map(|&number| {
                let mut borrowed = seen_cell.borrow_mut();

                if !borrowed.contains(&number) {
                    borrowed.push(number);
                    Some(number)
                } else {
                    None
                }
            })
        })
        .collect();

    println!("{:?}", a);
}
```
