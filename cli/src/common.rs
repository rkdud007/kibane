use clap::{Parser, ValueEnum};
use node::{
    network::{get_network_genesis, get_network_id, get_p2p_bootnodes},
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
}

pub fn run() {
    let args = Cli::parse();
    // 1. Get the network from the command line arguments
    let network = args.network.into();
    // 2. Get corresponding bootstrap nodes
    let p2p_bootnodes = get_p2p_bootnodes(network);

    // 3. Relevant network configuration libp2p
    let p2p_local_keypair = libp2p::identity::Keypair::generate_ed25519();
    let network_id = get_network_id(network);
    let network_genesis = get_network_genesis(network);
    println!("p2p_bootnodes: {:?}", p2p_bootnodes);

    let store = Store::new();
    println!("initiating new store")
}
