use std::collections::HashMap;

use serde::Deserialize;

pub struct ConstHandler {
    pub champions: HashMap<String, ConstChampion>,
    pub summoner_spells: HashMap<String, ConstSummonerSpell>,
}

impl ConstHandler {
    pub fn new() -> Self {
        Self {
            champions: Self::get_champions(),
            summoner_spells: Self::get_summoner_spells(),
        }
    }

    pub fn get_champion_by_id(&self, id: u32) -> &ConstChampion {
        self.champions.values().find(|champion| champion.key.parse::<u32>().unwrap() == id).unwrap()
    }

    pub fn get_summoner_spell_by_id(&self, id: u16) -> &ConstSummonerSpell {
        self.summoner_spells.values().find(|spell| spell.key.parse::<u16>().unwrap() == id).unwrap()
    }

    fn get_champions() -> HashMap<String, ConstChampion> {
        let json_file_path = std::path::Path::new("./assets/champion.json");
        let file = std::fs::File::open(json_file_path).unwrap();
        let champion_file: ChampionAssets = serde_json::from_reader(file).unwrap();
        champion_file.data
    }

    fn get_summoner_spells() -> HashMap<String, ConstSummonerSpell> {
        let json_file_path = std::path::Path::new("./assets/summoner_spell.json");
        let file = std::fs::File::open(json_file_path).unwrap();
        let summoner_spell_file: SummonerSpellAssets = serde_json::from_reader(file).unwrap();
        summoner_spell_file.data
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ChampionAssets {
    data: HashMap<String, ConstChampion>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ConstChampion {
    key: String,
    name: String,
    title: String,
}

impl ConstChampion {
    pub fn name_title(&self) -> String {
        format!("{} - {}", self.title, self.name)
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct SummonerSpellAssets {
    data: HashMap<String, ConstSummonerSpell>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ConstSummonerSpell {
    pub name: String,
    pub description: String,
    pub key: String,
    pub image: SpellImageInfo,
}

impl ConstSummonerSpell {
    pub fn icon_url(&self) -> String {
        format!("http://ddragon.leagueoflegends.com/cdn/12.1.1/img/spell/{}", self.image.full)
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct SpellImageInfo {
    pub full: String,
}