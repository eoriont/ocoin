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

// Dumb: add random number to each message to avoid duplication
// Even dumber: it should be a timestamp, not a random number
#[derive(Serialize, Deserialize, Debug)]
pub enum MempoolMessage {
    NewTransaction(SignedTransaction, u16),
    CancelTransaction(SignedTransaction, u16),
    NewBlock(Block, u16),
    RetrieveTransactions(u16),
    ListTransactions(Vec<SignedTransaction>, u16),
}
