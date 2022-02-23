use blockchain::{block::Block, signed_transaction::SignedTransaction};
use serde::{Deserialize, Serialize};

// Dumb: add random number to each message to avoid duplication
// Even dumber: it should be a timestamp, not a random number
#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    NewTransaction(SignedTransaction, u16),
    CancelTransaction(SignedTransaction, u16),
    NewBlock(Block, u16),
    RetrieveTransactions(u16),
    ListTransactions(Vec<SignedTransaction>, u16),
}
