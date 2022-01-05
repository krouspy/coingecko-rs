use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Exchange {
    pub id: String,
    pub name: String,
    pub year_established: Option<u16>,
    pub country: Option<String>,
    pub description: Option<String>,
    pub url: String,
    pub image: String,
    pub has_trading_incentive: Option<bool>,
    pub trust_score: u16,
    pub trade_volume_24h_btc: f32,
    pub trade_volume_24h_btc_normalized: f32,
}

#[derive(Deserialize, Debug)]
pub struct CoinMarket {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub image: Option<String>,
    pub current_price: Option<f32>,
    pub market_cap: Option<u32>,
    pub market_cap_rank: Option<u32>,
    pub fully_diluted_valuation: Option<u32>,
    pub total_volume: Option<u32>,
    pub high_24h: Option<f32>,
    pub low_24h: Option<f32>,
    pub price_change_24h: Option<f32>,
    pub price_change_percentage_24h: Option<f32>,
    pub market_cap_change_24h: Option<f32>,
    pub market_cap_change_percentage_24h: Option<f32>,
    pub circulating_supply: Option<f32>,
    pub total_supply: Option<f32>,
    pub max_supply: Option<u32>,
    pub ath: Option<f32>,
    pub ath_change_percentage: Option<f32>,
    pub ath_date: Option<String>,
    pub atl: Option<f32>,
    pub atl_change_percentage: Option<f32>,
    pub atl_date: Option<String>,
    pub roi: Option<f32>,
    pub last_updated: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Coin {
    pub id: String,
    pub symbol: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Market {
    pub name: String,
    pub identifier: String,
    pub has_trading_incentive: bool,
}

#[derive(Deserialize, Debug)]
pub struct Converted {
    pub btc: f32,
    pub eth: f32,
    pub usd: f32,
}

#[derive(Deserialize, Debug)]
pub struct Ticker {
    pub base: String,
    pub target: String,
    pub market: Market,
    pub last: f32,
    pub volume: f32,
    pub converted_last: Converted,
    pub converted_volume: Converted,
    pub trust_score: String,
    pub bid_ask_spread_percentage: f32,
    pub timestamp: String,
    pub last_traded_at: String,
    pub last_fetch_at: String,
    pub is_anomaly: bool,
    pub is_stale: bool,
    pub trade_url: core::option::Option<String>,
    pub coin_id: String,
}

#[derive(Deserialize, Debug)]
pub struct CoinTickers {
    pub name: String,
    pub tickers: Vec<Ticker>,
}

#[derive(Deserialize, Debug)]
pub struct Ping {
    pub gecko_says: String,
}
