use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Summoner {
    pub display_name: String,
    pub game_name: String,
    #[serde(rename = "puuid")]
    pub puuid: String,
    #[serde(rename = "summonerId")]
    pub summoner_id: u64,
}