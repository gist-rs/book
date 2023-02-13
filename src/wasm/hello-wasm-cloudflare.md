# Hello Wasm Cloudflare

## Fun Facts

- You can use [`js`](https://github.com/cloudflare/workers-sdk/tree/main/fixtures) or [`rs`](https://github.com/cloudflare/workers-rs) or both `js`+`rs`. // `js` has more examples/features complete than `rs`.
- You can call worker from `pages`. // Think `CMS` with backend-less which can hydrate/dehydrate anywhere you desire.
- You can make and modify `request`/`response`.
- 🦀 You can fetch through `reqwest` or `worker::Fetch::Request`. // See example below. 👇

## How to fetch and response as json with `reqwest`

```rust,no_run
use worker::*;

pub async fn fetch_json(url: &str) -> anyhow::Result<serde_json::Value> {
    Ok(reqwest::get(url).await?.json().await?)
}

#[event(fetch)]
pub async fn main(_: Request, _: Env, _: Context) -> Result<Response> {
    let url = "https://raw.githubusercontent.com/gist-rs/book/main/examples/r4/20-fetch-json-reqwest/src/foo.json";
    let json_result = fetch_json(url).await;

    match json_result {
        Ok(value) => Response::ok(value.to_string()),
        Err(err) => Response::error(format!("${err}"), 500),
    }
}
```

> 💡 For more `reqwest` example see [here](https://book.gist.rs/rust/r4/mod.html)

## How to fetch with upgrade websocket via `worker::Fetch::Request`

> 🤔 Refer to [Cloudflare issue](https://github.com/cloudflare/workers-rs/issues/239#issuecomment-1295684938)

```rust,no_run
use worker::*;

#[event(fetch)]
pub async fn main(_: Request, _: Env, _: Context) -> Result<Response> {
    let url = "https://httpbin.org/anything";

    let mut headers = Headers::new();
    headers.set("Upgrade", "websocket")?;

    let request = Request::new_with_init(
        url,
        RequestInit::new().with_headers(headers)
    )?;

    Fetch::Request(request).send().await
}
```
