use serde::Deserialize;

use crate::common::PartialRunes;



#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub champion_name: String,
    pub summoner_name: String,
    pub summoner_spells: SummonerSpells,
    pub is_bot: bool,
    pub is_dead: bool,
    pub items: Vec<Item>,
    pub level: usize,
    pub position: String,
    pub raw_champion_name: String,
    pub respawn_timer: f64,
    pub runes: PartialRunes,
    pub scores: Scores,
    #[serde(rename = "skinID")]
    pub skin_id: usize,
    pub team: Team,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Scores {
    pub assists: usize,
    pub creep_score: usize,
    pub deaths: usize,
    pub kills: usize,
    pub ward_score: f64,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Item {}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SummonerSpell {
    pub display_name: String,
    pub raw_description: String,
    pub raw_display_name: String
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SummonerSpells {
    pub summoner_spell_one: SummonerSpell,
    pub summoner_spell_two: SummonerSpell,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum Team {
    #[serde(rename = "ORDER")]
    Order,
    #[serde(rename = "CHAOS")]
    Chaos,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Ability {
    pub ability_level: Option<u8>,
    pub display_name: String,
    pub id: String,
    pub raw_description: String,
    pub raw_display_name: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Abilities {
    #[serde(rename = "Q")]
    pub q: Ability,
    #[serde(rename = "W")]
    pub w: Ability,
    #[serde(rename = "E")]
    pub e: Ability,
    #[serde(rename = "R")]
    pub r: Ability,
    #[serde(rename = "Passive")]
    pub passive: Ability,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChampionStats {
    pub ability_power: f64,
    pub armor: f64,
    pub armor_penetration_flat: f64,
    pub attack_damage: f64,
    pub attack_range: f64,
    pub attack_speed: f64,
    pub bonus_armor_penetration_percent: f64,
    pub bonus_magic_penetration_percent: f64,
    pub crit_chance: f64,
    pub crit_damage: f64,
    pub current_health: f64,
    pub heal_shield_power: Option<f64>, // Optional because not found in official docs
    pub health_regen_rate: f64,
    pub life_steal: f64,
    pub magic_lethality: f64,
    pub magic_penetration_flat: f64,
    pub magic_penetration_percent: f64,
    pub magic_resist: f64,
    pub max_health: f64,
    pub move_speed: f64,
    pub omnivamp: Option<f64>, // Optional because not found in official docs
    pub physical_lethality: f64,
    pub physical_vamp: Option<f64>, // Optional because not found in official docs
    pub resource_max: f64,
    pub resource_regen_rate: f64,
    pub resource_type: String,
    pub resource_value: f64,
    pub spell_vamp: f64,
    pub tenacity: f64,
}