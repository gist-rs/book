use currency_rs::{Currency, CurrencyOpts};
use rand::Rng;
use serde::{Deserialize, Serialize};

// We need this lifetime 👇.
struct GetBalanceOptions<'a> {
    rpc_url: &'static str,
    id: u32,
    // Because of this options 👇 required it.
    currency_opts: Option<CurrencyOpts<'a>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RpcPayload {
    jsonrpc: String,
    id: u32,
    method: String,
    params: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GetBalanceResponse {
    jsonrpc: String,
    id: u32,
    result: GetBalanceResponseResult,
}

#[derive(Debug, Serialize, Deserialize)]
struct GetBalanceResponseResult {
    value: u64,
}

#[derive(Debug)]
#[allow(dead_code)]
struct UiBalance {
    lamports: f64,
    ui_lamports: String,
}

// We need this 'a  👇
async fn get_balance<'a>(
    pubkey: String,
    // But this can be elided 'a👇
    options: GetBalanceOptions<'_>,
) -> anyhow::Result<UiBalance> {
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
            rpc_url: "https://rpc.ankr.com/solana",
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
