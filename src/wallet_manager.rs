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

    pub fn add_wallet(&mut self, name: String, priv_key: String) {
        let wallet = Wallet::new(priv_key);
        self.wallets.insert(name, wallet);
    }
}
