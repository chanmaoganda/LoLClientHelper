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
    fn get_player(&self) -> &Participant {
        self.participants.get(0).unwrap()
    }

    pub fn get_game_mode(&self) -> &str {
        &self.game_mode
    }

    pub fn get_game_info(&self) -> String {
        format!("{}, Mode: {}, Type: {}", 
            self.game_creation_date, self.game_mode, self.game_type)
    }

    pub fn get_timestamp(&self) -> String {
        format!("{}", self.game_creation_date)
    }

    pub fn get_date(&self) -> String {
        let mut timestamp = self.game_creation_date.clone();
        let _ = timestamp.split_off(10);
        timestamp
    }

    pub fn get_name_title(&self) -> String {
        self.get_player().name_title()
    }

    pub fn get_kda_result(&self) -> String {
        self.get_player().kda_result()
    }

    pub fn get_win_status(&self) -> bool {
        self.get_player().win_status()
    }

    pub fn get_champion_icon_url(&self) -> String {
        self.get_player().champion_url()
    }

    pub fn get_summoner_spell_urls(&self) -> (String, String) {
        self.get_player().summoner_spell_urls()
    }
}