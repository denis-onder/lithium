use chrono::Utc;
use crypto_hash::{hex_digest, Algorithm};

extern crate serde;
extern crate serde_json;

// Import this crate to derive the Serialize and Deserialize traits.
#[macro_use]
extern crate serde_derive;

static DIFFICULTY: usize = 5;

fn create_hash(input: String) -> String {
    return hex_digest(Algorithm::SHA256, input.as_bytes());
}

trait Hashable {
    fn new(created_at: i64, data: Vec<Transaction>, previous_block_hash: String) -> Self;
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

impl Hashable for Block {
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
}

type Blockchain = Vec<Block>;

fn create_transaction(from_address: String, to_address: String, amount: u32) -> Transaction {
    return Transaction {
        from_address: from_address,
        to_address: to_address,
        amount: amount,
    };
}

fn main() {
    let date_time = Utc::now();

    let tx1 = create_transaction(String::from("null"), String::from("null"), 0);
    let block1 = Block::new(date_time.timestamp(), vec![tx1], String::from("NULL"));

    let mut lithium: Blockchain = Vec::new();

    lithium.push(block1);

    println!("{:?}", lithium);
}
