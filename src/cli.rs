use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about = "Shinobi  - An intelligence tool for the shadows of Solana.")]
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

        #[arg(long, help="Display the NFT image as ASCII art in the terminal")]
        image: bool,
    },
    Dossier{
        #[arg(required=true, help="The wallet address to analyze")]
        wallet_address:String,
    },
}