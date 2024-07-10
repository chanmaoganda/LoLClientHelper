use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Summoner {
    pub display_name: Option<String>,
    pub game_name: Option<String>,
    #[serde(rename = "puuid")]
    pub puuid: String,
    #[serde(rename = "summonerId")]
    pub summoner_id: u64,
    pub tag_line: Option<String>,
}