use crate::wallet::Wallet;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct WalletManager {
    pub wallets: HashMap<String, Wallet>,
}

impl WalletManager {
    pub fn new() -> Self {
        WalletManager {
            wallets: HashMap::new(),
        }
    }

    pub fn new_wallet(&mut self, name: String) {
        let wallet = Wallet::new();
        self.wallets.insert(name, wallet);
    }

    pub fn get_wallet(&self, name: &String) -> &Wallet {
        self.wallets.get(name).expect("no wallet!")
    }
}
