use serde::Deserialize;

use crate::{Participant, ParticipantIdentity};

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct GameHistoryQuery {
    #[serde(rename = "accountId")]
    pub account_id: u64,
    #[serde(rename = "platformId")]
    pub platform_id: String,
    #[serde(rename = "games")]
    pub game_history: GameHistory,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GameHistory {
    pub game_count: u32,
    pub game_index_begin: u32,
    pub game_index_end: u32,
    #[serde(rename = "games")]
    pub game_list: Vec<Game>,    
}

impl GameHistory {
    pub fn get_game_by_map(&self, map_id: u32) -> Vec<&Game> {
        self.game_list.iter().filter(|game| game.map_id == map_id).collect()
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    game_creation_date: String,
    #[serde(rename = "gameId")]
    game_id: u64,
    #[serde(rename = "mapId")]
    map_id: u32,
    participant_identities: Vec<ParticipantIdentity>,
    participants: Vec<Participant>,
    #[serde(rename = "queueId")]
    queue_id: u16,
    #[serde(rename = "seasonId")]
    season_id: u16,
    game_mode: String,
    game_type: String,
}

impl Game {
    pub fn get_game_info(&self) -> String {
        format!("{}, Mode: {}, Type: {}", 
            self.game_creation_date, self.game_mode, self.game_type)
    }

    pub fn get_player_info(&self) -> String {
        let champion = self.participants.get(0).unwrap();
        format!("player info: {}", champion.name_title())
    }

    pub fn get_kda_result(&self) -> String {
        let champion = self.participants.get(0).unwrap();
        champion.kda_result()
    }

    pub fn get_champion_icon_url(&self) -> String {
        format!("https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/v1/champion-icons/{}.png", 
            self.participants.get(0).unwrap().champion_key())
    }
}