use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug)]
// This will auto convert form `camelCase` to `snake_case`
#[serde(rename_all = "camelCase")]
struct AnimalData {
    id: String,
    weight: f32,
    // This will auto convert form "createdAt" to `created_at`
    created_at: String,
}

// 👇 Derive from thiserror::Error
#[derive(Error, Debug)]
enum CustomError {
    #[error("Request failed: {0}")]
    RequestError(reqwest::Error),

    #[error("JSON parsing error: {0}")]
    JsonParseError(reqwest::Error),
}

async fn fetch(url: &str) -> Result<Vec<AnimalData>, CustomError> {
    let response = reqwest::get(url).await.map_err(CustomError::RequestError)?;
    let animals = response
        .json::<Vec<AnimalData>>()
        .await
        .map_err(CustomError::JsonParseError)?;

    Ok(animals)
}

#[tokio::main]
async fn main() {
    let animals = match fetch("https://raw.githubusercontent.com/gist-rs/book/main/examples/r4/20-fetch-json-reqwest/src/foo.json").await {
        Ok(animals) => {
            println!("{animals:#?}");
            // Will return parsed JSON as Vec<AnimalData> type.
            animals
        }
        Err(err) => {
            // Will yelling.
            println!("No animals!: {}", err);
            
            // Will return empty vector animals.
            Vec::from([])
        }
    };

    println!("{animals:#?}");
}
