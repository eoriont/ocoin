use crate::wallet_manager::WalletManager;
use crate::Blockchain;
use crate::Transaction;
use serde::{Deserialize, Serialize};
use std::fs;

pub fn handleCommands(cmd: String, chain: &mut Blockchain, wallet_manager: &mut WalletManager) {
    let args: Vec<&str> = cmd.trim().split(" ").collect();
    match args[0] {
        "mine" => {
            println!("Length: {}", chain.blocks.len());

            let mut block = chain.get_new_block();

            let transaction =
                Transaction::new("0".to_owned(), "0".to_owned(), 10.0, "0".to_owned());
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
            for b in &chain.blocks {
                println!("Hash: {}", b.get_hash());
            }
        }
        "save_chain" => {
            // Save to a file
            let chain_str = serde_json::to_string(chain).expect("test save");
            let _res = fs::write("./chain.ocoin", chain_str);
        }
        "load_chain" => {
            let chain_str = fs::read_to_string("./chain.ocoin").expect("test load");
            *chain = serde_json::from_str::<Blockchain>(&chain_str).expect("test deserialize");
        }
        "save_wallets" => {
            let w_str = serde_json::to_string(wallet_manager).expect("test save wallets");
            let _res = fs::write("./wallets.ocoin", w_str);
        }
        "load_wallets" => {
            let w_str = fs::read_to_string("./wallets.ocoin").expect("test load wallets");
            *wallet_manager =
                serde_json::from_str::<WalletManager>(&w_str).expect("test deserialize wallets");
        }
        "new_wallet" => {
            let name = args[1].to_owned();
            let priv_key = args[2].to_owned();
            wallet_manager.add_wallet(name, priv_key);
            println!("New wallet: {} {}", args[1], args[2]);
        }
        _ => println!("Unrecognized command!"),
    }
}
