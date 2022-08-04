use clap::{Parser, Subcommand};


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    StartMempool {
        #[clap(value_parser)]
        address: Option<String>
    },
    NewTx {
        #[clap(value_parser)]
        wallet1: String,
        wallet2: String,
        amount: f64
    },
    DisplayBlockchain,
    DisplayBlock {
        #[clap(value_parser)]
        block_id: usize
    },
    DisplayCurrentTransactions,
    SaveBlockchain,
    LoadBlockchain,
    NewWallet {
        #[clap(value_parser)]
        name: String
    },
    Mine {
        #[clap(value_parser)]
        wallet: String
    }
}
