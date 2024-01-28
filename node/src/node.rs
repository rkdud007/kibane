use anyhow::Result;
use libp2p::futures::StreamExt;
use libp2p::gossipsub::IdentTopic;
use libp2p::identity::Keypair;
use libp2p::swarm::{NetworkBehaviour, SwarmEvent};
use libp2p::{gossipsub, Multiaddr};
use std::sync::Arc;
use tokio::select;

use crate::common::Hash;
use crate::store::Store;

pub struct NodeConfig {
    /// An id of the network to connect to.
    pub network_id: String,
    /// The hash of the genesis block in network.
    pub genesis_hash: Option<Hash>,
    /// The keypair to be used as [`Node`]s identity.
    pub p2p_local_keypair: Keypair,
    /// List of bootstrap nodes to connect to and trust.
    pub p2p_bootnodes: Vec<Multiaddr>,
    /// List of the addresses where [`Node`] will listen for incoming connections.
    pub p2p_listen_on: Vec<Multiaddr>,
    /// The store for headers.
    pub store: Store,
}

impl NodeConfig {
    pub fn new(
        network_id: String,
        genesis_hash: Option<Hash>,
        p2p_local_keypair: Keypair,
        p2p_bootnodes: Vec<Multiaddr>,
        p2p_listen_on: Vec<Multiaddr>,
        store: Store,
    ) -> Self {
        Self {
            network_id,
            genesis_hash,
            p2p_local_keypair,
            p2p_bootnodes,
            p2p_listen_on,
            store,
        }
    }
}

pub(crate) fn gossipsub_ident_topic(network: &str, topic: &str) -> IdentTopic {
    let network = network.trim_matches('/');
    let topic = topic.trim_matches('/');
    let s = format!("/{network}/{topic}");
    IdentTopic::new(s)
}

#[derive(NetworkBehaviour)]
struct Behaviour {
    gossipsub: gossipsub::Behaviour,
}

#[derive(Debug)]
pub struct Node {
    store: Arc<Store>,
}

impl Node {
    pub async fn new(node_config: NodeConfig) -> Result<Self> {
        let store = Arc::new(node_config.store);
        let header_sub_topic = gossipsub_ident_topic(&node_config.network_id, "/header-sub/v0.0.1");
        let message_authenticity =
            gossipsub::MessageAuthenticity::Signed(node_config.p2p_local_keypair.clone());
        let config = gossipsub::ConfigBuilder::default()
            .validation_mode(gossipsub::ValidationMode::Strict)
            .validate_messages()
            .build()
            .unwrap();
        // build a gossipsub network behaviour
        let mut gossipsub: gossipsub::Behaviour =
            gossipsub::Behaviour::new(message_authenticity, config).unwrap();
        if gossipsub.subscribe(&header_sub_topic).is_ok() {
            println!("Subscribed to topic: {:?}", header_sub_topic);
        } else {
            eprintln!("Failed to subscribe to topic: {:?}", header_sub_topic);
        }
        //? 3. Swarm behaviour config
        let behaviour = Behaviour { gossipsub };
        let mut swarm = libp2p::SwarmBuilder::with_existing_identity(node_config.p2p_local_keypair)
            .with_tokio()
            .with_tcp(
                libp2p::tcp::Config::default(),
                libp2p::tls::Config::new,
                libp2p::yamux::Config::default,
            )
            .unwrap()
            .with_behaviour(|_| behaviour)
            .unwrap()
            .build();

        swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

        // Tell Swarm to listen on all bootnodes
        for addr in node_config.p2p_bootnodes {
            swarm.dial(addr.clone()).unwrap();
            println!("Dialed {addr}")
        }

        loop {
            select! {

                event = swarm.select_next_some() => {
                    match event {
                        SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {address:?}"),
                        SwarmEvent::Behaviour(BehaviourEvent::Gossipsub(gossipsub::Event::Message {
                            propagation_source,
                            message_id,
                            message,
                        })) => {
                            println!(
                                "Received message from {:?}: {}",
                                propagation_source,
                                String::from_utf8_lossy(&message.data)
                            );
                        },
                        SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                            println!("Connected to {:?}", peer_id);
                        },
                        SwarmEvent::ConnectionClosed { peer_id, .. } => {
                            println!("Disconnected from {:?}", peer_id);
                        },
                        _ => {}
                    }
                }
            }
        }
    }
}
