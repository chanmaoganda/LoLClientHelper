/*!
A Library for interacting with the League of Legends Client API and the League of Legends Model.

# Interacting with LCU (League Client Update)
``` rust
pub async fn test() -> anyhow::Result<()> {
    let lcu_client = lcu_api::LcuClient::new();
    let histories = lcu_client.get_summoner_match_histories().await?;
    println!("{:?}", histories);
    Ok(())
}
```
*/
pub use league_model::*;
pub use lcu_api::*;
pub use client_api::*;