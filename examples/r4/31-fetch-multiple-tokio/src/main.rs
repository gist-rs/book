use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::task;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct AnimalData {
    id: String,
    weight: f32,
    created_at: String,
}

async fn fetch(url: &str) -> anyhow::Result<AnimalData> {
    Ok(Client::new()
        .get(url)
        .send()
        .await?
        .json::<AnimalData>()
        .await?)
}

async fn fetch_multiple(urls: [&'static str; 2]) -> anyhow::Result<Vec<AnimalData>> {
    // ✨ How to use task::spawn
    let tasks = urls.iter().map(|url| task::spawn(fetch(url)));

    // ✨ Collect task result.
    let mut results = vec![];
    for task in tasks {
        results.push(task.await);
    }

    // Return flatten results.
    Ok(results
        // We use into_iter so we get Vec<AnimalData> instead of Vec<&AnimalData>
        .into_iter()
        // ✨ Ignore task join result
        .flatten()
        // ✨ Ignore fetch result
        .flatten()
        // Finally get it.
        .collect::<Vec<AnimalData>>())
}

#[tokio::main]
async fn main() {
    let urls:[&'static str;2] = [
        "https://raw.githubusercontent.com/gist-rs/book/main/examples/r4/31-fetch-multiple-tokio/src/foo.json",
    "https://raw.githubusercontent.com/gist-rs/book/main/examples/r4/31-fetch-multiple-tokio/src/bar.json" 
    ];

    let json = fetch_multiple(urls).await;
    println!("{json:#?}");
}
