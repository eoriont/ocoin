use block::Block;
use blockchain::Blockchain;
use std::env;
use std::fs;
use transaction::Transaction;

mod block;
mod blockchain;
mod transaction;

fn main() {
    let mut chain = Blockchain::new();
    println!("Length: {}", chain.blocks.len());

    let mut block = chain.get_new_block();

    let transaction = Transaction::new("0".to_owned(), "0".to_owned(), 10.0, "0".to_owned());
    block.add_transaction(transaction);

    chain.mine_block(&mut block);
    chain.append_block(block);

    let mut block = chain.get_new_block();
    let transaction = Transaction::new(
        "Joe".to_owned(),
        "Bob".to_owned(),
        10.0,
        "123478".to_owned(),
    );
    block.add_transaction(transaction);

    chain.mine_block(&mut block);
    chain.append_block(block);

    println!("Length: {}", chain.blocks.len());
    for b in chain.blocks {
        println!("Hash: {}", b.get_hash());
    }
    // let block = chain.blocks.last().expect("adding block didn't work");
    // println!("Transactions: {}", block.transactions.len())
    // let filename = env::args().nth(1).unwrap();
    // let contents = fs::read(filename).unwrap();
    // let str_contents = String::from_utf8(contents).unwrap();
    // println!("{}", str_contents);

    // Wallet 1: 3F9FC87321C345A9BB5EC79FE6818
    // Wallet 2: EC22AF2346C943226E4D7D5CBC2FA
}
