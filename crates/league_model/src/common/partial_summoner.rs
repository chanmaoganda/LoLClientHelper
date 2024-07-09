use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PartialSummoner {
    pub puuid: String,
    #[serde(rename = "summonerId")]
    pub summoner_id: u64,
    pub tag_line: String,
}