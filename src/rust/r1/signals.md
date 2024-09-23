# Signals

## An example of `futures_signals`

> [https://github.com/Pauan/rust-signals]()

## Code

<tabs>
<tab label="main.rs">

```rust,edition2021
{{#include ../../../examples/r1/hello-rust-signals/src/main.rs}}
```

</tab>
<tab label="Cargo.toml">

```toml
{{#include ../../../examples/r1/hello-rust-signals/Cargo.toml}}
```

</tab>
</tabs>

## Output

```
Start with: Mutable(5)
Changing my_state to 10
After map: 15
Before async calculation: 15
Starting async calculation (x2) for value: 15
Async calculation (x2) completed for value: 15, result: 30
New value: Some(30)
```
