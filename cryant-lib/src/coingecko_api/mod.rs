pub mod coin;
pub mod coin_price;

use std::borrow::Cow;

use coin::Coin;
use coin_price::{CoinPrice, CoinPriceResponse};
use reqwest::header::{HeaderMap, HeaderValue};

// dynamic result and error handling
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct CoinGeckoApi {
    client: reqwest::Client,
    base_url: String,
}

impl CoinGeckoApi {
    pub fn new(api_key: Cow<'static, str>) -> Self {
        let headers = {
            let mut headers = HeaderMap::new();
            headers.insert(
                "x-cg-demo-api-key",
                HeaderValue::from_str(&api_key).unwrap(),
            );
            headers
        };
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        Self {
            base_url: "https:/api.coingecko.com/api/v3".to_string(),
            client,
        }
    }

    pub async fn get_list_of_supported_coins(&self) -> Result<Vec<Coin>> {
        let url = format!("{}/coins/list", self.base_url);
        let response = self.client.get(url).send().await?;
        let json = response.json::<Vec<Coin>>().await?;
        Ok(json)
    }

    pub async fn get_price_of_coin(&self, coin_id: Vec<&str>, currency_id: Vec<&str>) -> Result<Vec<CoinPrice>> {
        if coin_id.is_empty() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "At least one coin id is required",
            )));
        }
        if currency_id.is_empty() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "At least one fiat currency id is required",
            )));
        }
        let currency_id_str = currency_id.join(",");    
        let coin_id_str = coin_id.join(",");
        let url = format!("{}/simple/price?ids={}&vs_currencies={}", self.base_url, coin_id_str, currency_id_str);
        let response = self.client.get(url).send().await?;
        let json = response.json::<CoinPriceResponse>().await?;
        Ok(json.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_list_of_supported_coins() {
        let api_key = std::env::var("COINGECKO_API_KEY").expect("COINGECKO_API_KEY environment variable not set");
        let coin_gecko = CoinGeckoApi::new(api_key.into());
        let coins = coin_gecko.get_list_of_supported_coins().await;
        assert!(coins.is_ok());
        // check if there is bitcoin in the list
        let binding = coins.unwrap();
        let bitcoin = binding.iter().find(|coin| coin.name == "Bitcoin");
        println!("{:?}", bitcoin);
        assert!(bitcoin.is_some());
    }

    #[tokio::test]
    async fn test_get_price_of_bitcoin_only_usd() {
        let api_key = std::env::var("COINGECKO_API_KEY").expect("COINGECKO_API_KEY environment variable not set");
        let coin_gecko = CoinGeckoApi::new(api_key.into());
        let coins = coin_gecko.get_price_of_coin(vec!["bitcoin"], vec!["usd",]).await;
        assert!(coins.is_ok());
        let binding = coins.unwrap();
        let bitcoin_usd = binding.iter().find(|coin| coin.fiat_symbol == "usd").unwrap().price;
        assert!(bitcoin_usd > 0.0);
    }

    #[tokio::test]
    async fn test_get_price_of_bitcoin_only_eur() {
        let api_key = std::env::var("COINGECKO_API_KEY").expect("COINGECKO_API_KEY environment variable not set");
        let coin_gecko = CoinGeckoApi::new(api_key.into());
        let coins = coin_gecko.get_price_of_coin(vec!["bitcoin"], vec!["eur",]).await;
        assert!(coins.is_ok());
        let binding = coins.unwrap();
        let bitcoin_eur = binding.iter().find(|coin| coin.fiat_symbol == "eur").unwrap().price;
        assert!(bitcoin_eur > 0.0);
    }

    #[tokio::test]
    async fn test_get_price_of_bitcoin_and_ethereum_usd() {
        let api_key = std::env::var("COINGECKO_API_KEY").expect("COINGECKO_API_KEY environment variable not set");
        let coin_gecko = CoinGeckoApi::new(api_key.into());
        let coins = coin_gecko.get_price_of_coin(vec!["bitcoin", "ethereum"], vec!["usd",]).await;
        assert!(coins.is_ok());
        let binding = coins.unwrap();
        let bitcoin_usd = binding.iter().find(|coin| coin.fiat_symbol == "usd" && coin.coin_symbol == "bitcoin").unwrap().price;
        assert!(bitcoin_usd > 0.0);
        let ethereum_usd = binding.iter().find(|coin| coin.fiat_symbol == "usd" && coin.coin_symbol == "ethereum").unwrap().price;
        assert!(ethereum_usd > 0.0);
        println!("bitcoin_usd: {}", bitcoin_usd);
        println!("ethereum_usd: {}", ethereum_usd);
    }

    #[tokio::test]
    async fn test_get_price_of_bitcoin_and_ethereum_eur() {
        let api_key = std::env::var("COINGECKO_API_KEY").expect("COINGECKO_API_KEY environment variable not set");
        let coin_gecko = CoinGeckoApi::new(api_key.into());
        let coins = coin_gecko.get_price_of_coin(vec!["bitcoin", "ethereum"], vec!["eur",]).await;
        assert!(coins.is_ok());
        let binding = coins.unwrap();
        let bitcoin_eur = binding.iter().find(|coin| coin.fiat_symbol == "eur" && coin.coin_symbol == "bitcoin").unwrap().price;
        assert!(bitcoin_eur > 0.0);
        let ethereum_eur = binding.iter().find(|coin| coin.fiat_symbol == "eur" && coin.coin_symbol == "ethereum").unwrap().price;
        assert!(ethereum_eur > 0.0);
        println!("bitcoin_eur: {}", bitcoin_eur);
        println!("ethereum_eur: {}", ethereum_eur);
    }

    #[tokio::test]
    async fn test_get_price_of_bitcoin_and_ethereum_eur_and_usd() {
        let api_key = std::env::var("COINGECKO_API_KEY").expect("COINGECKO_API_KEY environment variable not set");
        let coin_gecko = CoinGeckoApi::new(api_key.into());
        let coins = coin_gecko.get_price_of_coin(vec!["bitcoin", "ethereum"], vec!["eur", "usd"]).await;
        assert!(coins.is_ok());
        let binding = coins.unwrap();
        let bitcoin_eur = binding.iter().find(|coin| coin.fiat_symbol == "eur" && coin.coin_symbol == "bitcoin").unwrap().price;
        assert!(bitcoin_eur > 0.0);
        let bitcoin_usd = binding.iter().find(|coin| coin.fiat_symbol == "usd" && coin.coin_symbol == "bitcoin").unwrap().price;
        assert!(bitcoin_usd > 0.0);
        let ethereum_eur = binding.iter().find(|coin| coin.fiat_symbol == "eur" && coin.coin_symbol == "ethereum").unwrap().price;
        assert!(ethereum_eur > 0.0);
        let ethereum_usd = binding.iter().find(|coin| coin.fiat_symbol == "usd" && coin.coin_symbol == "ethereum").unwrap().price;
        assert!(ethereum_usd > 0.0);
        println!("bitcoin_eur: {}", bitcoin_eur);
        println!("bitcoin_usd: {}", bitcoin_usd);
        println!("ethereum_eur: {}", ethereum_eur);
        println!("ethereum_usd: {}", ethereum_usd);
    }


}