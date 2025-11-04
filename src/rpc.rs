use reqwest::Client;
use std::time::Duration;

const RPC_URL : &str = "https://api.mainnet-beta.solana.com";
pub fn create_rpc_client()->Client{
    Client::builder()
    .timeout(Duration::from_secs(10))
    .build()
    .expect("Failed to create RPC client")
}