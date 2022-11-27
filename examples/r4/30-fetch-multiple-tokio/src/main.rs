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

async fn fetch_multiple_with_one_client_join_all(urls: &[&str]) -> anyhow::Result<Vec<AnimalData>> {
    let client = Client::new();

    // ✨ How to use join_all.
    let results = future::join_all(urls.iter().map(|url| {
        let client = &client;
        async move {
            let resp = client.get(*url).send().await?;
            resp.json::<AnimalData>().await
        }
    }))
    .await;

    let result = results
        .into_iter()
        .map(|res| match res {
            Ok(json) => json,
            Err(err) => panic!("Error: {err}"),
        })
        .collect::<Vec<_>>();

    Ok(result)
}

#[tokio::main]
async fn main() {
    let sources = [
        "https://raw.githubusercontent.com/gist-rs/book/main/examples/r4/20-fetch-json-reqwest/src/foo.json",
    "https://raw.githubusercontent.com/gist-rs/book/main/examples/r4/20-fetch-json-reqwest/src/bar.json"
    ];

    let json = fetch_multiple_with_one_client_join_all(&sources).await;
    println!("{json:#?}");
}
