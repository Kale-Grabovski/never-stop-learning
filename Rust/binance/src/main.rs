use std::env;

mod binance_api;

fn main() {
    let key = match env::var("BINANCE_KEY") {
        Ok(val) => val,
        Err(_) => panic!("BINANCE_KEY env is not set"),
    };
    let secret = match env::var("BINANCE_SECRET") {
        Ok(val) => val,
        Err(_) => panic!("BINANCE_SECRET env is not set"),
    };
    let binance = binance_api::Binance::new(key, secret);

    let balances = binance.get_balances();
    for b in balances {
        println!("{:?}", b);
    }

    let tickers = binance.get_tickers();
    for b in tickers.into_iter() {
        println!("{:?}", b);
    }
}
