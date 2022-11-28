use futures::future;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct AnimalData {
    id: String,
    weight: f32,
    created_at: String,
}

// Shared client for each call.
async fn fetch_multiple_with_one_client_join_all(urls: &[&str]) -> anyhow::Result<Vec<AnimalData>> {
    // ✨ New shared client once.
    let client = Client::new();

    // ✨ How to use join_all.
    let results = future::join_all(urls.iter().map(|&url| {
        // ✨ Use shared client.
        let client = &client;
        async move {
            let resp = client.get(url).send().await?;
            resp.json::<AnimalData>().await
        }
    }))
    .await;

    // ✨ Return flattened results, silent if error.
    Ok(results
        // We use into_iter so we get Vec<AnimalData> instead of Vec<&AnimalData>
        .into_iter()
        .flatten()
        .collect::<Vec<_>>())
}

// Each client for each call.
async fn fetch_multiple_with_each_client_join_all(
    urls: &[&str],
) -> anyhow::Result<Vec<AnimalData>> {
    // ✨ How to use join_all.
    let results = future::join_all(urls.iter().map(|&url| async move {
        // Fetch each url with new client.
        reqwest::get(url).await?.json::<AnimalData>().await
    }))
    .await;

    // ✨ Return flattened results, silent if error.
    Ok(results
        // We use into_iter so we get Vec<AnimalData> instead of Vec<&AnimalData>
        .into_iter()
        .flatten()
        .collect::<Vec<_>>())
}

#[tokio::main]
async fn main() {
    let urls = [
        "https://raw.githubusercontent.com/gist-rs/book/main/examples/r4/30-fetch-multiple-futures/src/foo.json",
    "https://raw.githubusercontent.com/gist-rs/book/main/examples/r4/30-fetch-multiple-futures/src/bar.json"
    ];

    let json = fetch_multiple_with_one_client_join_all(&urls).await;
    println!("{json:#?}");

    let json = fetch_multiple_with_each_client_join_all(&urls).await;
    println!("{json:#?}");
}
