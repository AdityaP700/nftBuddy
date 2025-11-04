mod cli;
mod display;
mod errors;
mod jutsu;

use cli::{Cli,Commands};
use clap::Parser;

#[tokio::main]
async fn main()->Result<()> {
    //it parses the command from the outside world
    let cli = cli::Cli::parse();

    //it decides which specialist to send the thing
    match cli.command{
     Commands::Unmask {mint_address}=>{
        //we dispatch the unmask command to the jutsu module
        jutsu::unmask::run(mint_address).await?;
     }
    }
    Ok(())
}
