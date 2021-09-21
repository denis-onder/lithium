use chrono::Utc;
use crypto_hash::{hex_digest, Algorithm};

static DIFFICULTY: usize = 5;

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

fn create_block(created_at: i64, data: Transaction, previous_block_hash: String) -> Block {
    let mut nonce: i32 = 0;
    let mut hash: String = String::from("");
    let mut substr: String = String::from("");

    while substr != String::from("00000") {
        nonce = nonce + 1;

        let mut input: String = String::from("");

        input.push_str(data.from_address.as_str());
        input.push_str(data.to_address.as_str());
        input.push_str(data.amount.to_string().as_str());
        input.push_str(nonce.to_string().as_str());

        hash = hex_digest(Algorithm::SHA256, input.as_bytes());
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

fn main() {
    // create a Sha256 object
    let date_time = Utc::now();

    let tx1 = create_transaction(String::from("null"), String::from("null"), 0);
    let block1 = create_block(date_time.timestamp(), tx1, String::from("NULL"));

    let mut lithium: Blockchain = Vec::new();

    lithium.push(block1);

    println!("{:?}", lithium);
}
