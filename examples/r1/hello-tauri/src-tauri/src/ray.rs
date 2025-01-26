use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const SOL_JLP_POOL_ID: &str = "3d8ksMPuLpaQAUbuRr74tmovmyFFXgAsC3iE5NhsgvnH";
pub const RAYDIUM_BASE_API: &str = "https://api-v3.raydium.io";

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PoolInfoResponse {
    pub id: String,
    pub success: bool,
    pub data: Vec<PoolData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PoolData {
    pub r#type: String,
    pub program_id: String,
    pub id: String,
    pub mint_a: Mint,
    pub mint_b: Mint,
    pub reward_default_pool_infos: String,
    pub reward_default_infos: Vec<String>,
    pub price: f64,
    pub mint_amount_a: f64,
    pub mint_amount_b: f64,
    pub fee_rate: f64,
    pub open_time: String,
    pub tvl: f64,
    pub day: TimeFrameData,
    pub week: TimeFrameData,
    pub month: TimeFrameData,
    pub pooltype: Vec<String>,
    pub farm_upcoming_count: u32,
    pub farm_ongoing_count: u32,
    pub farm_finished_count: u32,
    pub config: Config,
    pub burn_percent: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Mint {
    pub chain_id: u32,
    pub address: String,
    pub program_id: String,
    #[serde(rename = "logoURI")]
    pub logo_uri: String,
    pub symbol: String,
    pub name: String,
    pub decimals: u8,
    pub tags: Vec<String>,
    pub extensions: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TimeFrameData {
    pub volume: f64,
    pub volume_quote: f64,
    pub volume_fee: f64,
    pub apr: f64,
    pub fee_apr: f64,
    pub price_min: f64,
    pub price_max: f64,
    pub reward_apr: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub id: String,
    pub index: u32,
    pub protocol_fee_rate: u32,
    pub trade_fee_rate: u32,
    pub tick_spacing: u32,
    pub fund_fee_rate: u32,
    pub default_range: f64,
    pub default_range_point: Vec<f64>,
}

async fn fetch(url: &str) -> anyhow::Result<PoolInfoResponse> {
    let json = reqwest::get(url).await?.json::<PoolInfoResponse>().await?;

    Ok(json)
}

#[allow(dead_code)]
async fn fetch_pool_info_by_id(id: &str) -> anyhow::Result<PoolData> {
    let pool_info = fetch(format!("{RAYDIUM_BASE_API}/pools/info/ids?ids={id}").as_str()).await;

    Ok(pool_info?.data[0].clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_pool_info_by_id() {
        let id = SOL_JLP_POOL_ID;
        let pool_info = fetch_pool_info_by_id(id).await;

        // Result
        println!("{pool_info:#?}");

        // Get price from pool that match id
        let price = pool_info.unwrap().price;

        println!("{price:#?}");
        assert!(price > 0.0);
    }
}
