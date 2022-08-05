use crate::signed_transaction::SignedTransaction;
use crate::transaction::Transaction;
use k256::ecdsa::{signature::Signer, Signature, SigningKey, VerifyingKey};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use std::str;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Wallet {
    pub private_key: String,
    pub public_key: String,
}

impl Wallet {
    pub fn new() -> Self {
        let priv_key = SigningKey::random(&mut OsRng);
        let pub_key = VerifyingKey::from(&priv_key);
        let priv_str = hex::encode(priv_key.to_bytes());
        let pub_str = hex::encode(pub_key.to_bytes());
        Wallet {
            private_key: priv_str,
            public_key: pub_str,
        }
    }

    pub fn sign_transaction(&self, transaction: Transaction) -> SignedTransaction {
        let priv_key_bytes = &hex::decode(&self.private_key).unwrap();
        let priv_key = SigningKey::from_bytes(&priv_key_bytes).unwrap();
        let tx_hash = transaction.get_hash();
        let sig: Signature = priv_key.sign(tx_hash.as_bytes());
        let sig_str = hex::encode(sig.to_vec());
        SignedTransaction {
            transaction,
            signature: sig_str,
        }
    }
}
