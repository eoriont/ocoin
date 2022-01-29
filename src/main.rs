use crate::commands::handleCommands;
use block::Block;
use blockchain::Blockchain;
use std::env;
use std::fs;
use std::io;
use std::io::*;
use transaction::Transaction;
use wallet_manager::WalletManager;

mod block;
mod blockchain;
mod commands;
mod transaction;
mod wallet;
mod wallet_manager;

fn main() {
    // let block = chain.blocks.last().expect("adding block didn't work");
    // println!("Transactions: {}", block.transactions.len())
    // let filename = env::args().nth(1).unwrap();
    // let contents = fs::read(filename).unwrap();
    // let str_contents = String::from_utf8(contents).unwrap();
    // println!("{}", str_contents);

    // Wallet 1: 3F9FC87321C345A9BB5EC79FE6818
    // Wallet 2: EC22AF2346C943226E4D7D5CBC2FA
    let mut chain = Blockchain::new();
    let mut wallet_manager = WalletManager::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Couldn't read user input!");

        handleCommands(input, &mut chain, &mut wallet_manager);
    }
}
