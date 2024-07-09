use anyhow::Result;
use lcu_api::LcuClient;
use league_model::GameHistoryQuery;


#[tokio::main]
async fn main() -> Result<()>{
    let lcu_client = LcuClient::new();
    let history: GameHistoryQuery = lcu_client.get("/lol-match-history/v1/products/lol/e83c55c9-2308-5e3a-8ce2-1a5dc33768c6/matches?begIndex=0&endIndex=4").await?;
    println!("{:?}", history);

    Ok(())
}