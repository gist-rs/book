# BoxStream

## How to handle streaming response in Rust?

![](/assets/kat.png) <span class="speech-bubble">Let's try `futures::boxstream`</span>

## Setup

```bash
cargo init futures-boxstream
cd futures-boxstream
cargo add tokio --features "full"
cargo add futures
```

## Code

<tabs>
<tab label="main.rs">

```rust,no_run
{{#include ../../../examples/r3/futures-boxstream/src/main.rs}}
```

</tab>
<tab label="Cargo.toml">

```toml
{{#include ../../../examples/r3/futures-boxstream/Cargo.toml}}
```

</tab>
</tabs>

## Output

```
HELLO
WORLD
!
Number: 1
Error: Oops
Number: 3
```
