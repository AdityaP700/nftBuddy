mod rpc;
mod cli;
mod display;
mod errors;
mod jutsu;
mod model;
use cli::Commands;
use clap::Parser;
#[tokio::main]
async fn main()->anyhow::Result<()> {
    let cli = cli::Cli::parse();

    //it decides which specialist to send the thing
    match cli.command{
     Commands::Unmask {mint_address, image}=>{
        //we dispatch the unmask command to the jutsu module
       let report = jutsu::unmask::run(mint_address, image).await?;
        display::print_unmask_report(report).await;
     }
     Commands::Dossier {wallet_address}=>{
        //we dispatch the dossier command to analyze a wallet
       let report = jutsu::dossier::run(wallet_address).await?;
        display::print_dossier_report(report).await;
     }
    }
    Ok(())
}
