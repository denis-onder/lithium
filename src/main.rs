#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;

mod domain;
use domain::{Block, HashableBlock, Lithium, Transaction};

fn create_transaction(from_address: String, to_address: String, amount: u32) -> Transaction {
    return Transaction {
        from_address: from_address,
        to_address: to_address,
        amount: amount,
    };
}

fn main() {
    let mut lithium: Lithium = Lithium {
        chain: vec![Block::create_genesis_block()],
        map: HashMap::new(),
        pending_transactions: vec![],
    };

    let tx1 = create_transaction("null".to_owned(), "null".to_owned(), 0);

    let block1 = Block::new(vec![tx1], lithium.chain.last().unwrap().hash.to_owned());

    lithium.chain.push(block1);

    println!("{:?}", lithium);
}
