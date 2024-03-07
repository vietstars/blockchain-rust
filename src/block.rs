use crate::errors::Result;

use crypto::digest::Digest;
use crypto::sha2::Sha256;
use log::info;
use serde::{Serialize,Deserialize};
use std::time::SystemTime;

const TARGET_HEXT: usize = 4;
const GENESIS_COINBASE_DATA: &str = "The Times 03/Jan/2009 Chancellor on brink of second bailout for banks";

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Block {
  timestamp: u128,
  transactions: String,
  prev_block_hash: String,
  hash: String,
  height: usize,
  nonce: i32,
}

impl Block {

  pub(crate) fn get_prev_hash(&self) -> String {
    self.prev_block_hash.clone()
  }

  pub fn get_hash(&self) -> String {
    self.hash.clone()
  }

  pub fn new_genesis_block() -> Block {
    Block::new_block(String::from("Gensis Block"), String::from(GENESIS_COINBASE_DATA), 0).unwrap()
  }

  pub fn new_block(data: String, prev_block_hash: String, height: usize) -> Result<Block> {
    let timestamp = SystemTime::now()
      .duration_since(SystemTime::UNIX_EPOCH)?
      .as_millis();
    let mut block = Block {
      timestamp: timestamp,
      transactions: data,
      prev_block_hash,
      hash: String::new(),
      height,
      nonce: 0,
    };
    block.run_proof_if_work()?;
    Ok(block)
  }

  fn run_proof_if_work(&mut self) -> Result<()> {
    info!("Mining the block");
    while !self.validate()? {
      self.nonce += 1;
    }
    let data = self.prepare_hash_data()?;
    let mut hasher = Sha256::new();
    hasher.input(&data[..]);
    self.hash = hasher.result_str();
    Ok(())
  }

  fn prepare_hash_data(&self) -> Result<Vec<u8>> {
    let content = (
      self.prev_block_hash.clone(),
      self.transactions.clone(),
      self.timestamp,
      TARGET_HEXT,
      self.nonce
    );
    let bytes = bincode::serialize(&content)?;
    Ok(bytes)
  }

  fn validate(&self) -> Result<bool> {
    let data = self.prepare_hash_data()?;
    let mut hasher = Sha256::new();
    hasher.input(&data[..]);
    let mut vec1: Vec<u8> = vec![];
    vec1.resize(TARGET_HEXT, '0' as u8);
    Ok(&hasher.result_str()[0..TARGET_HEXT] == String::from_utf8(vec1)?)
  }
}

#[cfg(test)]
mod tests {
  use crate::block::Block;

  #[test]
  fn test_blockchain() {
    let b = Block::new_genesis_block();
    dbg!(b);
  }
}
