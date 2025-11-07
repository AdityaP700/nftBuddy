use anyhow::{anyhow, Result};
use crate::rpc::create_rpc_client;
use crate::model::{DossierReport, CollectionSummary, HeliusAssetsResponse};
use serde_json::json;
use std::collections::HashMap;
use std::env;

pub async fn run(wallet_address: String) -> Result<DossierReport> {
    let (rpc_client, _) = create_rpc_client();

    let helius_api_key = env::var("HELIUS_API_KEY")
        .unwrap_or_else(|_| "".to_string());

    let helius_url = if helius_api_key.is_empty() {
        "https://api.mainnet-beta.solana.com".to_string()
    } else {
        format!("https://mainnet.helius-rpc.com/?api-key={}", helius_api_key)
    };

    let request_body = json!({
        "jsonrpc": "2.0",
        "id": "dossier-query",
        "method": "getAssetsByOwner",
        "params": {
            "ownerAddress": wallet_address,
            "page": 1,
            "limit": 1000,
            "displayOptions": {
                "showCollectionMetadata": true
            }
        }
    });

    let response = rpc_client
        .post(&helius_url)
        .json(&request_body)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!(
            "Helius API request failed with status: {}. Make sure HELIUS_API_KEY is set.",
            response.status()
        ));
    }

    let assets_response: HeliusAssetsResponse = response.json().await?;
    let assets = assets_response.result.items;
    let total_nfts = assets_response.result.total;

    let mut collection_counts: HashMap<String, u32> = HashMap::new();

    for asset in assets {
        let collection_name = if let Some(groupings) = asset.grouping {
            groupings
                .iter()
                .find(|g| g.group_key == "collection")
                .and_then(|g| Some(g.group_value.clone()))
                .or_else(|| {
                    // Fallback to metadata name
                    asset.content
                        .and_then(|c| c.metadata)
                        .and_then(|m| m.name)
                })
                .unwrap_or_else(|| "Unknown Collection".to_string())
        } else {
            "Unknown Collection".to_string()
        };

        *collection_counts.entry(collection_name).or_insert(0) += 1;
    }

    let mut collections: Vec<CollectionSummary> = collection_counts
        .into_iter()
        .map(|(name, count)| CollectionSummary { name, count })
        .collect();

    collections.sort_by(|a, b| b.count.cmp(&a.count));

    Ok(DossierReport {
        wallet_address,
        total_nfts,
        collections,
    })
}
