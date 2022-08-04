use blockchain::{blockchain::Blockchain, wallet_manager::WalletManager};
use mempool::mempool::Mempool;

use crate::communicator::Communicator;


pub struct Node {
    pub mempool: Mempool,
    pub blockchain: Blockchain,
    pub communicator: Communicator,
    pub wallet_manager: WalletManager
}

impl Node {
    pub async fn new() -> Node {
        Node {
            mempool: Mempool::new(),
            blockchain: Blockchain::new(),
            communicator: Communicator::new().await,
            wallet_manager: WalletManager::new()
        }
    }

}