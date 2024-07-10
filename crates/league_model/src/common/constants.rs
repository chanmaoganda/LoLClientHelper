use std::collections::HashMap;

use serde::Deserialize;

pub struct ConstHandler {
    pub champions: HashMap<String, ConstChampion>,
}

impl ConstHandler {
    pub fn new() -> Self {
        Self {
            champions: Self::get_champions(),
        }
    }

    pub fn get_champion_by_id(&self, id: u32) -> &ConstChampion {
        self.champions.values().find(|champion| champion.key.parse::<u32>().unwrap() == id).unwrap()
    }

    fn get_champions() -> HashMap<String, ConstChampion> {
        let json_file_path = std::path::Path::new("./assets/champion.json");
        let file = std::fs::File::open(json_file_path).unwrap();
        let champion_file: ChampionAssets = serde_json::from_reader(file).unwrap();
        champion_file.data
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