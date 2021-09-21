struct Transaction {
    from_address: String,
    to_address: String,
    amount: i32,
}

struct Block {
    hash: String,
    created_at: i64, // Timestamp
    data: Transaction,
    previousBlockHash: String,
    nonce: i32,
}

type Blockchain = Vec<Block>;

fn main() {
    let lithium: Blockchain = Vec::new();
    println!("Hello, world!");
}
