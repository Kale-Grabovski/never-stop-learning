extern crate postgres;

mod binance_api;
mod rates;

use std::env;
use postgres::{Connection, TlsMode};
use std::{thread, time};

fn main() {
    let key = env::var("BINANCE_KEY").expect("BINANCE_KEY env is not set");
    let secret = env::var("BINANCE_SECRET").expect("BINANCE_SECRET env is not set");
    let binance = binance_api::Binance::new(key, secret);

    let dsn = env::var("PGDSN").expect("PGDSN env is not set");
    let conn = Connection::connect(dsn, TlsMode::None).expect("Driver connection error");
    let rates = rates::Rates::new(&conn);
    let dur = time::Duration::from_secs(30);

    loop {
        binance.get_tickers()
            .iter()
            .for_each(|b| {
                let pair = rates.get_save_pair(&b.symbol).unwrap();
                rates.save_rate(&pair, b.price).expect("Unable to save rate");
                println!("{} pair saved with rate {}", pair.symbol, b.price);
            });
        thread::sleep(dur);
    }
}
