use serde::Deserialize;

use crate::{Participant, ParticipantIdentity};

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct GameHistoryQuery {
    #[serde(rename = "accountId")]
    account_id: u64,
    #[serde(rename = "platformId")]
    platform_id: String,
    #[serde(rename = "games")]
    game_history: GameHistory,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GameHistory {
    game_count: u32,
    game_index_begin: u32,
    game_index_end: u32,
    #[serde(rename = "games")]
    game_list: Vec<Game>,    
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
}
