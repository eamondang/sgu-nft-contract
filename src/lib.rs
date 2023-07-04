pub mod metadata; // module

use metadata::{NFTContractMetadata, Token, TokenMetadata};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Contract {
  pub owner_id: AccountId,
  pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<String>>,
  pub token_by_id: LookupMap<String, Token>,
  pub token_metadata_by_id: UnorderedMap<String, TokenMetadata>,
  pub metadata: Option<NFTContractMetadata>,
}

#[near_bindgen]
impl Contract {
  #[init]
  pub fn new() -> Self {
    let metdata = NFTContractMetadata {
      spec: "nft-1.0.0".to_string(),
      name: "SGU Bootcamp Contract".to_string(),
      symbol: "SGUBC".to_string(),
      icon: None,
      base_uri: None,
      reference: None,
      reference_hash: None,
    };

    Self {
      owner_id: env::signer_account_id(),
      tokens_per_owner: LookupMap::new(b"token_per_owner".to_vec()),
      token_by_id: LookupMap::new(b"token_by_id".to_vec()),
      token_metadata_by_id: UnorderedMap::new(b"token_metadata_by_id".to_vec()),
      metadata: Some(metdata),
    }
  }

  #[payable]
  pub fn mint(&mut self, token_id: String, title: Option<String>, description: Option<String>, media: Option<String>) {
    let tokenmetadat = TokenMetadata { title, description, media };
    let token = Token { token_id: token_id.clone(), owner_id: env::signer_account_id(), metadata: tokenmetadat };
    self.token_by_id.insert(&token_id, &token);
    // self.token_per_owner => insert
    // self.token_metadata_by_id => insert
  }

  pub fn get_token_by_id(&self, token_id: String) -> Token {
    self.token_by_id.get(&token_id).unwrap()
  }

  pub fn get_token_per_owner() {}
  pub fn get_token_metadata_by_id() {}
}
