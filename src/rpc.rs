use reqwest::Client;
use std::env;
use std::time::Duration;

pub fn create_rpc_client() -> (Client, String) {
    let rpc_url = env::var("RPC_URL").unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string());
    let client = Client::builder()
        .timeout(Duration::from_secs(15))
        .build()
        .expect("Failed to create RPC client");
    (client, rpc_url)
}