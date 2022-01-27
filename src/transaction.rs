use sha2::{Digest, Sha256};

pub struct Transaction {
    signature: String,
    sender: String,
    reciever: String,
    amt: f64,
}

impl Transaction {
    pub fn new(sender: String, reciever: String, amt: f64, signature: String) -> Self {
        Transaction {
            signature,
            sender,
            reciever,
            amt,
        }
    }

    pub fn get_hash(&self) -> String {
        let mut s = Sha256::new();
        s.update(&self.signature);
        s.update(&self.sender);
        s.update(&self.reciever);
        s.update(self.amt.to_ne_bytes());
        format!("{:X}", s.finalize())
    }
}
