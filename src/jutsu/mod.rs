pub mod unmask;

use anyhow::Result;

pub async fn run(mint_address:String)->Result<()>{
    println!("Running to unmask target:{}",mint_address);
    Ok(())
}