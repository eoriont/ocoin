use crate::blockchain::Blockchain;
use crate::transaction::Transaction;
use crate::wallet_manager::WalletManager;
use std::fs;

pub fn handle_commands(cmd: String, chain: &mut Blockchain, wallet_manager: &mut WalletManager) {
    let args: Vec<&str> = cmd.trim().split(" ").collect();
    match args[0] {
        "mine" => {
            println!("Length: {}", chain.blocks.len());

            let mut block = chain.get_new_block();

            block.transactions.extend(chain.current_txs.drain(..));

            chain.mine_block(&mut block);
            match chain.append_block(block) {
                Ok(()) => println!("Successfully mined block: {}", chain.get_prev_hash()),
                Err(e) => println!("{}", e),
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
            wallet_manager.new_wallet(name.clone());
            println!(
                "New wallet: {} {}",
                args[1],
                &wallet_manager.get_wallet(&name).public_key
            );
        }
        "tx" => {
            let w1 = args[1].to_owned();
            let w2 = args[2].to_owned();
            let amt = args[3].parse::<f64>().unwrap();
            println!("Made new tx: {} {} {}", w1, w2, amt);
            let tx = Transaction::new(w1.clone(), w2, amt);
            let signed_tx = wallet_manager.get_wallet(&w1).sign_transaction(tx);
            chain.current_txs.push(signed_tx);
        }
        "print_chain" => {
            println!("Length: {}", chain.blocks.len());
            for b in &chain.blocks {
                println!("Hash: {}", b.get_hash());
                for stx in &b.transactions {
                    let tx = &stx.transaction;
                    println!(
                        "| {} {} {} {}",
                        &tx.sender, &tx.reciever, &tx.amt, &stx.signature
                    );
                }
            }
        }
        "list_wallets" => {
            for (name, wallet) in &wallet_manager.wallets {
                println!("{} {}", name, wallet.public_key);
            }
        }

        _ => println!("Unrecognized command!"),
    }
}
