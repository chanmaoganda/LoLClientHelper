use lazy_static::lazy_static;
use league_model::{GameHistoryQuery, ModelError, Summoner};
use reqwest::{Certificate, Client, ClientBuilder};
use serde::Deserialize;

use crate::web_socket::{get_authorize_info, AuthorizeInfo, LcuError};

#[derive(Debug)]
pub struct LcuClient {
    client: Client,
    auth_info: AuthorizeInfo,
}

impl LcuClient {
    pub fn new() -> Self {
        Self::from_certificate(get_riot_certificate()).unwrap()
    }

    fn from_certificate(certificate: Certificate) -> Result<Self, LcuError> {
        Ok(Self {
            client: ClientBuilder::new()
               .add_root_certificate(certificate)
               .build()
               .unwrap(),
            auth_info: get_authorize_info()?,
        })
    }

    fn endpoint_url(&self, endpoint: &str) -> String {
        format!("https://127.0.0.1:{}{}", self.auth_info.port, endpoint)
    }

    pub async fn get<T>(&self, endpoint: &str) -> Result<T, reqwest::Error>
        where T : for<'de> Deserialize<'de>, {
        let url = self.endpoint_url(endpoint);
        let response = self.client.get(url)
           .header("Authorization", &self.auth_info.auth_token)
           .send().await?
           .json::<T>().await?;
        Ok(response)
    }

    pub async fn post<T> (&self, endpoint: &str) -> Result<T, reqwest::Error>
        where T: for<'de> Deserialize<'de>, {
        let url = self.endpoint_url(endpoint);
        let response = self.client.post(url)
           .header("Authorization", &self.auth_info.auth_token)
           .send().await?
           .json::<T>().await?;
        Ok(response)
    }

    pub async fn get_text(&self, endpoint: &str) -> Result<String, reqwest::Error> {
        let url = self.endpoint_url(endpoint);
        let response = self.client.get(url)
           .header("Authorization", &self.auth_info.auth_token)
           .send().await?
           .text().await?;
        Ok(response)
    }

    pub async fn post_text(&self, endpoint: &str) -> Result<String, reqwest::Error> {
        let url = self.endpoint_url(endpoint);
        let response = self.client.post(url)
           .header("Authorization", &self.auth_info.auth_token)
           .send().await?
           .text().await?;
        Ok(response)
    }

}

impl LcuClient {
    fn index_range_query(start: u32, end: u32) -> String {
        format!("?begIndex={}&endIndex={}", start, end)
    }

    pub async fn get_team_summoners(&self) -> anyhow::Result<Vec<Summoner>> {
        let slot_ids = vec![0, 1, 2, 3, 4];
        let mut summoners = Vec::with_capacity(5);
        for slot_id in slot_ids {
            let summoner: Summoner = 
                self.get(&format!("/lol-champ-select/v1/summoners/{}", slot_id))
                .await?;
            if summoner.puuid.is_empty() {
                continue;
            }
            summoners.push(summoner);
        }
        Ok(summoners)
    }

    pub async fn get_summoner_match_history(&self, summoner: Summoner, game_count: u32) -> Result<GameHistoryQuery, ModelError> {

        let url = format!("/lol-match-history/v1/products/lol/{}/matches{}", summoner.puuid, Self::index_range_query(0, game_count - 1));
        let history = self.get(&url)
            .await.map_err(|_| ModelError::HistoryNotFound)?;

        Ok(history)
    }

    pub async fn get_summoner_match_histories(&self, game_count: u32) -> anyhow::Result<Vec<GameHistoryQuery>> {
        let summoners = self.get_team_summoners().await?;
        let mut histories = Vec::with_capacity(game_count as usize);
        for summoner in summoners {
            let history = self.get_summoner_match_history(summoner, game_count).await?;
            histories.push(history);
        }
        Ok(histories)
    }
}

fn get_riot_certificate() -> Certificate {
    Certificate::from_pem(include_bytes!("riotgames.pem")).unwrap()
}

lazy_static! {

}