use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ConversionRate {
    pub base: String,
    pub target: String,
    pub rate: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ConversionFixerResponse {
    pub success: bool,
    pub timestamp: u64,
    pub base: String,
    pub date: String,
    pub rates: HashMap<String, f64>,
}

impl From<ConversionFixerResponse> for Vec<ConversionRate> {
    fn from(response: ConversionFixerResponse) -> Vec<ConversionRate> {
        let rates = response.rates.into_iter().map(|(k, v)| ConversionRate {
            base: response.base.clone(),
            target: k,
            rate: v,
        }).collect();
        return rates;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion_rate_from_fixer_response() {
        let response = ConversionFixerResponse {
            success: true,
            timestamp: 1714857600,
            base: "EUR".to_string(),
            date: "2024-05-08".to_string(),
            rates: HashMap::from([("USD".to_string(), 1.10), ("GBP".to_string(), 0.85)]),
        };

        let conversion_rates = Vec::<ConversionRate>::from(response);
        assert_eq!(conversion_rates.len(), 2);
        assert!(conversion_rates.contains(&ConversionRate { base: "EUR".to_string(), target: "USD".to_string(), rate: 1.10 }));
        assert!(conversion_rates.contains(&ConversionRate { base: "EUR".to_string(), target: "GBP".to_string(), rate: 0.85 }));
    }
}

