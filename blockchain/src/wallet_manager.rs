use crate::wallet::Wallet;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WalletManager {
    pub wallets: HashMap<String, Wallet>,
}

impl WalletManager {
    pub fn new() -> Self {
        let mut wallet_manager = WalletManager {
            wallets: HashMap::new(),
        };
        wallet_manager.new_wallet("0".to_string()); 
        wallet_manager
    }

    pub fn new_wallet(&mut self, name: String) {
        let wallet = Wallet::new();
        self.wallets.insert(name, wallet);
    }

    pub fn get_wallet(&self, name: &String) -> &Wallet {
        self.wallets.get(name).expect("no wallet!")
    }

    pub fn combine(&mut self, wallet_manager: WalletManager) {
        self.wallets.extend(wallet_manager.wallets);
    }
}
