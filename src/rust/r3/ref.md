# ref

> 🤔 [refer to Will Crichton](https://twitter.com/wcrichton/status/1597318042244915201)

```rust,editable
enum AnEnum { Branch(String) }

let x = AnEnum::Branch(String::new());

// Does not move x
match x { _ => {} }

// Moves x
match x { y => {} }

// 😱 Uncomment this to move on.
// let x = AnEnum::Branch(String::new());

// Does not move x
match x { AnEnum::Branch(_) => {} }

// Moves x
match x { AnEnum::Branch(y) => {} }

// 😱 Uncomment this to move on.
// let x = AnEnum::Branch(String::new());

// Does not move x
match x { AnEnum::Branch(ref y) => {} }
```
