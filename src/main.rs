use anyhow::Result;
use lcu_api::{LcuClient, Summoner};


#[tokio::main]
async fn main() -> Result<()>{
    let lcu_client = LcuClient::new();
    let summoner: Summoner = lcu_client.send("/lol-summoner/v1/current-summoner").await?;
    println!("Summoner: {:?}", summoner);
    Ok(())
}