use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CoinData {
    pub id: String,
    pub name: String,
    pub symbol: String,
    pub market_data: MarketData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketData {
    pub current_price: Prices,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Prices {
    pub usd: f64,
    pub mxn: f64,
}
