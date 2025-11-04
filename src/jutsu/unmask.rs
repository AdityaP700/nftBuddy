use anyhow::Result;
use crate::rpc::create_rpc_client;
use serde_json::json;

pub async fn run(mint_address:String)->Result<()>{
    println!("infiltarting network for target:{}",mint_address);

    let rpc_client= create_rpc_client();
    let message=json!({
        "jsonrpc":"2.0",
        "id":1,
        "method":"getAccountInfo",
        "params":[
            mint_address,
            {
                "encoding":"base64"
            }
        ]
    });

    let response_text = rpc_client.post("https://api.mainnet-beta.solana.com")
    .json(&message)
    .send()
    .await?
    .text()
    .await?;
    
    println!("Response detected from the shadows:");
    println!("Response: {}", response_text);
    Ok(())
}