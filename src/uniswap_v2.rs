use crate::kraken::TradeType;
use anyhow::Result;
use ethers::{types::U256, utils::format_ether};
use serde::Serialize;

#[derive(Debug)]
pub struct TradeWei {
    pub is_buy: bool,
    pub amount: U256,
}

pub fn simulate_trades(
    liquidity: U256,
    start_price: U256,
    trades: Vec<TradeType>,
    fees: u32,
) -> Result<(U256, U256, Vec<Movement>)> {
    // liquidity is our k
    // so main liquidity is k / intial eth price
    // x = eth reserve
    let mut movement = Vec::new();


    let y = liquidity / U256::from((format_ether(start_price).parse::<f64>()? * 10_000f64) as u128)
        * U256::from(10_000u64);
    let y = y.integer_sqrt();

    let x = liquidity / y;

    let mut liquidity = (x, y);

    movement.push( Movement { x: liquidity.0.as_u128(), y: liquidity.1.as_u128() });

    for trade in trades {
        liquidity = simulate_trade(liquidity, trade, fees);
        movement.push(Movement {
            x: liquidity.0.as_u128(),
            y: liquidity.1.as_u128(),
        });
    }

    Ok((liquidity.0, liquidity.1, movement))
}

pub fn simulate_trade(liquidity: (U256, U256), trade: TradeType, fees: u32) -> (U256, U256) {
    // equation is xy=k
    // we want to do (x-dx)(y+dy*fee)=k

    // so we have x = liquidity.0
    // y = liquidity.1
    // dx = trade.amount
    // dy = trade.amount * trade.price

    // x is eth
    // y is dollar

    // buy is +eth -dollar
    // sell is -eth +dollar
    match trade {
        TradeType::Sell(trade) => {
            let x = liquidity.0;
            let y = liquidity.1;

            let k = x * y;

            let dx = trade.amount;

            let dy = (k / (x - dx)) - y;
            let dy = dy - (dy * U256::from(fees) / 10_00);

            let new_x = x - dx;
            let new_y = y + dy;

            (new_x, new_y)
        }
        TradeType::Buy(trade) => {
            // flipped because we are buying eth
            // so pool y is usd, that needs to be lowered
            // unflipped later

            let x = liquidity.1;
            let y = liquidity.0;

            let k = x * y;

            let dx = trade.amount;

            let dy = (k / (x - dx)) - y;
            let dy = dy - (dy * U256::from(fees) / 10_00);

            let new_x = x - dx;
            let new_y = y + dy;

            (new_y, new_x)
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Movement {
    pub x: u128,
    pub y: u128,
}