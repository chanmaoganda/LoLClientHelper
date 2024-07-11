use serde::Deserialize;
use lazy_static::lazy_static;

use crate::{ConstHandler, Summoner};

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

lazy_static! {
    static ref HANDLER: ConstHandler = ConstHandler::new();
}

impl Participant {
    pub fn name_title(&self) -> String {
        let champion = HANDLER.get_champion_by_id(self.champion_id);
        format!("player info: {}", champion.name_title())
    }

    pub fn kda_result(&self) -> String {
        format!("KDA: {}/{}/{} Win: {}", self.stats.kills, self.stats.deaths, self.stats.assists, self.stats.win)
    }

    pub fn champion_url(&self) -> String {
        format!("https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/v1/champion-icons/{}.png", 
            self.champion_id)
    }

    pub fn summoner_spell_urls(&self) -> (String, String){
        let summoner_spell1 = HANDLER.get_summoner_spell_by_id(self.spell1_id);
        let summoner_spell2 = HANDLER.get_summoner_spell_by_id(self.spell2_id);
        (summoner_spell1.icon_url(), summoner_spell2.icon_url())
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChampionRecord {
    pub champ_level: u16,
    pub kills: u16,
    pub deaths: u16,
    pub assists: u16,
    pub win: bool,
    // we can add more info about that
}