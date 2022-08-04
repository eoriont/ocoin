// use std::{
//     collections::hash_map::DefaultHasher,
//     error::Error,
//     hash::{Hash, Hasher},
//     time::Duration,
// };

// use blockchain::{signed_transaction::SignedTransaction, transaction::Transaction};
// use libp2p::{
//     gossipsub::{
//         self,
//         error::{GossipsubHandlerError, PublishError},
//         Gossipsub, GossipsubEvent, GossipsubMessage, IdentTopic as Topic, MessageAuthenticity,
//         MessageId, ValidationMode,
//     },
//     identity,
//     swarm::SwarmEvent,
//     Multiaddr, PeerId, Swarm,
// };
// use rand::Rng;

use crate::mempool::Mempool;

pub struct App {
    pub mempool: Mempool,
}

impl App {
    // pub fn list_transactions(&self) {
    //     println!("{:?}", self.mempool);
    // }

    // pub fn request_synchronise(&mut self) {
    //     self.publish_message(&Message::RetrieveTransactions(rand_u16()));
    // }
    
    // pub fn new_transaction(&mut self, tx: SignedTransaction) {
    //     let msg = Message::NewTransaction(tx);
    //     self.publish_message(&msg);
    //     if let Message::NewTransaction(st) = msg {
    //         self.mempool.new_transaction(st);
    //     }
    // }

    // pub fn broadcast_synchronise(&mut self) {
    //     // Kinda dumb: drain txs into enum value, publish it, then take it out
    //     let new_msg =
    //         Message::ListTransactions(self.mempool.transactions.drain(..).collect(), rand_u16());
    //     self.publish_message(&new_msg);
    //     if let Message::ListTransactions(m, _) = new_msg {
    //         self.mempool.transactions = m;
    //     }
    // }

    // pub fn handle_command(&mut self, command: String) {
    //     match command.as_str() {
    //         cmd if cmd.starts_with("newtx") => {
    //             let args: Vec<&str> = cmd.split_whitespace().collect();
    //             let st = SignedTransaction {
    //                 transaction: Transaction::new(
    //                     args.get(1).unwrap().to_string(),
    //                     args.get(2).unwrap().to_string(),
    //                     args.get(3).unwrap().parse::<f64>().unwrap(),
    //                 ),
    //                 signature: String::from("hi"),
    //             };
    //             self.new_transaction(st);
    //         }

    //         "txlist" => self.list_transactions(),

    //         "reqsync" => self.request_synchronise(),

    //         "broadcastsync" => self.broadcast_synchronise(),

    //         _ => println!("Unrecognized command!"),
    //     }
    // }

    // pub fn handle_message(&mut self, msg: Message) {
    //     println!("{:?}", msg);
    //     match msg {
    //         Message::NewTransaction(tx, _) => self.mempool.new_transaction(tx),
    //         Message::CancelTransaction(tx, _) => self.mempool.cancel_transaction(tx),
    //         Message::NewBlock(block, _) => self.mempool.new_block(block),
    //         Message::RetrieveTransactions(_) => self.broadcast_synchronise(),

    //         Message::ListTransactions(txs, _) => {
    //             // todo: actually check if txs is more updated than transactions
    //             if self.mempool.transactions.len() < txs.len() {
    //                 self.mempool.transactions = txs;
    //             }
    //         }
    //     }
    // }
}

// fn rand_u16() -> u16 {
//     rand::thread_rng().gen()
// }
