mod constants;
pub mod types;

use reqwest::{Client, ClientBuilder};
use thiserror::Error;
use std::collections::HashMap;
use constants::ApiEndpoints;
use types::{Coin, CoinMarket, Exchange, Ping};

pub struct CoingeckoApi<'a> {
    client: Client,
    url: String,
    endpoints: ApiEndpoints<'a>,
}

impl<'a> CoingeckoApi<'a> {
    pub fn new() -> Self {
        let builder = ClientBuilder::new();
        let client = builder.build().unwrap();

        Self {
            client,
            url: format!("{}/api/v{}", constants::API_BASE_URL, constants::API_VERSION),
            endpoints: ApiEndpoints::new(),
        }
    }

    async fn get_request<T: serde::de::DeserializeOwned>(&self, endpoint: &str, parameters: Option<HashMap<&str, &str>>) -> Result<T, CoingeckoApiError> {
        let url = format!("{}/{}", self.url, endpoint);
        let res = self.client.get(url).query(&parameters).send().await?;
        let text = res.text().await?;
        let result: T = serde_json::from_str::<T>(&text)?;

        Ok(result)
    }

    pub async fn ping(&self) -> Result<Ping, CoingeckoApiError> {
        let endpoint = &self.endpoints.ping;
        let response: Ping = self.get_request(&endpoint, None).await?;

        Ok(response)
    }

    pub async fn list_coins(&self) -> Result<Vec<Coin>, CoingeckoApiError> {
        let endpoint = &self.endpoints.list_coins;
        let response: Vec<Coin> = self.get_request(&endpoint, None).await?;

        Ok(response)
    }

    pub async fn list_coins_markets(&self) -> Result<Vec<CoinMarket>, CoingeckoApiError> {
        let mut map = HashMap::new();
        map.insert("vs_currency", "usd");

        let endpoint = &self.endpoints.list_coins;
        let response: Vec<CoinMarket> = self.get_request(&endpoint, Some(map)).await?;

        Ok(response)
    }

    pub async fn list_exchanges(&self) -> Result<Vec<Exchange>, CoingeckoApiError> {
        let endpoint = &self.endpoints.list_exchanges;
        let response: Vec<Exchange> = self.get_request(&endpoint, None).await?;

        Ok(response)
    }

    pub async fn supported_vs_currencies(&self) -> Result<Vec<String>, CoingeckoApiError> {
        let endpoint = &self.endpoints.supported_vs_currencies;
        let response: Vec<String> = self.get_request(&endpoint, None).await?;

        Ok(response)
    }
}

#[derive(Debug, Error)]
pub enum CoingeckoApiError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
