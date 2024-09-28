use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Coin {
    pub id: String,
    pub symbol: String,
    pub name: String,
}