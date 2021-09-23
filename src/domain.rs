extern crate serde;
extern crate serde_json;

use chrono::Utc;
use crypto_hash::{hex_digest, Algorithm};
use std::collections::HashMap;

static DIFFICULTY: usize = 5;

fn get_current_timestamp() -> i64 {
  let date_time = Utc::now();

  return date_time.timestamp();
}

fn create_hash(input: String) -> String {
  return hex_digest(Algorithm::SHA256, input.as_bytes());
}

pub trait HashableBlock {
  fn new(data: Vec<Transaction>, previous_block_hash: String) -> Self;
  fn create_genesis_block() -> Self;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
  pub from_address: String,
  pub to_address: String,
  pub amount: u32,
}

#[derive(Debug)]
pub struct Block {
  pub hash: String,
  created_at: i64, // Timestamp
  data: Vec<Transaction>,
  previous_block_hash: String,
  nonce: i32,
}

pub type Blockchain = Vec<Block>;
pub type Blockmap = HashMap<String, Block>;

#[derive(Debug)]
pub struct Lithium {
  pub chain: Blockchain,
  pub map: Blockmap,
  pub pending_transactions: Vec<Transaction>,
}

impl HashableBlock for Block {
  fn new(data: Vec<Transaction>, previous_block_hash: String) -> Block {
    let mut nonce: i32 = 0;
    let mut hash: String = String::from("");
    let mut substr: String = String::from("");

    while substr != String::from("00000") {
      nonce = nonce + 1;

      let mut input: String = String::from("");

      for transaction in data.iter() {
        let serialied = serde_json::to_string(&transaction).unwrap();
        input.push_str(serialied.as_str());
      }

      input.push_str(nonce.to_string().as_str());

      hash = create_hash(input);
      substr = hash.chars().take(DIFFICULTY).collect();
    }

    return Block {
      hash: hash,
      created_at: get_current_timestamp(),
      data: data,
      previous_block_hash: previous_block_hash,
      nonce: nonce,
    };
  }

  fn create_genesis_block() -> Block {
    return Self::new(vec![], "LITHIUM".to_owned());
  }
}
