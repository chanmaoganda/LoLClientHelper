mod model;
mod client;
mod error;

pub use client::LoLClient;
pub use client::LoLInfoIdentifier::{*};

pub use model::{Abilities, ActivePlayer, AllGameData, Drake, Player};



#[deprecated(since = "0.1.0", note = "no longer needed for testing the functionality of the license")]
pub mod some_test_printing {

    use super::*;

    pub async fn test_all() -> Result<(), Box<dyn std::error::Error>> {
        let client = LoLClient::new();
        some_test_printing::all_game_data(&client).await?;
        some_test_printing::active_player(&client).await?;
        
        some_test_printing::active_player_name(&client).await?;
        some_test_printing::active_player_runes(&client).await?;
        some_test_printing::player_list(&client).await?;
        some_test_printing::event_data(&client).await?;
        some_test_printing::game_stats(&client).await?;
        Ok(())
    }

    pub async fn all_game_data(client: &LoLClient) -> Result<(), Box<dyn std::error::Error>>{
        let data = client.get_info(AllGameData).await?;
        let names = data.all_game_data()?;
        println!("names: {:?}", names);
        Ok(())
    }

    pub async fn active_player(client: &LoLClient) -> Result<(), Box<dyn std::error::Error>>{
        let data = client.get_info(ActivePlayer).await?;
        let player = data.active_player()?;
        println!("active player: {:?}", player);
        Ok(())
    }

    pub async fn active_player_name(client: &LoLClient) -> Result<(), Box<dyn std::error::Error>>{
        let data = client.get_info(ActivePlayerName).await?;
        let player = data.active_player_name()?;
        println!("active player name: {:?}", player);
        Ok(())
    }

    pub async fn active_player_runes(client: &LoLClient) -> Result<(), Box<dyn std::error::Error>>{
        let data = client.get_info(ActivePlayerRunes).await?;
        let runes = data.active_player_runes()?;
        println!("active player runes: {:?}", runes);
        Ok(())
    }

    pub async fn player_list(client: &LoLClient) -> Result<(), Box<dyn std::error::Error>>{
        let data = client.get_info(PlayerList).await?;
        let players = data.player_list()?;
        println!("player list: {:?}", players);
        Ok(())
    }

    pub async fn event_data(client: &LoLClient) -> Result<(), Box<dyn std::error::Error>>{
        let data = client.get_info(EventData).await?;
        let events = data.event_data()?;
        println!("event data: {:?}", events);
        Ok(())
    }

    pub async fn game_stats(client: &LoLClient) -> Result<(), Box<dyn std::error::Error>>{
        let data = client.get_info(GameStats).await?;
        let stats = data.game_stats()?;
        println!("game stats: {:?}", stats);
        Ok(())
    }
}