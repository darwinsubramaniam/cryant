use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CoinPrice {
    pub fiat_symbol: String,
    pub coin_symbol: String,
    pub price: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct CoinPriceResponse {
    #[serde(flatten)]
    pub data: HashMap<String, HashMap<String, f64>>,
}

impl From<CoinPriceResponse> for Vec<CoinPrice> {
    fn from(response: CoinPriceResponse) -> Self {
        let mut coin_prices = Vec::new();
        for (coin_symbol, fiat_prices) in response.data {
            for (fiat_symbol, price) in fiat_prices {
                coin_prices.push(CoinPrice { fiat_symbol: fiat_symbol.to_string(), coin_symbol: coin_symbol.to_string(), price });
            }
        }
        coin_prices
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coin_price_from_response() {
        let data = r#"
        {
          "bitcoin": {
            "usd": 65653,
            "sgd": 84029
          },
          "ethereum": {
            "usd": 2671.36,
            "sgd": 3419.07
          }
        }
        "#;
        let response = CoinPriceResponse { data: serde_json::from_str(data).unwrap() };
        let coin_prices: Vec<CoinPrice> = response.into();
        println!("{:?}", coin_prices);
        assert_eq!(coin_prices.len(), 4);    
    }
}



