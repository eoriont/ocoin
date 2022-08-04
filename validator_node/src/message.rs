use blockchain::{block::Block, signed_transaction::SignedTransaction};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    NewTransaction(SignedTransaction),
    CancelTransaction(SignedTransaction),
    NewBlock(Block),
    RetrieveTransactions,
    ListTransactions(Vec<SignedTransaction>),
}
