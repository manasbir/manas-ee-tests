use crate::cryptowatch::TradeInfo;
use ethers::{types::U256, utils::parse_ether};
use anyhow::Result;

#[derive(Debug)]
pub struct TradeWei {
    pub is_buy: bool,
    pub amount: U256,
}

pub fn simulate_trades(
    liquidity: U256,
    start_price: f64,
    trades: Vec<TradeInfo>,
    fees: u32,
) -> Result<(U256, U256)> {
    // liquidity is our k
    // so main liquidity is k / intial eth price
    // x = eth reserve

    let trades: Vec<TradeWei> = trades
        .iter()
        .map(|trade| TradeWei {
            is_buy: trade.is_buy,
            amount: parse_ether(trade.amount).unwrap(),
        })
        .collect();

    let y: U256 = (liquidity / U256::from((start_price * 10_000f64) as u128) ) * U256::from(10_000u64);
    let y =  y.integer_sqrt();

    let x = liquidity / y;

    dbg!(liquidity);
    dbg!(liquidity, x*y);

    let mut liquidity = (x, y);
    


    for trade in trades {
        liquidity = simulate_trade(liquidity, trade, fees);
    }

    Ok(liquidity)

}

pub fn simulate_trade(liquidity: (U256, U256), trade: TradeWei, fees: u32) -> (U256, U256) {
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

    if trade.is_buy {
        // flipped because we are buying eth
        // so pool y is usd, that needs to be lowered
        // unflipped laer

        let x = liquidity.1;
        let y = liquidity.0;
        println!("x,y: {}, {}", x,y);
        let k = x * y;
        println!("k: {}", k);
        let dx = trade.amount;

        println!("dx: {}", dx);

        dbg!(x-dx);

        let dy = (k / (x - dx)) - y;
        let dy = dy - (dy * U256::from(fees) / 10_00);

        let new_x = x - dx;
        let new_y = y + dy;
        (new_y, new_x)
    } else {
        let x = liquidity.0;
        let y = liquidity.1;
        println!("sell x,y: {}, {}", x,y);

        let k = x * y;
        println!("sell k: {}", k);

        println!("sell dx: {}", trade.amount);
        println!("sell dy: {}", (k / (x - trade.amount)) - y);

        let dx = trade.amount;

        let dy = (k / (x - dx)) - y;
        let dy = dy - (dy * U256::from(fees) / 10_00);

        let new_x = x - dx;
        let new_y = y + dy;
        (new_x, new_y)
    }
}
