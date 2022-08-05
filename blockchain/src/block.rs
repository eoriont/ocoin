use crate::signed_transaction::SignedTransaction;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub prev_hash: String,
    pub transactions: Vec<SignedTransaction>,
    pub nonce: i32,
}

impl Block {
    pub fn new(prev_hash: String) -> Self {
        Block {
            prev_hash,
            transactions: vec![],
            nonce: 0,
        }
    }

    pub fn increase_nonce(&mut self) {
        self.nonce += 1;
    }

    pub fn validate_pow(&self) -> bool {
        self.get_hash().starts_with("00")
    }

    pub fn get_hash(&self) -> String {
        let mut s = Sha256::new();
        s.update(&self.prev_hash);
        let t: Vec<String> = (&self.transactions)
            .into_iter()
            .map(|t| t.get_hash())
            .collect();
        s.update(t.into_iter().fold(String::new(), |x, y| x + &y));
        // s.update(&self.transactions.iter().map(|t| t.get_hash()));
        // self.transactions.into_iter().inspect(|t| s.update(t.get_hash()))
        for t in &self.transactions {
            s.update(t.get_hash());
        }
        s.update(self.nonce.to_le_bytes());
        format!("{:X}", s.finalize())
    }
}
