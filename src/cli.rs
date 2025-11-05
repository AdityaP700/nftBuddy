use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about = "Solana Shinobi 🥷 - An intelligence tool for the shadows of Solana.")]
#[command(propagate_version=true)]
pub struct Cli{
   #[command(subcommand)]
   pub command : Commands,
}

#[derive(Subcommand,Debug)]
pub enum Commands{
    Unmask{
        #[arg(required=true, help="The mint address of the NFT to unmask")]
        mint_address:String,
    },
}