# Create your own lib

## How to implement `callback`.

> 🤔 Refer to : https://stackoverflow.com/questions/41081240/idiomatic-callbacks-in-rust

### 1️⃣ Store the `callback` as `FnMut` in the `Box` with the callback setter generic on callback type. 🤯

```rust,editable
type Callback = fn();

struct Processor {
    callback: Callback,
}

impl Processor {
    fn set_callback(&mut self, c: Callback) {
        self.callback = c;
    }

    fn process_events(&self) {
        (self.callback)();
    }
}

fn simple_callback() {
    println!("hello world!");
}

fn main() {
    let p = Processor {
        callback: simple_callback,
    };
    p.process_events(); // hello world!
}
```

### 2️⃣ Callbacks as generic function objects

```rust,editable
struct Processor<CB> {
    callback: CB,
}

impl<CB> Processor<CB>
where
    CB: FnMut(),
{
    fn set_callback(&mut self, c: CB) {
        self.callback = c;
    }

    fn process_events(&mut self) {
        (self.callback)();
    }
}

fn main() {
    let s = "world!".to_string();
    let callback = || println!("hello {}", s);
    let mut p = Processor { callback };
    p.process_events();
}
```

### 3️⃣ Non-generic callbacks: function trait objects

```rust,editable
struct Processor {
    callback: Box<dyn FnMut()>,
}

impl Processor {
    fn set_callback(&mut self, c: impl FnMut() + 'static) {
        self.callback = Box::new(c);
    }

    fn process_events(&mut self) {
        (self.callback)();
    }
}

fn simple_callback() {
    println!("hello");
}

fn main() {
    let mut p = Processor {
        callback: Box::new(simple_callback),
    };
    p.process_events();
    let s = "world!".to_string();
    let callback2 = move || println!("hello {}", s);
    p.set_callback(callback2);
    p.process_events();
}
```

### 4️⃣ Lifetime of references inside boxed closures

```rust,editable
struct Processor<'a> {
    callback: Box<dyn FnMut() + 'a>,
}

impl<'a> Processor<'a> {
    fn set_callback(&mut self, c: impl FnMut() + 'a) {
        self.callback = Box::new(c);
    }

    fn process_events(&mut self) {
        (self.callback)();
    }
}

fn simple_callback() {
    println!("hello");
}

fn main() {
    let mut p = Processor {
        callback: Box::new(simple_callback),
    };
    p.process_events();
    let s = "world!".to_string();
    let callback2 = move || println!("hello {}", s);
    p.set_callback(callback2);
    p.process_events();
}
```
