use anyhow::Result;
use instant::{Duration, Instant};
use libp2p::futures::StreamExt;
use libp2p::gossipsub::IdentTopic;
use libp2p::multiaddr::Protocol;
use libp2p::swarm::NetworkBehaviour;
use libp2p::{gossipsub, kad, Multiaddr, PeerId, StreamProtocol, Swarm};

use crate::node::NodeConfig;

#[derive(NetworkBehaviour)]
struct Behaviour {
    gossipsub: gossipsub::Behaviour,
    kademlia: kad::Behaviour<kad::store::MemoryStore>,
}

pub(crate) struct SwarmInterval(tokio::time::Interval);

impl SwarmInterval {
    pub(crate) async fn new(dur: Duration) -> Self {
        let mut inner = tokio::time::interval(dur);

        // In Tokio the first tick returns immediately, so we
        // consume to it to create an identical cross-platform
        // behavior.
        inner.tick().await;

        SwarmInterval(inner)
    }

    pub(crate) async fn tick(&mut self) {
        self.0.tick().await;
    }
}

pub(crate) fn gossipsub_ident_topic(network: &str, topic: &str) -> IdentTopic {
    let network = network.trim_matches('/');
    let topic = topic.trim_matches('/');
    let s = format!("/{network}/{topic}");
    IdentTopic::new(s)
}

pub(crate) fn celestia_protocol_id(network: &str, protocol: &str) -> StreamProtocol {
    let network = network.trim_matches('/');
    let network = format!("/celestia/{network}");
    protocol_id(&network, protocol)
}

pub(crate) fn protocol_id(network: &str, protocol: &str) -> StreamProtocol {
    let network = network.trim_matches('/');
    let protocol = protocol.trim_matches('/');
    let s = format!("/{network}/{protocol}");
    StreamProtocol::try_from_owned(s).expect("does not start from '/'")
}

pub struct SwarmRunner {
    swarm: Swarm<Behaviour>,
}

impl SwarmRunner {
    pub fn new(node_config: &NodeConfig) -> Result<Self> {
        let kademlia = init_kademlia(node_config)?;
        let gossipsub = init_gossip(node_config)?;
        let behaviour = Behaviour {
            gossipsub,
            kademlia,
        };
        let local_keypair = node_config.p2p_local_keypair.clone();
        let mut swarm = libp2p::SwarmBuilder::with_existing_identity(local_keypair)
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
        for addr in &node_config.p2p_bootnodes {
            swarm.dial(addr.clone()).unwrap();
            println!("Dialed {addr}")
        }
        Ok(SwarmRunner { swarm })
    }

    pub async fn run(&mut self) {
        let mut report_interval = SwarmInterval::new(Duration::from_secs(60)).await;
        let mut kademlia_interval = SwarmInterval::new(Duration::from_secs(30)).await;
        let mut kademlia_last_bootstrap = Instant::now();

        // Initiate discovery
        let _ = self.swarm.behaviour_mut().kademlia.bootstrap();

        loop {
            tokio::select! {
                _ = report_interval.tick() => {
                    println!("Report interval tick");
                }
                _ = kademlia_interval.tick() => {
                    println!("Kademlia interval tick");
                }
                ev = self.swarm.select_next_some() => {
                    println!("Event: {:?}", ev);
                }
            }
        }
    }
}

fn init_kademlia(node_config: &NodeConfig) -> Result<kad::Behaviour<kad::store::MemoryStore>> {
    let local_peer_id = PeerId::from(node_config.p2p_local_keypair.public());
    let mut config = kad::Config::default();

    let protocol_id = celestia_protocol_id(&node_config.network_id, "/kad/1.0.0");

    config.set_protocol_names(vec![protocol_id]);

    let store = kad::store::MemoryStore::new(local_peer_id);
    let mut kademlia = kad::Behaviour::with_config(local_peer_id, store, config);

    for addr in &node_config.p2p_bootnodes {
        if let Some(peer_id) = addr.peer_id() {
            kademlia.add_address(&peer_id, addr.to_owned());
        }
    }

    if !node_config.p2p_listen_on.is_empty() {
        kademlia.set_mode(Some(kad::Mode::Server));
    }

    Ok(kademlia)
}

fn init_gossip(node_config: &NodeConfig) -> Result<gossipsub::Behaviour> {
    let header_sub_topic = gossipsub_ident_topic(&node_config.network_id, "/header-sub/v0.0.1");
    let message_authenticity =
        gossipsub::MessageAuthenticity::Signed(node_config.p2p_local_keypair.clone());
    let config = gossipsub::ConfigBuilder::default()
        .validation_mode(gossipsub::ValidationMode::Strict)
        .validate_messages()
        .build()
        .unwrap();
    let mut gossipsub: gossipsub::Behaviour =
        gossipsub::Behaviour::new(message_authenticity, config).unwrap();
    if gossipsub.subscribe(&header_sub_topic).is_ok() {
        println!("Subscribed to topic: {:?}", header_sub_topic);
    } else {
        eprintln!("Failed to subscribe to topic: {:?}", header_sub_topic);
    }
    Ok(gossipsub)
}

pub(crate) trait MultiaddrExt {
    fn peer_id(&self) -> Option<PeerId>;
}

impl MultiaddrExt for Multiaddr {
    fn peer_id(&self) -> Option<PeerId> {
        self.iter().find_map(|proto| match proto {
            Protocol::P2p(peer_id) => Some(peer_id),
            _ => None,
        })
    }
}
