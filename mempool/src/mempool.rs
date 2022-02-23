use blockchain::{block::Block, signed_transaction::SignedTransaction};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Mempool {
    pub transactions: Vec<SignedTransaction>,
}

impl Mempool {
    pub fn new() -> Mempool {
        Mempool {
            transactions: vec![],
        }
    }

    pub fn new_transaction(&mut self, tx: SignedTransaction) {
        self.transactions.push(tx);
    }

    pub fn cancel_transaction(&mut self, tx: SignedTransaction) {
        let tx_hash = tx.get_hash();
        let pos = self
            .transactions
            .iter()
            .position(move |tx| tx.get_hash() == tx_hash);
        if let Some(pos) = pos {
            self.transactions.remove(pos);
        }
    }

    pub fn new_block(&mut self, _block: Block) {
        // TODO
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use blockchain::transaction::Transaction;

    #[test]
    fn test_new_transaction() {
        let mut mempool = Mempool::new();
        let transaction = SignedTransaction {
            transaction: Transaction::new("Eli".to_string(), "Ari".to_string(), 10.0),
            signature: String::from("Hi"),
        };
        mempool.new_transaction(transaction);
        assert_eq!(mempool.transactions.len(), 1)
    }

    #[test]
    fn test_cancel_transaction() {
        let mut mempool = Mempool::new();
        let transaction = SignedTransaction {
            transaction: Transaction::new("Eli".to_string(), "Ari".to_string(), 10.0),
            signature: String::from("Hi"),
        };
        mempool.new_transaction(transaction.clone());
        mempool.cancel_transaction(transaction);
        assert_eq!(mempool.transactions.len(), 0)
    }

    #[test]
    fn test_new_block() {
        // TODO
        assert!(true)
    }
}
