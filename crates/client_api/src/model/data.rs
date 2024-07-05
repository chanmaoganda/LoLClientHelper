use serde::Deserialize;

use super::{event::Events, player::{Abilities, ChampionStats}, FullRunes, Player};

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum GameMode {
    #[serde(rename = "CLASSIC")]
    Classic,
    #[serde(rename = "ARAM")]
    Aram,
    #[serde(rename = "PRACTICETOOL")]
    Practicetool,
    #[serde(rename = "CHERRY")]
    Arena,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GameData {
    pub game_mode: GameMode,
    pub game_time: f64,
    pub map_name: String,
    pub map_number: usize,
    pub map_terrain: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AllGameData {
    pub active_player: ActivePlayer,
    pub all_players: Vec<Player>,
    pub events: Events,
    pub game_data: GameData,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ActivePlayer {
    pub abilities: Abilities,
    pub champion_stats: ChampionStats,
    pub current_gold: f64,
    pub full_runes: FullRunes,
    pub level: usize,
    pub summoner_name: String,
}