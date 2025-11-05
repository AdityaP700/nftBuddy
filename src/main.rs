mod rpc;
mod cli;
mod display;
mod errors;
mod jutsu;
mod model;
use cli::{Cli,Commands};
use clap::Parser;
use dotenv::dotenv;
#[tokio::main]
async fn main()->anyhow::Result<()> {
    dotenv().ok();
    let cli = cli::Cli::parse();

    //it decides which specialist to send the thing
    match cli.command{
     Commands::Unmask {mint_address}=>{
        //we dispatch the unmask command to the jutsu module
       let report = jutsu::unmask::run(mint_address).await?;
        display::print_unmask_report(report);
     }
    }
    Ok(())
}
