# Callback

## How to implement `callback`.

![](/assets/kat.png) <span class="speech-bubble">Below is simple `callback`.</span>

```rust,editable
fn callback<F>(say: F, message: &str)
where
    // 👇 We use `FnOnce` here because...
    F: FnOnce(String), // 👈 We not mutate anything and call only once.
{
    // Just saying. 👇 `to_owned` or `to_string` can be use.
    say(message.to_owned());

    // 😱 uncomment to see scary error.
    // say(message.to_owned());
}

fn main() {
    // callback the say(message)
    callback(|msg| {
        println!("Say: {}", msg);
    }, "Hi!");
}
```

> 🤔 Refer to : https://stackoverflow.com/questions/41081240/idiomatic-callbacks-in-rust

![](/assets/kat.png) <span class="speech-bubble">Now we will use `struct` to hold the `callback`.</span>

### 1️⃣ "Function pointers": `callbacks` as `fn`

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
        (self.callback)(); // 👈 Use parentheses to call the function.
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

![](/assets/kat.png) <span class="speech-bubble">Let's make it more generic.</span>

### 2️⃣ Callbacks as generic function objects

```rust,editable
struct Processor<CB> {
    callback: CB,
}

impl<CB> Processor<CB>
where
    // 👇 We use `FnMut` here because...
    CB: FnMut(), // 👈 `FnOnce` is call only once, `Fn` tend to readonly.
{
    fn set_callback(&mut self, c: CB) {
        self.callback = c;
    }

    // This mutating self 👇 because `FnMut`
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

![](/assets/kat.png) <span class="speech-bubble">It's generic but callback is not, let's go deeper.</span>

### 3️⃣ Non-generic callbacks: function trait objects

```rust,editable
struct Processor {
    // We put previous `FnMut` into the `Box`
    callback: Box<dyn FnMut()>, // And `dyn` for more dynamic fn.
}

impl Processor {
    // We will need lifetime bound on the type here 👇
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

![](/assets/kat.png) <span class="speech-bubble">Nicer but `'static` is too much, let's fix it.</span>

### 4️⃣ Lifetime of references inside boxed closures

```rust,editable
// Now we use `'a` 👇 here instead.
struct Processor<'a> {
    callback: Box<dyn FnMut() + 'a>, // 👈 Also here
}

// Now 👇 this look messy (for good)
impl<'a> Processor<'a> {
    fn set_callback(&mut self, c: impl FnMut() + 'a) {// 👈 Also here
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
