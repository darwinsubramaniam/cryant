use std::borrow::Cow;

use conversion_rate::{ConversionFixerResponse, ConversionRate};
use fiat_symbol::{FiatSymbol, FixerSymbolsResponse};
use reqwest::{header::{HeaderMap, HeaderValue}, Error};

pub mod fiat_symbol;
pub mod conversion_rate;

pub struct FixerApi {
    base_url: String,
    client: reqwest::Client,
}

impl FixerApi {
    pub fn new(api_key: Cow<'static, str>) -> Self {
        let headers = {
            let mut headers = HeaderMap::new();
            headers.insert("apikey", HeaderValue::from_str(&api_key).unwrap());
            headers
        };
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build().unwrap();
        Self { base_url: "https://api.apilayer.com/fixer".to_string(), client }
    }

    /**
     * Get the list of currencies supported by the Fixer API.
     * 
     * @returns The list of currencies supported by the Fixer API.
     */
    pub async fn get_list_of_currencies(&self) -> Result<Vec<FiatSymbol>, Error> {
        let url = format!("{}/symbols", self.base_url);
        let response = self.client.get(url).send().await?;
        let json = response.json::<FixerSymbolsResponse>().await?;  
        Ok(json.into())
    }

    /**
     * Get the exchange rate for a given base currency and optional list of symbols.
     * 
     * @param base The base currency.
     * @param symbols An optional list of symbols to get the exchange rate for. If not symbols are provided, the exchange rate for all symbols will be returned against the base currency.    
     * @returns The exchange rate for the given base currency and symbols.
     */
    pub async fn get_exchange_rate(&self, base: &str, symbols: Option<Vec<&str>>) -> Result<Vec<ConversionRate>, Error> {

        let symbols_query = symbols.map(|symbols| symbols.join(",")).unwrap_or("".to_string());

        let url = match symbols_query.is_empty() {
            true => format!("{}/latest?base={}", self.base_url, base),
            false => format!("{}/latest?symbols={}&base={}", self.base_url, symbols_query, base),
        };

        let response = self.client.get(url).send().await?;
        let json = response.json::<ConversionFixerResponse>().await?;
        Ok(json.into())
    }


}


#[cfg(test)]
mod tests {
    use super::*;


    #[tokio::test]
    async fn test_get_list_of_currencies() {
        let api_key = std::env::var("FIXER_API_KEY").expect("FIXER_API_KEY environment variable not set");
        let fixer = FixerApi::new(api_key.into());
        let result = fixer.get_list_of_currencies().await;
        assert!(result.is_ok());
        let symbols = result.unwrap();
        assert!(symbols.len() > 0);
        assert!(symbols.contains(&FiatSymbol { symbol: "USD".to_string(), name: "United States Dollar".to_string() }));
        assert!(symbols.contains(&FiatSymbol { symbol: "EUR".to_string(), name: "Euro".to_string() }));
    }

    #[tokio::test]
    async fn test_get_exchange_rate() {
        let api_key = std::env::var("FIXER_API_KEY").expect("FIXER_API_KEY environment variable not set");
        let fixer = FixerApi::new(api_key.into());
        let result = fixer.get_exchange_rate("EUR", Some(vec!["USD", "GBP"])).await;
        assert!(result.is_ok());
        let conversion_rates = result.unwrap();
        assert!(conversion_rates.len() > 0);
        println!("{:?}", conversion_rates);
    }
}