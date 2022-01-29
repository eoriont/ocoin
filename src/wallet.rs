use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize)]
pub struct Wallet {
    pub private_key: String,
    pub public_key: String,
}

impl Wallet {
    pub fn new(priv_key: String) -> Self {
        let pub_key = priv_to_pub_key(&priv_key);
        Wallet {
            private_key: priv_key,
            public_key: pub_key,
        }
    }
}

pub fn priv_to_pub_key(priv_key: &String) -> String {
    let mut s = Sha256::new();
    s.update(priv_key);
    format!("{:X}", s.finalize())
}
