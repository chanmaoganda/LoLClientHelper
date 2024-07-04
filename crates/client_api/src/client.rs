use reqwest::{Certificate, IntoUrl};
use serde::Deserialize;

use crate::model::{FullRunes, Player};
use crate::error::QueryError;
use crate::api;


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

    async fn get_data<T, U>(&self, endpoint: U) -> Result<T, QueryError>
        where T: for<'de> Deserialize<'de>,
              U: IntoUrl 
    {
        let data = self.client.get(endpoint).send().await?.json::<T>().await?;
        Ok(data)
    }

    pub fn from_certificate(certificate: Certificate) -> Result<Self, QueryError> {
        Ok(LoLClient {
            client: reqwest::ClientBuilder::new()
                .add_root_certificate(certificate)
                .build()?
        })
    } 
}

impl LoLClient {
    pub async fn player_list(&self) -> Result<Vec<Player>, QueryError> {
        self.get_data(api!("playerlist")).await
    }

    pub async fn active_player_runes(&self) -> Result<FullRunes, QueryError> {
        self.get_data(api!("activeplayerrunes")).await
    }
}

#[macro_export]
macro_rules! api {
    ($endpoint: expr) => {
        concat!("https://127.0.0.1:2999/liveclientdata/", $endpoint)
    }
}
