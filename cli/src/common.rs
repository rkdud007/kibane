use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, ValueEnum)]
pub(crate) enum ArgNetwork {
    // Mainnet,
    // Arabica,
    Mocha,
    // Private,
}

#[derive(Debug, Parser)]
struct Cli {
    #[arg(short = 'n', long)]
    #[arg(value_name = "NETWORK")]
    #[arg(help = "The network to connect to")]
    network: ArgNetwork,
}

pub fn run() {
    let args = Cli::parse();
    println!("{:?}", args);
}
