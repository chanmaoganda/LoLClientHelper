use reqwest::{Certificate, Client, ClientBuilder};
use serde::Deserialize;


use crate::web_socket::{get_authorize_info, AuthorizeInfo, LcuError};

#[derive(Debug)]
pub struct LcuClient {
    client: Client,
    auth_info: AuthorizeInfo,
}

fn get_riot_certificate() -> Certificate {
    Certificate::from_pem(include_bytes!("riotgames.pem")).unwrap()
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

    pub async fn send<T>(&self, endpoint: &str) -> Result<T, reqwest::Error>
        where T : for<'de> Deserialize<'de>, {
        let url = format!("https://127.0.0.1:{}{}", self.auth_info.port, endpoint);
        println!("Sending request to {}", url);
        let a = self.client.get(url.clone())
           .header(reqwest::header::ACCEPT, "application/json")
           .header("Authorization", &self.auth_info.auth_token)
           .send().await?
           .text().await?;
        println!("Received response: {}", a);

        let response = self.client.get(url.clone())
           .header("Authorization", &self.auth_info.auth_token)
           .send().await?
           .json::<T>().await?;
        println!("Received response");
        Ok(response)
    }
}