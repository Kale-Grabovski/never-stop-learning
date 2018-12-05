extern crate postgres;

mod binance_api;
mod rates;

use std::env;
use postgres::{Connection, TlsMode};

fn main() {
    let key = env::var("BINANCE_KEY").expect("BINANCE_KEY env is not set");
    let secret = env::var("BINANCE_SECRET").expect("BINANCE_SECRET env is not set");
    let binance = binance_api::Binance::new(key, secret);

    let balances = binance.get_balances();
    for b in balances {
        println!("{:?}", b);
    }

    let dsn = env::var("PGDSN").expect("PGDSN env is not set");
    let conn = Connection::connect(dsn, TlsMode::None).expect("Driver connection error");
    let rates = rates::Rates::new(&conn);

    let tickers = binance.get_tickers();
    for b in tickers.into_iter() {
        let pair = rates.get_save_pair(&b.symbol);
        println!("{:?}", pair);
    }
}
