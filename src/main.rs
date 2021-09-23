#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;

mod domain;
use domain::{Block, HashableBlock, HashableTransaction, Lithium, Transaction};

fn main() {
    let mut lithium: Lithium = Lithium {
        chain: vec![Block::create_genesis_block()],
        map: HashMap::new(),
        pending_transactions: vec![],
    };

    let tx1 = Transaction::new("null".to_owned(), "null".to_owned(), 0);

    let block1 = Block::new(vec![tx1], lithium.chain.last().unwrap().hash.to_owned());

    lithium.chain.push(block1);

    println!("Block added to chain:\n{:?}", lithium.chain.last());
}
