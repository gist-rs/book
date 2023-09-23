# Serde

## Using serde with a field that can be an uuid or an empty string

> 🤔 [refer to reddit](https://www.reddit.com/r/learnrust/comments/z9jlvy/using_serde_with_a_field_that_can_be_an_uuid_or/)

```rust,no_run
use serde::Deserialize;
use std::convert::TryFrom;
use uuid::Uuid;

#[derive(Deserialize)]
struct Foo {
    parent: Id
}

#[derive(Deserialize)]
#[serde(try_from = "String")]
enum Id {
    Root,
    Uuid(Uuid)
}

impl TryFrom<String> for Id {
    type Error = uuid::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value == "" {
            Ok(Self::Root)
        } else {
            let uuid = Uuid::parse_str(&value)?;
            Ok(Self::Uuid(uuid))
        }
    }
}
```

> 💡 [Read more](https://serde.rs/container-attrs.html#try_from)

![](/assets/kat.png) <span class="speech-bubble">[borsh](https://borsh.io/) is alternative to `serde`.</span>
