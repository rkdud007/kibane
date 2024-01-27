use clap::{Parser, ValueEnum};
use libp2p::Multiaddr;
use node::{
    network::{get_network_genesis, get_network_id, get_p2p_bootnodes},
    node::{Node, NodeConfig},
    store::Store,
};

#[derive(Debug, Clone, ValueEnum)]
pub(crate) enum ArgsNetwork {
    // Mainnet,
    // Arabica,
    Mocha,
    // Private,
}

impl From<ArgsNetwork> for node::network::Network {
    fn from(val: ArgsNetwork) -> Self {
        match val {
            ArgsNetwork::Mocha => node::network::Network::Mocha,
        }
    }
}

#[derive(Debug, Parser)]
struct Cli {
    #[arg(short = 'n', long)]
    #[arg(value_name = "NETWORK")]
    #[arg(help = "The network to connect to")]
    network: ArgsNetwork,

    #[arg(short = 'l', long)]
    #[arg(value_name = "LISTEN_ADDRS")]
    #[arg(help = "The addresses to listen on")]
    listen_addrs: Vec<Multiaddr>,
}

pub fn run() {
    let args = Cli::parse();
    // 1. Get the network from the command line arguments
    let network = args.network.into();

    // 2. Config network arguments
    let p2p_bootnodes = get_p2p_bootnodes(network);
    let p2p_local_keypair = libp2p::identity::Keypair::generate_ed25519();
    let network_id = get_network_id(network);
    let network_genesis = get_network_genesis(network);
    println!("p2p_bootnodes: {:?}", p2p_bootnodes);

    // 3. Initiate a new node to sync with other peers
    let store = Store::new();
    let node_config = NodeConfig::new(
        network_id.to_string(),
        network_genesis,
        p2p_local_keypair,
        p2p_bootnodes,
        args.listen_addrs,
        store,
    );

    let node = Node::new(node_config);
    println!("initiating new store:{:?}", node)
}
