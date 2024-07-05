use std::collections::HashMap;

use lazy_static::lazy_static;
use reqwest::{Certificate, IntoUrl};
use serde::Deserialize;

use crate::model::{Abilities, ActivePlayer, AllGameData, Events, FullRunes, GameData, Player};
use crate::error::ClientError;

pub struct LoLClient {
    client: reqwest::Client,
}

impl Default for LoLClient {
    fn default() -> Self {
        Self::new()
    }
}

fn get_riot_certificate() -> Certificate{
    Certificate::from_pem(include_bytes!("riotgames.cer")).unwrap()
}

impl LoLClient {
    pub fn new() -> Self {
        Self::from_certificate(get_riot_certificate()).unwrap()
    }

    async fn get_data<T, U>(&self, endpoint: U) -> Result<T, ClientError>
        where T: for<'de> Deserialize<'de>,
              U: IntoUrl 
    {
        let data = self.client.get(endpoint).send().await?.json::<T>().await?;
        Ok(data)
    }

    pub fn from_certificate(certificate: Certificate) -> Result<Self, ClientError> {
        Ok(LoLClient {
            client: reqwest::ClientBuilder::new()
                .add_root_certificate(certificate)
                .build()?
        })
    } 
}

impl LoLClient {
    pub async fn get_info(&self, identifier: LoLInfoIdentifier) -> Result<LoLInfoWrapper, ClientError> {
        let info_type = IDENTIFIER_STR_MAP.get(&identifier).unwrap().to_string();
        let url = api(&info_type);
        
        match identifier {
            LoLInfoIdentifier::AllGameData => {
                let data = self.get_data(url).await?;
                Ok(LoLInfoWrapper::AllGameData(data))
            }
            LoLInfoIdentifier::ActivePlayer => {
                let data = self.get_data(url).await?;
                Ok(LoLInfoWrapper::ActivePlayer(data))
            },
            LoLInfoIdentifier::ActivePlayerName => {
                let data = self.get_data(url).await?;
                Ok(LoLInfoWrapper::ActivePlayerName(data))
            },
            LoLInfoIdentifier::ActivePlayerAbilities => {
                let data = self.get_data(url).await?;
                Ok(LoLInfoWrapper::ActivePlayerAbilities(data))
            },
            LoLInfoIdentifier::ActivePlayerRunes => {
                let data = self.get_data(url).await?;
                Ok(LoLInfoWrapper::ActivePlayerRunes(data))
            },
            LoLInfoIdentifier::PlayerList => {
                let data = self.get_data(url).await?;
                Ok(LoLInfoWrapper::PlayerList(data))
            },
            LoLInfoIdentifier::EventData => {
                let data = self.get_data(url).await?;
                Ok(LoLInfoWrapper::EventData(data))
            },
            LoLInfoIdentifier::GameStats => {
                let data = self.get_data(url).await?;
                Ok(LoLInfoWrapper::GameStats(data))
            },
        }
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

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum LoLInfoWrapper {
    AllGameData(AllGameData),
    ActivePlayer(ActivePlayer),
    ActivePlayerName(String),
    ActivePlayerAbilities(Abilities),
    ActivePlayerRunes(FullRunes),
    PlayerList(Vec<Player>),
    EventData(Events),
    GameStats(GameData),
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