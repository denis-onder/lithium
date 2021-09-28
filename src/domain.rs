extern crate rand;
extern crate serde;
extern crate serde_json;

use chrono::Utc;
use crypto_hash::{hex_digest, Algorithm};
use std::collections::HashMap;
use std::env;

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

pub trait HashableTransaction {
  fn new(from_address: String, to_address: String, amount: u32) -> Self;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
  pub hash: String,
  pub from_address: String,
  pub to_address: String,
  pub amount: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
  pub hash: String,
  created_at: i64, // Timestamp
  data: Vec<Transaction>,
  previous_block_hash: String,
}

impl HashableBlock for Block {
  fn new(data: Vec<Transaction>, previous_block_hash: String) -> Block {
    let mut nonce: i32 = 0;
    let mut hash: String = String::from("");
    let mut substr: String = String::from("");
    let difficulty: usize = match env::var("DIFFICULTY") {
      Ok(value) => value.parse().unwrap(),
      Err(_) => 2,
    };

    while substr != vec!["0"; difficulty].join("") {
      nonce = nonce + 1;

      let mut input: String = String::from("");

      for transaction in data.iter() {
        let serialized_transaction = serde_json::to_string(&transaction).unwrap();
        input.push_str(serialized_transaction.as_str());
      }

      input.push_str(nonce.to_string().as_str());

      hash = create_hash(input);
      substr = hash.chars().take(difficulty).collect();
    }

    return Block {
      hash: hash,
      created_at: get_current_timestamp(),
      data: data,
      previous_block_hash: previous_block_hash,
    };
  }

  fn create_genesis_block() -> Block {
    return Self::new(vec![], "LITHIUM".to_owned());
  }
}

impl HashableTransaction for Transaction {
  fn new(from_address: String, to_address: String, amount: u32) -> Transaction {
    let mut input: String = String::from("");
    let nonce: u64 = rand::random();

    input.push_str(from_address.as_str());
    input.push_str(to_address.as_str());
    input.push_str(&amount.to_string());
    input.push_str(&nonce.to_string());

    let hash = create_hash(input);

    return Transaction {
      hash: hash,
      to_address: to_address,
      from_address: from_address,
      amount: amount,
    };
  }
}

pub type Blockchain = Vec<Block>;
pub type Blockmap = HashMap<String, Block>;

pub trait ChainMethods {
  fn add_block(&mut self, block: Block);
  fn validate_chain(self) -> bool;
}

pub trait LithiumMethods {
  fn mine_block(&mut self);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Lithium {
  pub chain: Blockchain,
  pub map: Blockmap,
  pub pending_transactions: Vec<Transaction>,
}

impl LithiumMethods for Lithium {
  fn mine_block(&mut self) {
    let previous_block_hash = String::from(self.chain.last().unwrap().hash.as_str());
    let block = Block::new(self.pending_transactions.to_owned(), previous_block_hash);

    self.chain.push(block);
  }
}

impl ChainMethods for Blockchain {
  fn add_block(&mut self, block: Block) {
    self.push(block);
  }
  fn validate_chain(self) -> bool {
    for i in 1..self.len() {
      let current_block = &self[i];
      let previoust_block = &self[i - 1];

      if current_block.previous_block_hash != previoust_block.hash {
        return false;
      }
    }

    return true;
  }
}
