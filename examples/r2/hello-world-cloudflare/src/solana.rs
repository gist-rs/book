use anyhow::{anyhow, Result};
use currency_rs::{Currency, CurrencyOpts};
use serde::{Deserialize, Serialize};
use worker::{console_error, console_log};

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

pub async fn get_balance(pubkey: String, options: GetBalanceOptions) -> Result<UiBalance> {
    let client = reqwest::Client::new();
    let json_payload = RpcPayload {
        jsonrpc: "2.0".to_owned(),
        id: options.id,
        method: "getBalance".to_owned(),
        params: vec![pubkey],
    };

    let http_response = client
        .post(options.rpc_url)
        .json(&json_payload)
        .send()
        .await?;

    if !http_response.status().is_success() {
        let status = http_response.status();
        let error_text = http_response
            .text()
            .await
            .unwrap_or_else(|e| format!("Failed to read error body: {}", e));
        console_error!(
            "RPC request to {} failed with status: {} and body: {}",
            options.rpc_url,
            status,
            error_text
        );
        return Err(anyhow!(
            "RPC request failed with status: {} and body: {}",
            status,
            error_text
        ));
    }

    let response_text = http_response.text().await?;
    console_log!(
        "Raw RPC response from {}: {}",
        options.rpc_url,
        response_text
    );

    let balance_response: GetBalanceResponse =
        serde_json::from_str(&response_text).map_err(|e| {
            console_error!(
                "Failed to deserialize RPC response: {}. Response text from {} was: {}",
                e,
                options.rpc_url,
                response_text
            );
            anyhow!(
                "Error decoding RPC response body: {}. Raw response from {} was: {}",
                e,
                options.rpc_url,
                response_text
            )
        })?;

    let lamports = balance_response.result.value as f64 / 10u64.pow(9) as f64;
    let ui_lamports = Currency::new_float(lamports, options.currency_opts).format();

    Ok(UiBalance {
        lamports,
        ui_lamports,
    })
}
