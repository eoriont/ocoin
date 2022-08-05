use std::{collections::HashMap};

use crate::block::Block;
use crate::signed_transaction::SignedTransaction;
use anyhow::bail;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            blocks: vec![],
        }
    }

    pub fn append_block(&mut self, mut block: Block) -> Result<(), &str> {
        if self.validate_new_block(&mut block) {
            self.blocks.push(block);
            return Ok(());
        }
        return Err("Block didn't validate");
    }

    pub fn get_prev_hash(&self) -> String {
        match self.blocks.last() {
            None => "0".to_owned(),
            Some(latest_block) => latest_block.get_hash(),
        }
    }

    pub fn validate_new_block(&self, block: &Block) -> bool {
        if !block.validate_pow() { return false }
        if let Err(_) = self.validate_new_transactions(&block.transactions) { return false }
        true
    }

    pub fn validate_new_transactions(&self, new_txs: &Vec<SignedTransaction>) -> anyhow::Result<HashMap<String, f64>> {
        let mut wallet_state: HashMap<String, f64> = HashMap::new();

        for b in &self.blocks {
            if !self.validate_transactions(&b.transactions, &mut wallet_state) { bail!("Failed to validate block in blockchain!") }
        }

        if !self.validate_transactions(new_txs, &mut wallet_state) { bail!("Failed to validate block in new transactions!") }

        Ok(wallet_state)
    }

    pub fn validate_transactions(&self, stxs: &Vec<SignedTransaction>, wallet_state: &mut HashMap<String, f64>) -> bool {
        for stx in stxs {
            let tx = &stx.transaction;
            if !wallet_state.contains_key(&tx.sender) {
                wallet_state.insert(tx.sender.clone(), 0.0);
            }
            if !wallet_state.contains_key(&tx.reciever) {
                wallet_state.insert(tx.reciever.clone(), 0.0);
            }

            *wallet_state.get_mut(&tx.sender).unwrap() -= tx.amt;
            *wallet_state.get_mut(&tx.reciever).unwrap() += tx.amt;

            if &tx.sender != &"0".to_string() {
                if wallet_state.get(&tx.sender).unwrap() < &0.0 {
                    return false;
                }
            }
        }
        true
    }
}
