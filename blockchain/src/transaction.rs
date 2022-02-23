use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub sender: String,
    pub reciever: String,
    pub amt: f64,
}

impl Transaction {
    pub fn new(sender: String, reciever: String, amt: f64) -> Self {
        Transaction {
            sender,
            reciever,
            amt,
        }
    }

    pub fn get_hash(&self) -> String {
        let mut s = Sha256::new();
        s.update(&self.sender);
        s.update(&self.reciever);
        s.update(self.amt.to_ne_bytes());
        format!("{:X}", s.finalize())
    }
}
