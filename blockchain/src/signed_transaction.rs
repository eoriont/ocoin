use crate::transaction::Transaction;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignedTransaction {
    pub transaction: Transaction,
    pub signature: String,
}

impl SignedTransaction {
    pub fn get_hash(&self) -> String {
        let mut s = Sha256::new();
        s.update(&self.signature);
        s.update(self.transaction.get_hash());
        format!("{:X}", s.finalize())
    }
}
