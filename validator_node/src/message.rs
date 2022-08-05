use blockchain::{block::Block, blockchain::Blockchain, wallet_manager::WalletManager};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    RetrieveBlockchain,
    Blockchain(Blockchain, WalletManager),

    NewBlock(Block, WalletManager),
}
