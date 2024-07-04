use lol_game_client_api::api::GameClient;
use tokio;

#[tokio::main]
async fn main() {
    let client = GameClient::new();
    let active_player = client.active_player().await.unwrap();
    
    println!("Stats Runes: {:?}", active_player.full_runes.stat_runes);

    let players = client.player_list().await.unwrap();
    
}