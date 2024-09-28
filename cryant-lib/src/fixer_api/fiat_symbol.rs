use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct FixerSymbolsResponse {
    pub success: bool,
    pub symbols: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FiatSymbol {
    pub symbol: String,
    pub name: String,
}


impl From<FixerSymbolsResponse> for Vec<FiatSymbol> {   
    fn from(response: FixerSymbolsResponse) -> Self {
        response.symbols.into_iter().map(|(symbol, name)| FiatSymbol { symbol, name }).collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        let response = FixerSymbolsResponse {
            success: true,
            symbols: HashMap::from([("USD".to_string(), "United States Dollar".to_string()), ("EUR".to_string(), "Euro".to_string())]),
        };
        let symbols = Vec::from(response);
        assert_eq!(symbols.len(), 2);
        // order is not guaranteed so we need to check for the presence of each symbol
        assert!(symbols.contains(&FiatSymbol { symbol: "USD".to_string(), name: "United States Dollar".to_string() }));
        assert!(symbols.contains(&FiatSymbol { symbol: "EUR".to_string(), name: "Euro".to_string() }));
    }
}

