#[macro_use]
extern crate serde_derive;
extern crate dotenv;

use dotenv::dotenv;
use std::collections::HashMap;

mod domain;
use domain::{Block, ChainMethods, HashableBlock, HashableTransaction, Lithium, Transaction};

fn main() {
    dotenv().ok();

    let mut lithium: Lithium = Lithium {
        chain: vec![Block::create_genesis_block()],
        map: HashMap::new(),
        pending_transactions: vec![],
    };

    for i in 0..3 {
        let transaction = Transaction::new("null".to_owned(), "null".to_owned(), i);
        lithium.pending_transactions.push(transaction);

        let block = Block::new(
            vec![lithium.pending_transactions.pop().unwrap()],
            lithium.chain.last().unwrap().hash.to_owned(),
        );

        lithium.chain.add_block(block);

        println!(
            "Block added to chain:\n{}\n",
            lithium.chain.last().unwrap().hash
        );
    }

    println!("Valid:\n{:?}", lithium.chain.validate_chain());
}
