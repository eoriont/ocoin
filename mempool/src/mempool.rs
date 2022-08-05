use blockchain::{block::Block, signed_transaction::SignedTransaction, wallet::Wallet, transaction::Transaction};
use serde::{Deserialize, Serialize};
use anyhow::anyhow;

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

    pub fn cancel_transaction(&mut self, tx: SignedTransaction) -> anyhow::Result<()> {
        let tx_hash = tx.get_hash();
        let pos = self
            .transactions
            .iter()
            .position(move |tx| tx.get_hash() == tx_hash);
        if let Some(pos) = pos {
            self.transactions.remove(pos);
            return Ok(());
        } else {
            return Err(anyhow!("Error!"));
        }
    }

    pub fn new_block(&mut self, _block: Block) {
        // TODO
    }

    pub fn mine(&mut self, prev_hash: String, wallet0: &Wallet, miner: String) -> Block {
        let mut block = Block::new(prev_hash);

        // Mining reward
        self.transactions.push(
            wallet0.sign_transaction(Transaction::new("0".to_string(), miner, 10.0))
        );

        block.transactions.extend(self.transactions.drain(..));

        self.get_pow(&mut block);

        block
    }

    pub fn get_pow(&self, block: &mut Block) {
        while !block.validate_pow() {
            block.increase_nonce();
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use blockchain::transaction::Transaction;

//     #[test]
//     fn test_new_transaction() {
//         let mut mempool = Mempool::new();
//         let transaction = SignedTransaction {
//             transaction: Transaction::new("Eli".to_string(), "Ari".to_string(), 10.0),
//             signature: String::from("Hi"),
//         };
//         mempool.new_transaction(transaction);
//         assert_eq!(mempool.transactions.len(), 1)
//     }

//     #[test]
//     fn test_cancel_transaction() {
//         let mut mempool = Mempool::new();
//         let transaction = SignedTransaction {
//             transaction: Transaction::new("Eli".to_string(), "Ari".to_string(), 10.0),
//             signature: String::from("Hi"),
//         };
//         mempool.new_transaction(transaction.clone());
//         mempool.cancel_transaction(transaction);
//         assert_eq!(mempool.transactions.len(), 0)
//     }

//     #[test]
//     fn test_new_block() {
//         // TODO
//         assert!(true)
//     }
// }
