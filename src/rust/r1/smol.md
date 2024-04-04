## smol

> 🤔 Refer to [Why choose async/await over threads?](https://notgull.net/why-not-threads/)

## Don't

```rust
fn main() -> io::Result<()> {
    let socket = TcpListener::bind("0.0.0.0:80")?;

    loop {
        let (client, _) = socket.accept()?;
        thread::spawn(move || handle_client(client));
    }
}
```

## Do

- Use [async/await via smol](https://notgull.net/why-not-threads/)

```rust
#[apply(smol_macros::main!)]
async fn main(ex: &smol::Executor) -> io::Result<()> {
    let socket = TcpListener::bind("0.0.0.0:80").await?;

    loop {
        let (client, _) = socket.accept().await?;
        ex.spawn(async move {
            handle_client(client).await;
        }).detach();
    }
}
```
