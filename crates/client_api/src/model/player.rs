use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Player {
    pub champion_name: String,
    pub summoner_name: String,
    pub summoner_spells: SummonerSpells,

}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SummonerSpell {
    pub display_name: String,
    pub raw_description: String,
    pub raw_display_name: String
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SummonerSpells {
    pub first_summoner_spell: SummonerSpell,
    pub second_summoner_spell: SummonerSpell,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum Team {
    #[serde(rename = "ORDER")]
    Order,
    #[serde(rename = "CHAOS")]
    Chaos,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
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