# W4 - Basic

## SystemTime

### Before

```rust
use std::time::SystemTime;

let now = SystemTime::now()
    .duration_since(SystemTime::UNIX_EPOCH)?
    .as_secs();
```

### After

```rust
use instant::SystemTime;

let now = SystemTime::now()
    .duration_since(SystemTime::UNIX_EPOCH)?
    .as_secs();
```
