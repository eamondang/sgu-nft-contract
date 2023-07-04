use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct NFTContractMetadata {
  pub spec: String, // require
  pub name: String,
  pub symbol: String,
  pub icon: Option<String>,
  pub base_uri: Option<String>,
  pub reference: Option<String>,
  pub reference_hash: Option<String>, // Some(String) , None
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct TokenMetadata {
  pub title: Option<String>,
  pub description: Option<String>,
  pub media: Option<String>,
  pub media_hash: Option<String>,
  pub copies: Option<u64>,
  pub issued_at: Option<u64>,
  pub expires_at: Option<u64>,
  pub starts_at: Option<u64>,
  pub updated_at: Option<u64>,
  pub extra: Option<String>,
  pub reference: Option<String>,
  pub reference_hash: Option<String>,
}
