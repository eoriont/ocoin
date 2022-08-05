pub mod message;
pub mod node;
pub mod communicator;
pub mod clap_args;

use std::fs;

use blockchain::{transaction::Transaction, blockchain::Blockchain, wallet_manager::WalletManager};
use clap::{Parser};
use async_std::{io::{stdin, stdout, WriteExt}};
use clap_args::{Cli, Commands};
use node::Node;

const BLOCKCHAIN_FILE: &str = "./1.ocoin_blockchain";
const WALLETS_FILE: &str = "./1.ocoin_wallets";

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let mut node = Node::new().await;

    loop {
        print!("> ");
        stdout().flush().await.unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).await.unwrap();

        let command = Cli::try_parse_from(format!("cli {}", input).split_whitespace());
        match command {
            Ok(command) => execute_command(&mut node, command),
            Err(e) => println!("{}", e)
        }
    }
}

fn execute_command(node: &mut Node, cli: Cli) {
    match cli.command {
        Commands::StartMempool { address } => {
            println!("Starting the validator node and looking for address {}", address.unwrap_or("None".to_string()));
        }
        Commands::NewTx { wallet1, wallet2, amount } => {
            let tx = Transaction::new(wallet1.clone(), wallet2, amount);
            let signed_tx = node.wallet_manager.get_wallet(&wallet1).sign_transaction(tx);
            node.blockchain.current_txs.push(signed_tx);

            if let Ok(_) = node.blockchain.validate_new_transactions(&node.blockchain.current_txs) {
                println!("Successfully added transaction!");
            } else {
                node.blockchain.current_txs.pop();
                println!("Error: invalid transaction!");
            }
        }
        Commands::DisplayBlockchain => {
            println!("Length: {}", node.blockchain.blocks.len());
            for b in &node.blockchain.blocks {
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
        Commands::DisplayBlock { block_id } => {
            let block = node.blockchain.blocks.get(block_id);
            match block {
                Some(b) => {
                    println!("Hash: {}", b.get_hash());
                    for stx in &b.transactions {
                        let tx = &stx.transaction;
                        println!(
                            "| {} {} {} {}",
                            &tx.sender, &tx.reciever, &tx.amt, &stx.signature
                        );
                    }
                },
                None => {
                    println!("Failed to find block {}", block_id);
                }
            }
        }
        Commands::DisplayCurrentTransactions => {
            for stx in &node.blockchain.current_txs {
                let tx = &stx.transaction;
                println!(
                    "| {} {} {} {}",
                    &tx.sender, &tx.reciever, &tx.amt, &stx.signature
                );
            }
        }
        Commands::SaveBlockchain => {
            // Blockchain
            let chain_str = serde_json::to_string(&node.blockchain).expect("test save");
            let _res = fs::write(BLOCKCHAIN_FILE, chain_str);

            // Wallets
            let wallets_str = serde_json::to_string(&node.wallet_manager).expect("test save");
            let _res = fs::write(WALLETS_FILE, wallets_str);

            println!("Successfully saved the blockchain to {} and {}", BLOCKCHAIN_FILE, WALLETS_FILE);
        }
        Commands::LoadBlockchain => {
            // Blockchain
            let chain_str = fs::read_to_string(BLOCKCHAIN_FILE).expect("test load");
            node.blockchain = serde_json::from_str::<Blockchain>(&chain_str).expect("test deserialize");

            // Wallets
            let wallets_str = fs::read_to_string(WALLETS_FILE).expect("test load");
            node.wallet_manager = serde_json::from_str::<WalletManager>(&wallets_str).expect("test deserialize");

            println!("Successfully loaded the blockchain from {} and {}", BLOCKCHAIN_FILE, WALLETS_FILE);
        }
        Commands::NewWallet { name } => {
            node.wallet_manager.new_wallet(name.clone());
            println!(
                "New wallet: {} {}",
                name,
                node.wallet_manager.get_wallet(&name).public_key
            );
        }
        Commands::Mine { wallet } => {
            println!("Length: {}", node.blockchain.blocks.len());

            let mut block = node.blockchain.get_new_block();

            // Mining reward
            node.blockchain.current_txs.push(
                node.wallet_manager.get_wallet(&"0".to_string()).sign_transaction(Transaction::new("0".to_string(), wallet, 10.0))
            );

            block.transactions.extend(node.blockchain.current_txs.drain(..));

            node.blockchain.mine_block(&mut block);
            match node.blockchain.append_block(block) {
                Ok(()) => println!("Successfully mined block: {}", node.blockchain.get_prev_hash()),
                Err(e) => println!("{}", e),
            }

        }
        Commands::DisplayBalances => {
            let wallet_state = node.blockchain.validate_new_transactions(&node.blockchain.current_txs).unwrap();

            for (wallet, amount) in wallet_state.into_iter() {
                println!(
                    "| {} {}",
                    wallet, amount
                );
            }
        }
    }
}