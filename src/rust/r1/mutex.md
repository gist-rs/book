# Mutex

## Problem

```rust,editable
use std::thread;

fn main() {
    let numbers = vec![1, 2, 3];

    thread::spawn(move || {
        for n in &numbers {
            println!("{n}");
        }
    }).join().unwrap();

    // 😱 This will error use of moved value: `numbers`
    thread::spawn(move || {
        //        ^^^^^^^ value used here after move
        println!("length: {}", numbers.len());
    }).join().unwrap();
}
```

## Solution

```rust,editable
use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    // Use Mutex to protect the shared vector
    let mutex_numbers = Mutex::new(vec![1, 2, 3]);

    // Use Arc here 👇 to share the numbers vector between threads
    let numbers = Arc::new(mutex_numbers);

    // We clone vector to pass it to thread
    let numbers_clone = Arc::clone(&numbers);
    thread::spawn(move || {
        // We lock 🔒 the vector to safely read/mod it.
        let numbers = numbers_clone.lock().unwrap();
        for n in &*numbers {
            println!("{:?}", n);
        }
    }).join().unwrap();

    // We can use numbers freely here.
    thread::spawn(move || {
        let numbers = numbers.lock().unwrap();
        println!("length: {}", numbers.len());
    }).join().unwrap();
}
```
