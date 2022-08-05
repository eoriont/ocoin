pub mod message;
pub mod node;
pub mod communicator;
pub mod clap_args;

use std::{fs, cmp::{Ordering}};

use blockchain::{transaction::Transaction, blockchain::Blockchain, wallet_manager::{WalletManager, self}};
use clap::{Parser};
use async_std::{io::{prelude::*, stdin, stdout, WriteExt, BufReader}};
use clap_args::{Cli, Commands};
use futures::{select, StreamExt};
use message::Message;
use node::Node;

const BLOCKCHAIN_FILE: &str = "./1.ocoin_blockchain";
const WALLETS_FILE: &str = "./1.ocoin_wallets";

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let mut node = Node::new().await;

    // TODO: put this on multiple threads instead of a select
    // That requires some sort of mutex (an arc perhaps?)
    print!("> ");
    stdout().flush().await.unwrap();
    let mut stdin = BufReader::new(stdin()).lines().fuse();

    loop {
        select! {
            input = stdin.select_next_some() => {
                let command = Cli::try_parse_from(format!("cli {}", input.unwrap()).split_whitespace());
                match command {
                    Ok(command) => execute_command(&mut node, command),
                    Err(e) => println!("{}", e)
                }
                print!("> ");
                stdout().flush().await.unwrap()
            },
            event = node.communicator.swarm.select_next_some() => {
                if let Some(message) = node.communicator.handle_network_event(event) {
                    handle_message(&mut node, message);
                }
            }
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
            node.mempool.transactions.push(signed_tx);

            if let Ok(_) = node.blockchain.validate_new_transactions(&node.mempool.transactions) {
                println!("Successfully added transaction!");
            } else {
                node.mempool.transactions.pop();
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
            for stx in &node.mempool.transactions {
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

            let block = node.mempool.mine(node.blockchain.get_prev_hash(), node.wallet_manager.get_wallet(&"0".to_string()), wallet);

            match node.blockchain.append_block(block.clone()) {
                Ok(()) => {
                    println!("Successfully mined block: {}", node.blockchain.get_prev_hash());
                    node.communicator.publish_message(&Message::NewBlock(block, node.wallet_manager.clone()));
                }
                Err(e) => println!("{}", e),
            }
        }
        Commands::DisplayBalances => {
            let wallet_state = node.blockchain.validate_new_transactions(&node.mempool.transactions).unwrap();

            for (wallet, amount) in wallet_state.into_iter() {
                println!(
                    "| {} {}",
                    wallet, amount
                );
            }
        }
        Commands::Quit => {
            println!("Quitting. Goodbye!");
            std::process::exit(0)
        }
        Commands::Connect { addr } => {
            // TODO: take prints out to top level
            if let Ok(multiaddr) = addr.parse() {
                async_std::task::block_on(node.communicator.connect(multiaddr));
                // Possibly ask to retrieve blockchain?
            } else {
                println!("Invalid address: {}", addr)
            }
        }
        Commands::RetrieveBlockchain => {
            node.communicator.publish_message(&Message::RetrieveBlockchain);
        }
    }
}

fn handle_message(node: &mut Node, message: Message) {
    match message {
        Message::RetrieveBlockchain => {
            let msg = Message::Blockchain(node.blockchain.clone(), node.wallet_manager.clone());
            node.communicator.publish_message(&msg);
        }
        Message::Blockchain(blockchain, wallet_manager) => {
            node.wallet_manager.combine(wallet_manager);
            match blockchain.blocks.len().cmp(&node.blockchain.blocks.len()) {
                Ordering::Less => {
                    println!("Current blockchain is newer");
                    // Maybe notify others?
                }
                Ordering::Equal => {
                    // same length
                    // check if the blocks are the same?
                }
                Ordering::Greater => {
                    println!("Replaced blockchain with new one");
                    node.blockchain = blockchain;
                }
            }
        }
        Message::NewBlock(block, wallet_manager) => {
            node.wallet_manager.combine(wallet_manager);
            match node.blockchain.append_block(block) {
                Ok(()) => println!("Successfully added new block: {}", node.blockchain.get_prev_hash()),
                Err(e) => println!("{}", e),
            }
        }
    }
}