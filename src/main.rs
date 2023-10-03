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

    let (trades, start_price, end_price) = client.get_most_recent_trades("ethusd").await?;

    let mut num_of_buys: u32 = 0;
    let mut num_of_sells: u32 = 0;
    let mut buy_amount: U256 = U256::zero();
    let mut sell_amount: U256 = U256::zero();

    for trade in trades.iter() {
        match trade {
            kraken::TradeType::Buy(trade) => {
                num_of_buys += 1;
                buy_amount += trade.amount;
            }
            kraken::TradeType::Sell(trade) => {
                num_of_sells += 1;
                sell_amount += trade.amount;
            }
        }
    }

    // let liquidity: U256 = parse_ether(trades
    //     .iter()
    //     .map(|trade| trade.amount)
    //     .sum::<f64>())?;

    let liquidity: U256 =
        U256::from(478057209076417332255322960494178308u128) * U256::from(100_000_000_000_000u64);

    let new_price = uniswap_v2::simulate_trades(liquidity, start_price, trades, 3)?;

    println!("start price: {}", format_ether(start_price));
    println!("end price: {}", format_ether(end_price));
    println!("num of buys: {}", num_of_buys);
    println!("num of sells: {}", num_of_sells);
    println!("buy amount: {}", format_ether(buy_amount));
    println!("sell amount: {}", format_ether(sell_amount));
    println!("univ2 liquidity pools: {:?}", new_price);
    println!("univ2 liquidity pools: {:?}", new_price.0 * new_price.1);
    println!("univ2 new price: {}", new_price.0 / new_price.1);

    Ok(())
}
