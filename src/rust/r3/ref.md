# ref

> 🤔 [refer to Will Crichton](https://twitter.com/wcrichton/status/1597318042244915201)

```rust,editable
fn main (){
  enum AnEnum { Branch(String) }

  let x = AnEnum::Branch(String::new());

  // Does not move x
  match x { _ => {} }

  // Moves x
  match x { _y => {} }

  // 💁 Uncomment this to get rid of error.
  // let x = AnEnum::Branch(String::new());

  // Does not move x
  match x { AnEnum::Branch(_) => {} }

  // Moves x
  match x { AnEnum::Branch(_y) => {} }

  // 💁 Uncomment this to get rid of error.
  // let x = AnEnum::Branch(String::new());

  // Does not move x
  match x { AnEnum::Branch(ref _y) => {} }
}
```
