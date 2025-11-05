use anyhow::{anyhow, Result};
use crate::rpc::create_rpc_client;
use serde_json::{json, Value};
use base64::engine::general_purpose::STANDARD;
use base64::Engine as _;
use mpl_token_metadata::state::{TokenMetadataAccount, Metadata};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use crate::model::{OffChainMetadata,UnmaskReport};
pub async fn run(mint_address:String)->Result<UnmaskReport>{
    println!("infiltarting network for target:{}",mint_address);

    let (rpc_client, rpc_url) = create_rpc_client();
    // Derive the metadata PDA for the mint so we query the metadata account (where URI lives).
    let mint_pubkey = Pubkey::from_str(&mint_address)
        .map_err(|e| anyhow!("Invalid mint address '{}': {}", mint_address, e))?;

    // Metaplex Token Metadata program id (canonical)
    let metadata_program_id = Pubkey::from_str("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")?;

    let seeds: &[&[u8]] = &[
        b"metadata",
        metadata_program_id.as_ref(),
        mint_pubkey.as_ref(),
    ];

    let (metadata_pubkey, _bump) = Pubkey::find_program_address(seeds, &metadata_program_id);

    let message = json!({
        "jsonrpc":"2.0",
        "id":1,
        "method":"getAccountInfo",
        "params":[
            metadata_pubkey.to_string(),
            {
                "encoding":"base64"
            }
        ]
    });

    // Send RPC and read raw text so we can inspect unexpected responses (errors, rate limits, etc.)
    let resp_text = rpc_client
        .post(&rpc_url)
        .json(&message)
        .send()
        .await?
        .text()
        .await?;

    // Parse into a Value so we can handle both success and error shapes gracefully.
    let v: Value = serde_json::from_str(&resp_text)
        .map_err(|e| anyhow!("Failed to parse RPC response as JSON: {}\nResponse text: {}", e, resp_text))?;

    // Check for JSON-RPC error first
    if let Some(err) = v.get("error") {
        return Err(anyhow!("RPC returned error: {}", err));
    }

    let result = v.get("result").ok_or_else(|| anyhow!("Missing `result` field in RPC response: {}", resp_text))?;

    // If metadata PDA doesn't exist, the RPC returns value: null. Try a helpful fallback check.
    if result.get("value").map(|v| v.is_null()).unwrap_or(true) {
        // Try fetching the mint account itself so we can tell the user whether the mint exists.
        println!("Metadata PDA {} returned null. Checking the mint account directly...", metadata_pubkey);

        let fallback_msg = json!({
            "jsonrpc":"2.0",
            "id":1,
            "method":"getAccountInfo",
            "params":[
                mint_pubkey.to_string(),
                { "encoding":"base64" }
            ]
        });

        let fb_text = rpc_client
            .post(&rpc_url)
            .json(&fallback_msg)
            .send()
            .await?
            .text()
            .await?;

        let fb_v: Value = serde_json::from_str(&fb_text)
            .unwrap_or(Value::String(fb_text.clone()));

        return Err(anyhow!("Metadata PDA {} not found. Fallback mint query returned: {}", metadata_pubkey, fb_v));
    }

    // Extract the encrypted data scroll.
    let base64_data = result
        .get("value")
        .and_then(|val| val.get("data"))
        .and_then(|d| d.get(0))
        .and_then(|s| s.as_str())
        .ok_or_else(|| anyhow!("Failed to find account data in RPC result: {}", resp_text))?;

    let bytes = STANDARD.decode(base64_data)
        .map_err(|e| anyhow!("base64 decode failed: {}", e))?;
    let metadata = Metadata::safe_deserialize(&mut bytes.as_slice())
        .map_err(|e| anyhow!("Failed to parse metadata from bytes: {}", e))?;

    let mut off_chain_uri = metadata.data.uri.trim_end_matches('\0').trim().to_string();

    // Resolve common schemes and guard against empty/invalid URIs
    if off_chain_uri.is_empty() {
        println!("No off-chain URI present in metadata. Using on-chain info only.");
        let off_chain_fallback = OffChainMetadata {
            name: metadata.data.name.trim_end_matches('\0').trim().to_string(),
            symbol: metadata.data.symbol.trim_end_matches('\0').trim().to_string(),
            image: String::new(),
            attributes: Vec::new(),
        };
        let report = UnmaskReport { on_chain: metadata, off_chain: off_chain_fallback };
        // Mirror existing prints for continuity
        println!("\n[MISSION COMPLETE - PARTIAL DOSSIER]");
        println!("\n--- ON-CHAIN DATA ---");
        println!("👑 Update Authority: {}", report.on_chain.update_authority);
        println!("🔒 Mutable: {}", report.on_chain.is_mutable);
        println!("\n--- OFF-CHAIN DATA ---");
        println!("📛 Name: {}", report.off_chain.name);
        println!("🖼️  Image URL: {}", report.off_chain.image);
        println!("\n✨ Attributes:");
        for attr in &report.off_chain.attributes { println!("   - {}: {}", attr.trait_type, attr.value); }
        return Ok(report);
    }

    // Handle IPFS URIs
    if off_chain_uri.starts_with("ipfs://") {
        off_chain_uri = off_chain_uri.replace("ipfs://", "https://ipfs.io/ipfs/");
    } else if off_chain_uri.starts_with("ar://") {
        off_chain_uri = off_chain_uri.replacen("ar://", "https://arweave.net/", 1);
    } else if off_chain_uri.starts_with('/') {
        // Relative arweave path
        off_chain_uri = format!("https://arweave.net{}", off_chain_uri);
    }

    if !(off_chain_uri.starts_with("http://") || off_chain_uri.starts_with("https://")) {
        println!("Off-chain URI '{}' is not HTTP(S). Using on-chain info only.", off_chain_uri);
        let off_chain_fallback = OffChainMetadata {
            name: metadata.data.name.trim_end_matches('\0').trim().to_string(),
            symbol: metadata.data.symbol.trim_end_matches('\0').trim().to_string(),
            image: String::new(),
            attributes: Vec::new(),
        };
        let report = UnmaskReport { on_chain: metadata, off_chain: off_chain_fallback };
        println!("\n[MISSION COMPLETE - PARTIAL DOSSIER]");
        println!("\n--- ON-CHAIN DATA ---");
        println!("👑 Update Authority: {}", report.on_chain.update_authority);
        println!("🔒 Mutable: {}", report.on_chain.is_mutable);
        println!("\n--- OFF-CHAIN DATA ---");
        println!("📛 Name: {}", report.off_chain.name);
        println!("🖼️  Image URL: {}", report.off_chain.image);
        println!("\n✨ Attributes:");
        for attr in &report.off_chain.attributes { println!("   - {}: {}", attr.trait_type, attr.value); }
        return Ok(report);
    }

    println!("Found breadcrumb. Infiltrating off-chain location: {}", off_chain_uri);
    let off_chain_response = rpc_client
        .get(off_chain_uri)
        .send()
        .await?
        .json::<OffChainMetadata>()
        .await?;

    let report = UnmaskReport {
        on_chain: metadata,
        off_chain: off_chain_response,
    };
    println!("\n[MISSION COMPLETE - FULL DOSSIER]");
    println!("\n--- ON-CHAIN DATA ---");
    println!("👑 Update Authority: {}", report.on_chain.update_authority);
    println!("🔒 Mutable: {}", report.on_chain.is_mutable);
    println!("\n--- OFF-CHAIN DATA ---");
    println!("📛 Name: {}", report.off_chain.name);
    println!("🖼️  Image URL: {}", report.off_chain.image);
    println!("\n✨ Attributes:");
    for attr in &report.off_chain.attributes { println!("   - {}: {}", attr.trait_type, attr.value); }
    Ok(report)
}