use anyhow::Result;
use ethers::{utils::parse_ether, types::U256};
use serde::{Deserialize, Serialize};

pub struct KrakenClient {
    pub client: reqwest::Client,
    pub api_key: String,
}

impl KrakenClient {
    pub async fn new(api_key: String) -> Self {
        let client = reqwest::Client::builder()
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    "X-CW-API-Key",
                    reqwest::header::HeaderValue::from_str(&api_key).unwrap(),
                );
                headers
            })
            .build()
            .unwrap();
        Self { client, api_key }
    }

    pub async fn get_orderbook_data(
        &self,
        pair: &str,
    ) -> Result<serde_json::Value> {
        let url = format!(
            "https://api.kraken.com/0/public/Trades?pair={}",
            pair
        );
        let value = self
            .client
            .get(&url)
            .send()
            .await
            .unwrap()
            .json::<serde_json::Value>()
            .await?;
        Ok(value)
    }

    pub async fn get_most_recent_trades(
        &self,
        pair: &str,
    ) -> Result<(Vec<TradeType>, U256, U256, Vec<TradeJsonData>)> {
        let url = format!(
            "https://api.kraken.com/0/public/Trades?pair={}&limit=1000",
            pair
        );
        let res = self
            .client
            .get(&url)
            .send()
            .await
            .unwrap()
            .json::<MostRecentTradesRes>()
            .await?;

        let start_price = parse_ether(&res.result.xethzusd[0].price)?;
        let end_price = parse_ether(&res.result.xethzusd[res.result.xethzusd.len() - 1].price)?;

        let mut trades = Vec::new();
        let mut json_data = Vec::new();

        for raw_trade in res.result.xethzusd {
            let trade = Trade {
                price: parse_ether(&raw_trade.price)?,
                amount: parse_ether(&raw_trade.volume)?,
            };
            if raw_trade.buy_sell == "b" {
                trades.push(TradeType::Buy(trade));
            } else {
                trades.push(TradeType::Sell(trade));
            }

            let json_info = TradeJsonData {
                price: raw_trade.price,
                amount: raw_trade.volume,
                buy_sell: raw_trade.buy_sell,
            };

            json_data.push(json_info);
        }

        Ok((trades, start_price, end_price, json_data))
    }
}

#[derive(Debug, Deserialize)]
pub struct MostRecentTradesRes {
    result: PairInfo,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct PairInfo {
    xethzusd: Vec<TickData>,
}

#[derive(Debug, Deserialize)]
pub struct TickData {
    pub price: String,
    pub volume: String,
    pub time: f64,
    pub buy_sell: String,
    pub market_limit: String,
    pub miscellaneous: String,
    pub trade_id: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct Trade {
    pub price: U256,
    pub amount: U256,
}

#[derive(Debug, Clone, Copy)]
pub enum TradeType {
    Buy(Trade),
    Sell(Trade)
}

#[derive(Debug, Serialize)]
pub struct TradeJsonData {
    pub price: String,
    pub amount: String,
    pub buy_sell: String,
}