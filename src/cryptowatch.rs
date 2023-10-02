use anyhow::Result;
use serde::Deserialize;

pub struct CryptoWatchClient {
    pub client: reqwest::Client,
    pub api_key: String,
}

impl CryptoWatchClient {
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
        exchange: &str,
        pair: &str,
    ) -> Result<serde_json::Value> {
        let url = format!(
            "https://api.cryptowat.ch/markets/{}/{}/orderbook",
            exchange, pair
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
        exchange: &str,
        pair: &str,
    ) -> Result<(Vec<TradeInfo>, f64, f64)> {
        let url = format!(
            "https://api.cryptowat.ch/markets/{}/{}/trades?limit=1000",
            exchange, pair
        );
        let res = self
            .client
            .get(&url)
            .send()
            .await
            .unwrap()
            .json::<MostRecentTradesRes>()
            .await?;
        let mut trades = Vec::new();
        for trade in res.result {
            let trade = Trade {
                id: trade[0],
                timestamp: trade[1],
                price: trade[2],
                amount: trade[3],
            };
            trades.push(trade);
        }

        let start_price = trades[0].price;
        let end_price = trades[trades.len() - 1].price;

        let mut trades_info = Vec::new();

        for (index, trade) in trades.iter().enumerate() {
            if index == trades.len() - 1 {
                break;
            }

            let next_price = trades[index + 1].price;

            let trade_info = TradeInfo {
                amount: trade.amount,
                // if price is less than next price, then the price went up
                // hence being a buy
                is_buy: trade.price < next_price,
            };

            trades_info.push(trade_info);
        }

        println!("start price: {}", start_price);
        println!("end price: {}", end_price);

        Ok((trades_info, start_price, end_price))
    }
}

#[derive(Debug)]
pub struct TradeInfo {
    pub amount: f64,
    pub is_buy: bool,
}

#[derive(Debug, Deserialize)]
pub struct MostRecentTradesRes {
    pub result: Vec<Vec<f64>>,
}

#[derive(Debug)]
pub struct Trade {
    pub id: f64,
    pub timestamp: f64,
    pub price: f64,
    pub amount: f64,
}
