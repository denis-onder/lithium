use chrono::Utc;
use crypto_hash::{hex_digest, Algorithm};
use std::collections::HashMap;

extern crate serde;
extern crate serde_json;

// Import this crate to derive the Serialize and Deserialize traits.
#[macro_use]
extern crate serde_derive;

static DIFFICULTY: usize = 5;

fn get_current_timestamp() -> i64 {
    let date_time = Utc::now();

    return date_time.timestamp();
}

fn create_hash(input: String) -> String {
    return hex_digest(Algorithm::SHA256, input.as_bytes());
}

trait HashableBlock {
    fn new(created_at: i64, data: Vec<Transaction>, previous_block_hash: String) -> Self;
    fn create_genesis_block() -> Self;
}

#[derive(Serialize, Deserialize, Debug)]
struct Transaction {
    from_address: String,
    to_address: String,
    amount: u32,
}

#[derive(Debug)]
struct Block {
    hash: String,
    created_at: i64, // Timestamp
    data: Vec<Transaction>,
    previous_block_hash: String,
    nonce: i32,
}

type Blockchain = Vec<Block>;
type Blockmap = HashMap<String, Block>;

#[derive(Debug)]
struct Lithium {
    chain: Blockchain,
    map: Blockmap,
    pending_transactions: Vec<Transaction>,
}

impl HashableBlock for Block {
    fn new(created_at: i64, data: Vec<Transaction>, previous_block_hash: String) -> Block {
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
            created_at: created_at,
            data: data,
            previous_block_hash: previous_block_hash,
            nonce: nonce,
        };
    }

    fn create_genesis_block() -> Block {
        return Self::new(get_current_timestamp(), vec![], "LITHIUM".to_owned());
    }
}

fn create_transaction(from_address: String, to_address: String, amount: u32) -> Transaction {
    return Transaction {
        from_address: from_address,
        to_address: to_address,
        amount: amount,
    };
}

fn main() {
    let tx1 = create_transaction(String::from("null"), String::from("null"), 0);
    let block1 = Block::new(get_current_timestamp(), vec![tx1], String::from("NULL"));

    let mut lithium: Lithium = Lithium {
        chain: vec![Block::create_genesis_block()],
        map: HashMap::new(),
        pending_transactions: vec![],
    };

    lithium.chain.push(block1);

    println!("{:?}", lithium);
}
