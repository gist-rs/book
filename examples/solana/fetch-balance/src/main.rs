use currency_rs::{Currency, CurrencyOpts};
use rand::Rng;
use serde::{Deserialize, Serialize};

pub struct GetBalanceOptions {
    pub rpc_url: &'static str,
    pub id: u32,
    pub currency_opts: Option<CurrencyOpts>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcPayload {
    pub jsonrpc: String,
    pub id: u32,
    pub method: String,
    pub params: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBalanceResponse {
    pub jsonrpc: String,
    pub id: u32,
    pub result: GetBalanceResponseResult,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBalanceResponseResult {
    pub value: u64,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct UiBalance {
    pub lamports: f64,
    pub ui_lamports: String,
}

async fn get_balance(pubkey: String, options: GetBalanceOptions) -> anyhow::Result<UiBalance> {
    let client = reqwest::Client::new();
    let json_payload = RpcPayload {
        jsonrpc: "2.0".to_owned(),
        id: options.id,
        method: "getBalance".to_owned(),
        params: vec![pubkey],
    };

    // We can deserialize json by 👇 specify type
    let balance_response: GetBalanceResponse = client
        .post(options.rpc_url)
        .json(&json_payload)
        .send()
        .await?
        .json()
        .await?;

    let lamports = balance_response.result.value as f64 / 10u64.pow(9) as f64;
    let ui_lamports = Currency::new_float(lamports, options.currency_opts).format();

    Ok(UiBalance {
        lamports,
        ui_lamports,
    })
}

// We need tokio main here due to async.
#[tokio::main]
async fn main() {
    let ui_balance_result = get_balance(
        // Consider donate some SOL to this 👇 account to see some number show up 😆
        "gistmeAhMG7AcKSPCHis8JikGmKT9tRRyZpyMLNNULq".to_owned(),
        GetBalanceOptions {
            rpc_url: "https://api.mainnet-beta.solana.com",
            id: rand::thread_rng().gen_range(0u32..u32::MAX),
            currency_opts: Some(
                CurrencyOpts::new()
                    .set_precision(2)
                    .set_symbol("")
                    .set_separator(",")
                    .set_decimal("."),
            ),
        },
    )
    .await;

    // Consider use match to handle this 👇 as a homework.
    println!("ui_balance_result: {:?}", ui_balance_result);
}
