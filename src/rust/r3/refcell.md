# RefCell

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

## How is returning a mutable reference that is behind an immutable reference, passed as an argument to the function, handled?

> 🤔 [refer to stack overflow](https://stackoverflow.com/questions/52197812/returning-a-mutable-reference-that-is-behind-an-immutable-reference-passed-to-t)

```rust,editable
use std::cell::RefCell;
use std::ops::DerefMut;

struct Foo { i: i32 }

struct Bar<'b> {
    // store the data in a RefCell for interior mutability
    f: &'b RefCell<Foo>
}

impl<'a: 'b, 'b> Bar<'b> {
    // Return a RefMut smart pointer instead of mutable ref, but hide the implementation,
    // just exposing it as something that can be mutably dereferenced as a Foo
    fn func(&'a self) -> impl DerefMut<Target = Foo> + 'b {
         self.f.borrow_mut()
    }
}

fn main() {
    let foo = RefCell::new(Foo { i: 1 });
    let bar = Bar { f: &foo };

    let mut f = bar.func();
    f.i = 3;
}
```
