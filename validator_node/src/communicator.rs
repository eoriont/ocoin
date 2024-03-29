use std::{
    collections::hash_map::DefaultHasher,
    error::Error,
    hash::{Hash, Hasher},
    time::Duration,
};
use libp2p::{
    gossipsub::{
        self,
        error::{GossipsubHandlerError, PublishError},
        Gossipsub, GossipsubEvent, GossipsubMessage, IdentTopic as Topic, MessageAuthenticity,
        MessageId, ValidationMode,
    },
    identity,
    swarm::SwarmEvent,
    Multiaddr, PeerId, Swarm,
};
use crate::message::Message;
use libp2p::futures::StreamExt;
use async_std::{io::{stdout, WriteExt}};

pub struct Communicator {
    pub swarm: Swarm<Gossipsub>,
    topic: Topic,
    pub addresses: Vec<Multiaddr>,
}

impl Communicator {
    pub async fn new() -> Communicator {
        let (swarm, topic) = Communicator::init_swarm().await.unwrap();
        Communicator {
            swarm,
            topic,
            addresses: vec![],
        }
    }

    // pub fn request_synchronise(&mut self) {
    //     self.publish_message(&Message::RetrieveTransactions);
    // }

    pub fn publish_message(&mut self, msg: &Message) {
        // todo
        if let Err(e) = self
            .swarm
            .behaviour_mut()
            .publish(self.topic.clone(), serde_json::to_vec(msg).unwrap())
        {
            match e {
                PublishError::InsufficientPeers => {
                    println!("No peers!")
                }
                x => println!("{}", x),
            }
        }
    }

    async fn init_swarm() -> Result<(Swarm<Gossipsub>, Topic), Box<dyn Error>> {
        // Create a random PeerId
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());
        println!("Local peer id: {:?}", local_peer_id);

        // Set up an encrypted TCP Transport over the Mplex and Yamux protocols
        let transport = libp2p::development_transport(local_key.clone()).await?;

        // Create a Gossipsub topic
        let topic = Topic::new("blockchain");

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

        Ok((swarm, topic))
    }

    pub async fn connect(&mut self, addr: Multiaddr) {
        // Reach out to another node if specified
        match self.swarm.dial(addr.clone()) {
            Ok(_) => println!("Dialed {:?}", addr),
            Err(e) => println!("Dial {:?} failed: {:?}", addr, e),
        };
    }

    pub fn handle_network_event(&mut self, msg: SwarmEvent<GossipsubEvent, GossipsubHandlerError>) -> Option<Message> {
        println!("");
        match msg {
            SwarmEvent::Behaviour(GossipsubEvent::Message { message, .. }) => {
                let mempool_message = serde_json::from_slice::<Message>(&message.data).unwrap();
                return Some(mempool_message);
            }
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("Listening on {:?}", address);
                self.addresses.push(address);
            }
            x => {
                println!("{:?}", x)
            }
        }
        print!("> ");
        async_std::task::block_on(stdout().flush()).unwrap();
        None
    }

    pub async fn get_next_event(&mut self) -> SwarmEvent<GossipsubEvent, GossipsubHandlerError> {
        self.swarm.select_next_some().await
    }

}