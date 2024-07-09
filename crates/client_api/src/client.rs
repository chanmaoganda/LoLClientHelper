use std::collections::HashMap;

use lazy_static::lazy_static;
use reqwest::{Certificate, IntoUrl};
use serde::Deserialize;

use league_model::{Abilities, ActivePlayer, AllGameData, Events, FullRunes, GameData, Player};
use crate::error::ClientError;

pub struct LoLClient {
    client: reqwest::Client,
}

impl Default for LoLClient {
    fn default() -> Self {
        Self::new()
    }
}

fn get_riot_certificate() -> Certificate {
    Certificate::from_pem(include_bytes!("riotgames.pem")).unwrap()
}

impl LoLClient {
    pub fn new() -> Self {
        Self::from_certificate(get_riot_certificate()).unwrap()
    }

    async fn get_data<T, U>(&self, url: U) -> Result<T, ClientError>
        where T: for<'de> Deserialize<'de>,
              U: IntoUrl 
    {
        let data = self.client.get(url)
            .send().await?
            .json::<T>().await?;
        Ok(data)
    }

    fn from_certificate(certificate: Certificate) -> Result<Self, ClientError> {
        Ok(LoLClient {
            client: reqwest::ClientBuilder::new()
                .add_root_certificate(certificate)
                .build()?
        })
    } 
}

impl LoLClient {

    pub async fn all_game_data(&self) -> Result<AllGameData, ClientError> {
        let url = api(IDENTIFIER_STR_MAP.get(&LoLInfoIdentifier::AllGameData).unwrap());
        let data = self.get_data(url).await?;
        Ok(data)
    }

    pub async fn active_player(&self) -> Result<ActivePlayer, ClientError> {
        let url = api(IDENTIFIER_STR_MAP.get(&LoLInfoIdentifier::ActivePlayer).unwrap());
        let data = self.get_data(url).await?;
        Ok(data)
    }

    pub async fn active_player_name(&self) -> Result<String, ClientError> {
        let url = api(IDENTIFIER_STR_MAP.get(&LoLInfoIdentifier::ActivePlayerName).unwrap());
        let data = self.get_data(url).await?;
        Ok(data)
    }

    pub async fn active_player_abilities(&self) -> Result<Abilities, ClientError> {
        let url = api(IDENTIFIER_STR_MAP.get(&LoLInfoIdentifier::ActivePlayerAbilities).unwrap());
        let data = self.get_data(url).await?;
        Ok(data)
    }

    pub async fn active_player_runes(&self) -> Result<FullRunes, ClientError> {
        let url = api(IDENTIFIER_STR_MAP.get(&LoLInfoIdentifier::ActivePlayerRunes).unwrap());
        let data = self.get_data(url).await?;
        Ok(data)
    }

    pub async fn player_list(&self) -> Result<Vec<Player>, ClientError> {
        let url = api(IDENTIFIER_STR_MAP.get(&LoLInfoIdentifier::PlayerList).unwrap());
        let data = self.get_data(url).await?;
        Ok(data)
    }

    pub async fn event_data(&self) -> Result<Events, ClientError> {
        let url = api(IDENTIFIER_STR_MAP.get(&LoLInfoIdentifier::EventData).unwrap());
        let data = self.get_data(url).await?;
        Ok(data)
    }

    pub async fn game_stats(&self) -> Result<GameData, ClientError> {
        let url = api(IDENTIFIER_STR_MAP.get(&LoLInfoIdentifier::GameStats).unwrap());
        let data = self.get_data(url).await?;
        Ok(data)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LoLInfoIdentifier {
    AllGameData,
    ActivePlayer,
    ActivePlayerName,
    ActivePlayerAbilities,
    ActivePlayerRunes,
    PlayerList,
    EventData,
    GameStats,
}

lazy_static! {
    static ref IDENTIFIER_STR_MAP: HashMap<LoLInfoIdentifier, &'static str> = {
        let mut map = HashMap::with_capacity(8);
        map.insert(LoLInfoIdentifier::AllGameData, "allgamedata");
        map.insert(LoLInfoIdentifier::ActivePlayer, "activeplayer");
        map.insert(LoLInfoIdentifier::ActivePlayerName, "activeplayername");
        map.insert(LoLInfoIdentifier::ActivePlayerAbilities, "activeplayerabilities");
        map.insert(LoLInfoIdentifier::ActivePlayerRunes, "activeplayerrunes");
        map.insert(LoLInfoIdentifier::PlayerList, "playerlist");
        map.insert(LoLInfoIdentifier::EventData, "eventdata");
        map.insert(LoLInfoIdentifier::GameStats, "gamestats");

        map
    };
}


pub fn api(endpoint: &str) -> String {
    format!("https://127.0.0.1:2999/liveclientdata/{}", endpoint)
}