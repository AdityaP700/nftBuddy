use serde::Deserialize;
use mpl_token_metadata::state::Metadata;
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RpcResponse {
    pub result: RpcResult,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RpcResult {
    pub value: AccountInfo,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfo {
    pub data: (String, String),
    pub executable: bool,
    pub lamports: u64,
    pub owner: String,
    pub rent_epoch: u64,
}

#[derive(Deserialize, Debug)]
pub struct OffChainMetadata {
    pub name: String,
    pub symbol: String,
    pub image: String,
    pub attributes: Vec<Attribute>,
}

#[derive(Deserialize, Debug)]
pub struct Attribute {
    pub trait_type: String,
    pub value: String,
}
pub struct UnmaskReport{
    pub on_chain:Metadata,
    pub off_chain :OffChainMetadata,
}
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum AccountData {
    Base64(String),

}