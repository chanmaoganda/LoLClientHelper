use serde::Deserialize;

use crate::Summoner;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Champion {
    pub id: u32,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ParticipantIdentity {
    #[serde(rename = "participantId")]
    participant_id: u32,
    player: Summoner,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Participant {
    #[serde(rename = "championId")]
    champion_id: u32,
    #[serde(rename = "spell1Id")]
    spell1_id: u16,
    #[serde(rename = "spell2Id")]
    spell2_id: u16,
    stats: ChampionRecord,
    #[serde(rename = "teamId")]
    team_id: u16,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChampionRecord {
    champ_level: u16,
    kills: u16,
    deaths: u16,
    assists: u16,
    win: bool,
    // we can add more info about that
}