pub const API_BASE_URL: &str = "https://api.coingecko.com";
pub const API_VERSION: u64 = 3;

pub struct ApiEndpoints<'a> {
    pub ping: &'a str,
    pub list_coins: &'a str,
    pub list_coins_markets: &'a str,
    pub supported_vs_currencies: &'a str,
    pub list_exchanges: &'a str,
}

impl<'a> ApiEndpoints<'a> {
    pub fn new() -> Self {
        Self {
            ping: "/ping",
            list_coins: "/coins/list",
            list_coins_markets: "/coins/markets",
            list_exchanges: "/exchanges",
            supported_vs_currencies: "/simple/supported_vs_currencies",
        }
    }
}
