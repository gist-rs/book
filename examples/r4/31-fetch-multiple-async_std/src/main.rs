use async_std::task;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct AnimalData {
    id: String,
    weight: f32,
    created_at: String,
}

async fn fetch_multiple_client_join_all(urls: Vec<String>) -> anyhow::Result<Vec<AnimalData>> {
    // ✨ How to use spawn_local.
    let handles = urls.into_iter().map(|url| {
        let client = Client::new();
        task::spawn_local(async move {
            let resp = client.get(url).send().await?;
            resp.json::<AnimalData>().await
        })
    });

    println!("handles:{:#?}", handles);

    let mut results = vec![];
    for handle in handles {
        results.push(handle.await);
    }

    println!("results:{:#?}", results);

    let mut errors = vec![];
    let result = results
        .into_iter()
        .filter_map(|e| e.map_err(|err| errors.push(err.to_string())).ok())
        .collect::<Vec<_>>();

    Ok(result)
}

#[tokio::main]
async fn main() {
    let sources = vec![
        "https://raw.githubusercontent.com/gist-rs/book/main/examples/r4/31-fetch-multiple-async_std/src/foo.json".to_owned(),
    "https://raw.githubusercontent.com/gist-rs/book/main/examples/r4/31-fetch-multiple-async_std/src/bar.json".to_owned()
    ];

    let json = fetch_multiple_client_join_all(sources).await;
    println!("{json:#?}");
}
