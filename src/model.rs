use serde::Deserialize;
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Deserialize, Debug)]
pub struct OffChainMetadata {
    pub name: String,
    #[allow(dead_code)]
    pub symbol: String,
    pub image: String,
    pub attributes: Vec<Attribute>,
}


#[derive(Deserialize, Debug)]
pub struct Attribute {
    pub trait_type: String,
    pub value: String,
}

// Simplified Metadata struct for mpl-token-metadata v3.2
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct Metadata {
    pub key: u8,
    pub update_authority: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub data: Data,
    pub primary_sale_happened: bool,
    pub is_mutable: bool,
    pub edition_nonce: Option<u8>,
    pub token_standard: Option<u8>,
    pub collection: Option<Collection>,
    pub uses: Option<Uses>,
    pub collection_details: Option<CollectionDetails>,
    pub programmable_config: Option<ProgrammableConfig>,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct Data {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub seller_fee_basis_points: u16,
    pub creators: Option<Vec<Creator>>,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct Creator {
    pub address: solana_sdk::pubkey::Pubkey,
    pub verified: bool,
    pub share: u8,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct Collection {
    pub verified: bool,
    pub key: solana_sdk::pubkey::Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct Uses {
    pub use_method: u8,
    pub remaining: u64,
    pub total: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum CollectionDetails {
    V1 { size: u64 },
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum ProgrammableConfig {
    V1 {
        rule_set: Option<solana_sdk::pubkey::Pubkey>,
    },
}

pub struct UnmaskReport{
    pub on_chain: Metadata,
    pub off_chain: OffChainMetadata,
    pub image_data: Option<Vec<u8>>,
}

#[derive(Deserialize, Debug)]
pub struct HeliusAssetsResponse {
    pub result: HeliusAssetsList,
}

#[derive(Deserialize, Debug)]
pub struct HeliusAssetsList {
    pub items: Vec<HeliusAsset>,
    pub total: u32,
}

#[derive(Deserialize, Debug)]
pub struct HeliusAsset {
    #[allow(dead_code)]
    pub id: String,
    pub content: Option<HeliusContent>,
    pub grouping: Option<Vec<HeliusGrouping>>,
}

#[derive(Deserialize, Debug)]
pub struct HeliusContent {
    pub metadata: Option<HeliusMetadata>,
}

#[derive(Deserialize, Debug)]
pub struct HeliusMetadata {
    pub name: Option<String>,
    #[allow(dead_code)]
    pub symbol: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct HeliusGrouping {
    pub group_key: String,
    pub group_value: String,
}

pub struct DossierReport {
    pub wallet_address: String,
    pub total_nfts: u32,
    pub collections: Vec<CollectionSummary>,
}

pub struct CollectionSummary {
    pub name: String,
    pub count: u32,
}