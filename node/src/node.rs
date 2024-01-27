use std::sync::Arc;

use libp2p::identity::Keypair;
use libp2p::Multiaddr;

use crate::common::Hash;
use crate::p2p::Peer2Peer;
use crate::store::Store;
use crate::syncer::Syncer;

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
    p2p: Arc<Peer2Peer>,
    store: Arc<Store>,
    syncer: Arc<Syncer>,
}

impl Node {
    pub fn new(config: NodeConfig) -> Self {
        let store = Arc::new(config.store);

        let p2p = Arc::new(Peer2Peer::new(store.clone()));
        let syncer = Arc::new(Syncer::new(store.clone()));
        Self { p2p, store, syncer }
    }
}
