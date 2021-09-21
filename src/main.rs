use chrono::Utc;

#[derive(Debug)]
struct Transaction {
    hash: String,
    from_address: String,
    to_address: String,
    amount: i32,
}

#[derive(Debug)]
struct Block {
    hash: String,
    created_at: i64, // Timestamp
    data: Transaction,
    previousBlockHash: String,
    nonce: i32,
}

type Blockchain = Vec<Block>;

fn create_transaction(from_address: String, to_address: String, amount: i32) -> Transaction {
    return Transaction {
        hash: String::from("TEST"),
        from_address: from_address,
        to_address: to_address,
        amount: amount,
    };
}

fn create_block(
    hash: String,
    created_at: i64,
    data: Transaction,
    previousBlockHash: String,
    nonce: i32,
) -> Block {
    return Block {
        hash: hash,
        created_at: created_at,
        data: data,
        previousBlockHash: previousBlockHash,
        nonce: nonce,
    };
}

fn main() {
    let date_time = Utc::now();
    // let difficulty = 5;

    let tx1 = create_transaction(String::from("null"), String::from("null"), 0);
    let block1 = create_block(
        String::from("TEST"),
        date_time.timestamp(),
        tx1,
        String::from("HEAD"),
        0,
    );

    let mut lithium: Blockchain = Vec::new();

    lithium.push(block1);

    println!("{:?}", lithium);
}
