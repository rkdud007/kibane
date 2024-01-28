use anyhow::{Ok, Result};
use libp2p::identity::Keypair;
use libp2p::Multiaddr;
use std::sync::Arc;
use tokio::spawn;

use crate::common::Hash;
use crate::store::Store;
use crate::swarm::SwarmRunner;

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

#[derive(Debug)]
pub struct Node {
    store: Arc<Store>,
}

impl Node {
    pub async fn new(node_config: NodeConfig) -> Result<Self> {
        let mut swarm_runner = SwarmRunner::new(&node_config)?;
        spawn(async move {
            swarm_runner.run().await;
        });
        let store = Arc::new(node_config.store);

        Ok(Self { store })
    }
}
