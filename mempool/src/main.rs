mod mempool;

use async_std::io;
use blockchain::transaction::Transaction;
use env_logger::{Builder, Env};
use futures::{prelude::*, select};
use libp2p::gossipsub::error::PublishError;
use libp2p::gossipsub::{
    Gossipsub, GossipsubEvent, GossipsubMessage, MessageAuthenticity, ValidationMode,
};
use libp2p::gossipsub::{IdentTopic as Topic, MessageId};
use libp2p::Swarm;
use libp2p::{gossipsub, identity, swarm::SwarmEvent, Multiaddr, PeerId};
use rand::{self, Rng};
use serde_json;
use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::hash::{Hash, Hasher};
use std::time::Duration;

use crate::mempool::{Mempool, MempoolMessage};
use blockchain::signed_transaction::SignedTransaction;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Builder::from_env(Env::default().default_filter_or("info")).init();

    // Create a random PeerId
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local peer id: {:?}", local_peer_id);

    // Set up an encrypted TCP Transport over the Mplex and Yamux protocols
    let transport = libp2p::development_transport(local_key.clone()).await?;

    // Create a Gossipsub topic
    let topic = Topic::new("transactions");

    // Create a Swarm to manage peers and events
    let mut swarm = {
        // To content-address message, we can take the hash of message and use it as an ID.
        let message_id_fn = |message: &GossipsubMessage| {
            let mut s = DefaultHasher::new();
            message.data.hash(&mut s);
            MessageId::from(s.finish().to_string())
        };

        // Set a custom gossipsub
        let gossipsub_config = gossipsub::GossipsubConfigBuilder::default()
            .heartbeat_interval(Duration::from_secs(10)) // This is set to aid debugging by not cluttering the log space
            .validation_mode(ValidationMode::Strict) // This sets the kind of message validation. The default is Strict (enforce message signing)
            .message_id_fn(message_id_fn) // content-address messages. No two messages of the
            // same content will be propagated.
            .build()
            .expect("Valid config");
        // build a gossipsub network behaviour
        let mut gossipsub: gossipsub::Gossipsub =
            gossipsub::Gossipsub::new(MessageAuthenticity::Signed(local_key), gossipsub_config)
                .expect("Correct configuration");

        // subscribes to our topic
        gossipsub.subscribe(&topic).unwrap();

        // add an explicit peer if one was provided
        if let Some(explicit) = std::env::args().nth(2) {
            let explicit = explicit.clone();
            match explicit.parse() {
                Ok(id) => gossipsub.add_explicit_peer(&id),
                Err(err) => println!("Failed to parse explicit peer id: {:?}", err),
            }
        }

        // build the swarm
        libp2p::Swarm::new(transport, gossipsub, local_peer_id)
    };

    // Listen on all interfaces and whatever port the OS assigns
    swarm
        .listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap())
        .unwrap();

    // Reach out to another node if specified
    if let Some(to_dial) = std::env::args().nth(1) {
        let address: Multiaddr = to_dial.parse().expect("User to provide valid address.");
        match swarm.dial(address.clone()) {
            Ok(_) => println!("Dialed {:?}", address),
            Err(e) => println!("Dial {:?} failed: {:?}", address, e),
        };
    }

    // Read full lines from stdin
    let mut stdin = io::BufReader::new(io::stdin()).lines().fuse();

    let mut mempool = Mempool::new();

    // Kick it off
    loop {
        select! {
            line = stdin.select_next_some() => {
                let cmd = line.expect("Stdin not to close");
                handle_command(cmd, &mut mempool, &mut swarm, topic.clone());

            },
            event = swarm.select_next_some() => match event {
                SwarmEvent::Behaviour(GossipsubEvent::Message {
                    propagation_source: peer_id,
                    message_id: id,
                    message,
                }) => {
                    let mempool_message = serde_json::from_slice::<MempoolMessage>(&message.data).unwrap();
                    println!(
                        "Got message: {:?} with id: {} from peer: {:?}",
                        mempool_message,
                        id,
                        peer_id
                    );
                    handle_message(&mut mempool, mempool_message, &mut swarm, topic.clone());

                },
                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("Listening on {:?}", address);
                }
                _ => {}
            }
        }
    }
}

fn handle_command(
    string: String,
    mempool: &mut Mempool,
    swarm: &mut Swarm<Gossipsub>,
    topic: Topic,
) {
    match string.as_str() {
        cmd if cmd.starts_with("newtx") => {
            let args: Vec<&str> = cmd.split_whitespace().collect();
            let t = Transaction::new(
                args.get(1).unwrap().to_string(),
                args.get(2).unwrap().to_string(),
                args.get(3).unwrap().parse::<f64>().unwrap(),
            );
            let st = SignedTransaction {
                transaction: t,
                signature: String::from("hi"),
            };
            let msg = MempoolMessage::NewTransaction(st, rand_u16());
            publish(&msg, swarm, topic);
            if let MempoolMessage::NewTransaction(st, _) = msg {
                mempool.new_transaction(st);
            }
        }

        "txlist" => {
            println!("{:?}", mempool);
        }

        "reqsync" => {
            // Ask everyone for their chains and use one
            publish(&MempoolMessage::RetrieveTransactions(rand_u16()), swarm, topic);
        }

        "broadcastsync" => {
            handle_message(mempool, MempoolMessage::RetrieveTransactions(rand_u16()), swarm, topic)
        }

        _ => {
            println!("Unrecognized command!")
        }
    }
}

pub fn handle_message(
    mempool: &mut Mempool,
    message: MempoolMessage,
    swarm: &mut Swarm<Gossipsub>,
    topic: Topic,
) {
    match message {
        MempoolMessage::NewTransaction(tx, _) => mempool.new_transaction(tx),
        MempoolMessage::CancelTransaction(tx, _) => mempool.cancel_transaction(tx),
        MempoolMessage::NewBlock(block, _) => mempool.new_block(block),
        MempoolMessage::RetrieveTransactions(_) => {
            // Kinda dumb: drain txs into enum value, publish it, then take it out
            let new_msg = MempoolMessage::ListTransactions(
                mempool.transactions.drain(..).collect(),
                rand_u16(),
            );
            publish(&new_msg, swarm, topic);
            if let MempoolMessage::ListTransactions(m, _) = new_msg {
                mempool.transactions = m;
            }
        }
        MempoolMessage::ListTransactions(txs, _) => {
            // TODO: actually check if txs is more updated than transactions
            if mempool.transactions.len() < txs.len() {
                mempool.transactions = txs;
            }
        }
    }
}

fn publish(msg: &MempoolMessage, swarm: &mut Swarm<Gossipsub>, topic: Topic) {
    if let Err(e) = swarm
        .behaviour_mut()
        .publish(topic, serde_json::to_vec(msg).unwrap())
    {
        match e {
            PublishError::InsufficientPeers => {
                println!("No peers!")
            }
            x => println!("{}", x),
        }
    }
}

fn rand_u16() -> u16 {
    rand::thread_rng().gen()
}
