use anyhow::Result;
use cryptowatch::CryptoWatchClient;
use ethers::types::U256;

pub mod cryptowatch;
pub mod uniswap_v2;

#[tokio::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_BACKTRACE", "1");

    dotenv::dotenv().ok();
    let api_key = dotenv::var("CRYPTOWATCH_API_KEY").unwrap();

    let client = CryptoWatchClient::new(api_key).await;

    let (trades, start_price, end_price) = client
        .get_most_recent_trades("binance", "ethusdt")
        .await?;

    let num_of_buys = trades.iter().filter(|trade| trade.is_buy).count();
    let num_of_sells = trades.iter().filter(|trade| !trade.is_buy).count();
    let buy_amount = trades
        .iter()
        .filter(|trade| trade.is_buy)
        .map(|trade| trade.amount)
        .sum::<f64>();

    let sell_amount = trades
        .iter()
        .filter(|trade| !trade.is_buy)
        .map(|trade| trade.amount)
        .sum::<f64>();

    // let liquidity: U256 = parse_ether(trades
    //     .iter()
    //     .map(|trade| trade.amount)
    //     .sum::<f64>())?;

    let liquidity: U256 = U256::from(478057209076417332255322960494178308u128) * U256::from(100_000_000_000u64);

    let new_price = uniswap_v2::simulate_trades(liquidity, start_price, trades, 3)?;


    println!("start price: {}", start_price);
    println!("end price: {}", end_price);
    println!("num of buys: {}", num_of_buys);
    println!("num of sells: {}", num_of_sells);
    println!("buy amount: {}", buy_amount);
    println!("sell amount: {}", sell_amount);
    println!("univ2 liquidity pools: {:?}", new_price);
    println!("univ2 liquidity pools: {:?}", new_price.0 * new_price.1);
    println!("univ2 new price: {}", new_price.0 / new_price.1);

    Ok(())
}
