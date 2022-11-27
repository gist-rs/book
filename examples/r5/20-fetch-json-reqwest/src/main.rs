use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
// ✨ This will auto convert form `camelCase` to `snake_case`
#[serde(rename_all = "camelCase")]
struct AnimalData {
    id: String,
    weight: f32,
    // ✨ This will auto convert form "createdAt" to `created_at`
    created_at: String,
}

// 👇 How to async fetch.   // 👇 How to use anyhow.
async fn fetch(url: &str) -> anyhow::Result<Vec<AnimalData>> {
    // ✨ Beware, This will new client every fetch.
    let json = Client::new()
        .get(url)
        .send()
        // ✨ Use `?` to unwrap fetch Result and return Error to anyhow above if has
        .await?
        // ✨ Parse to vec of AnimalData.
        .json::<Vec<AnimalData>>()
        // Use `?` to unwrap json parse Result and return Error to anyhow above if has
        .await?;

    Ok(json)
}

// ✨ How to async main.
#[tokio::main]
async fn main() {
    let json = fetch("https://raw.githubusercontent.com/gist-rs/book/main/examples/r5/20-fetch-json-reqwest/src/foo.json").await;
    println!("{json:#?}");
}
