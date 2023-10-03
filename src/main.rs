use anyhow::Result;
use ethers::{types::U256, utils::format_ether};
use kraken::KrakenClient;

pub mod kraken;
pub mod uniswap_v2;

#[tokio::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_BACKTRACE", "1");

    dotenv::dotenv().ok();
    let api_key = dotenv::var("CRYPTOWATCH_API_KEY").unwrap();

    let client = KrakenClient::new(api_key).await;

    let (trades, start_price, end_price, trade_data) = client.get_most_recent_trades("ethusd").await?;

    let mut num_of_buys: u32 = 0;
    let mut num_of_sells: u32 = 0;
    let mut buy_amount: U256 = U256::zero();
    let mut sell_amount: U256 = U256::zero();

    let mut highest_buy: U256 = U256::zero();
    let mut highest_sell: U256 = U256::zero();
    let mut lowest_buy: U256 = U256::MAX;
    let mut lowest_sell: U256 = U256::MAX;

    for trade in trades.iter() {
        match trade {
            kraken::TradeType::Buy(trade) => {
                num_of_buys += 1;
                buy_amount += trade.amount;
                if trade.price > highest_buy {
                    highest_buy = trade.price;
                }
                if trade.price < lowest_buy {
                    lowest_buy = trade.price;
                }
            }
            kraken::TradeType::Sell(trade) => {
                num_of_sells += 1;
                sell_amount += trade.amount;
                if trade.price > highest_sell {
                    highest_sell = trade.price;
                }
                if trade.price < lowest_sell {
                    lowest_sell = trade.price;
                }
            }
        }
    }

    // let liquidity: U256 = parse_ether(trades
    //     .iter()
    //     .map(|trade| trade.amount)
    //     .sum::<f64>())?;

    let liquidity: U256 =
        U256::from(478057209076417332255322960494178308u128) * U256::from(1_000_000_000_000_000u64);

    let new_price_0: (U256, U256, Vec<uniswap_v2::Movement>) = uniswap_v2::simulate_trades(liquidity, start_price, trades.clone(), 3)?;

    let file = std::fs::File::create("trade_info/order_book_data.json")?;
    let data = serde_json::to_string_pretty(&trade_data)?;
    std::io::Write::write_all(&mut std::io::BufWriter::new(file), data.as_bytes())?;
    let file = std::fs::File::create("trade_info/uni_movement_high_liquidity.json")?;
    let data = serde_json::to_string_pretty(&new_price_0.2)?;
    std::io::Write::write_all(&mut std::io::BufWriter::new(file), data.as_bytes())?;

    let liquidity: U256 =
        U256::from(478057209076417332255322960494178308u128) * U256::from(1_000_000_000_000u64);

    let new_price_1: (U256, U256, Vec<uniswap_v2::Movement>) = uniswap_v2::simulate_trades(liquidity, start_price, trades, 3)?;

    let file = std::fs::File::create("trade_info/uni_movement_low_liquidity.json")?;
    let data = serde_json::to_string_pretty(&new_price_1.2)?;
    std::io::Write::write_all(&mut std::io::BufWriter::new(file), data.as_bytes())?;

    println!("start price: {}", format_ether(start_price));
    println!("end price: {}", format_ether(end_price));
    println!("num of buys: {}", num_of_buys);
    println!("num of sells: {}", num_of_sells);
    println!("buy amount: {}", format_ether(buy_amount));
    println!("sell amount: {}", format_ether(sell_amount));
    println!("univ2 liquidity pools: {:?}", (new_price_0.0, new_price_0.1));
    println!("univ2 liquidity pools: {:?}", new_price_0.0 * new_price_0.1);
    println!("univ2 new price: {}", new_price_0.0.as_u128() as f64 / new_price_0.1.as_u128() as f64);
    println!("univ2 liquidity pools: {:?}", (new_price_1.0, new_price_1.1));
    println!("univ2 liquidity pools: {:?}", new_price_1.0 * new_price_1.1);
    println!("univ2 new price: {}", new_price_1.0.as_u128() as f64 / new_price_1.1.as_u128() as f64);
    println!("highest buy: {}", format_ether(highest_buy));
    println!("highest sell: {}", format_ether(highest_sell));
    println!("lowest buy: {}", format_ether(lowest_buy));
    println!("lowest sell: {}", format_ether(lowest_sell));

    Ok(())
}
