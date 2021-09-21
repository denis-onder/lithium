use chrono::Utc;
use crypto_hash::{hex_digest, Algorithm};

static DIFFICULTY: i8 = 5;

#[derive(Debug)]
struct Transaction {
    from_address: String,
    to_address: String,
    amount: u32,
}

#[derive(Debug)]
struct Block {
    hash: String,
    created_at: i64, // Timestamp
    data: Transaction,
    previous_block_hash: String,
    nonce: i32,
}

type Blockchain = Vec<Block>;

fn create_transaction(from_address: String, to_address: String, amount: u32) -> Transaction {
    return Transaction {
        from_address: from_address,
        to_address: to_address,
        amount: amount,
    };
}

fn create_block(
    created_at: i64,
    data: Transaction,
    previous_block_hash: String,
    nonce: i32,
) -> Block {
    let mut input: String = String::from("");

    input.push_str(data.from_address.as_str());
    input.push_str(data.to_address.as_str());
    input.push_str(data.amount.to_string().as_str());

    let hash = hex_digest(Algorithm::SHA256, input.as_bytes());

    return Block {
        hash: hash,
        created_at: created_at,
        data: data,
        previous_block_hash: previous_block_hash,
        nonce: nonce,
    };
}

fn main() {
    // create a Sha256 object
    let date_time = Utc::now();

    let tx1 = create_transaction(String::from("null"), String::from("null"), 0);
    let block1 = create_block(date_time.timestamp(), tx1, String::from("NULL"), 0);

    let mut lithium: Blockchain = Vec::new();

    lithium.push(block1);

    println!("{:?}", lithium);
}
