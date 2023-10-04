use anyhow::Result;
use csv::WriterBuilder;
use std::fs::File;

use amm_orderbook::{kraken::TradeJsonData, uniswap_v2::Movement};

fn main() -> Result<()> {
    // read from file json
    // already created
    let file = File::open("trade_info/uni_movement_high_liquidity.json")?;
    let json = serde_json::from_reader(file)?;
    let movements_1: Vec<Movement> = json;

    create_csv_uniswap("uni_movement_high_liquidity", movements_1)?;

    let file = File::open("trade_info/uni_movement_low_liquidity.json")?;
    let json = serde_json::from_reader(file)?;
    let movements_2: Vec<Movement> = json;

    create_csv_uniswap("uni_movement_low_liquidity", movements_2)?;

    let file = File::open("trade_info/order_book_data.json")?;
    let json = serde_json::from_reader(file)?;
    let prices: Vec<TradeJsonData> = json;

    let prices = prices
        .iter()
        .map(|price| price.price.parse::<f64>().unwrap())
        .collect::<Vec<f64>>();

    create_orderbook_prices_csv(prices)?;

    Ok(())
}

fn create_csv_uniswap(filename: &str, data: Vec<Movement>) -> Result<()> {
    let file = File::create(format!("csv/{}.csv", filename))?;

    // Create a CSV writer
    let mut writer = WriterBuilder::new().from_writer(file);

    // Write the CSV header
    writer.write_record(&["x", "y"])?;

    // Write the Movement data to the CSV file
    for movement in data.iter() {
        writer.serialize(movement)?;
    }

    // Flush the writer to ensure data is written to the file
    writer.flush()?;

    Ok(())
}

fn create_orderbook_prices_csv(prices: Vec<f64>) -> Result<()> {
    let file = File::create("csv/orderbook_prices.csv")?;

    // Create a CSV writer
    let mut writer = WriterBuilder::new().from_writer(file);

    // Write the CSV header
    writer.write_record(&["price"])?;

    // Write the Movement data to the CSV file
    for price in prices.iter() {
        writer.serialize(price)?;
    }

    // Flush the writer to ensure data is written to the file
    writer.flush()?;

    Ok(())
}
