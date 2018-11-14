extern crate reqwest;
extern crate hmac;
extern crate sha2;
extern crate hex;
extern crate serde;
extern crate serde_derive;

use serde_derive::{Serialize, Deserialize};
use sha2::Sha256;
use hmac::{Hmac, Mac};
use std::time::{SystemTime, UNIX_EPOCH};
use reqwest::header;
use std::env;

#[derive(Serialize, Deserialize)]
struct BalanceRaw {
    asset: String,
    free: String,
    locked: String,
}
#[derive(Serialize, Deserialize)]
struct BalancesRaw {
    balances: Vec<BalanceRaw>,
}
#[derive(Debug)]
struct Balance{
    asset: String,
    free: f32,
    locked: f32,
}

#[derive(Serialize, Deserialize)]
struct TickerRaw{
    symbol: String,
    price: String,
}
#[derive(Debug)]
struct Ticker{
    symbol: String,
    price: f32,
}

type HmacSha256 = Hmac<Sha256>;

fn format_request(path: &str, query: &str) -> String {
    let secret = match env::var("BINANCE_SECRET") {
        Ok(val) => val,
        Err(_) => panic!("BINANCE_SECRET env is not set"),
    };

    let mut mac = HmacSha256::new_varkey(secret.as_bytes()).expect("HMAC error");
    let start = SystemTime::now();
    let ts = start.duration_since(UNIX_EPOCH).expect("Time error");
    let ts = ts.as_secs() * 1000 + ts.subsec_nanos() as u64 / 1_000_000;

    let input = query.to_owned() + "&timestamp=" + &ts.to_string().to_owned();

    mac.input(input.as_bytes());

    let result = mac.result();
    let code_bytes = result.code();

    path.to_string()
        + &"?".to_string()
        + &input
        + &"&signature=".to_string()
        + &hex::encode(code_bytes)
}

fn get_response(path: &str, query: &str, sign_request: bool) -> String {
    let uri = "https://api.binance.com/api/v3".to_string();
    let mut headers = header::HeaderMap::new();
    let key = match env::var("BINANCE_KEY") {
        Ok(val) => val,
        Err(_) => panic!("BINANCE_KEY env is not set"),
    };
    headers.insert("X-MBX-APIKEY", header::HeaderValue::from_str(&key.to_owned()).unwrap());

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    let url = if sign_request {uri + &format_request(path, query)} else {uri + &path};
    client.get(&url)
        .send()
        .unwrap()
        .text()
        .unwrap()
}

fn get_balances() -> Vec<Balance> {
    let body = get_response("/account", "", true);
    let res: BalancesRaw = serde_json::from_str(&body).unwrap();

    let mut balances: Vec<Balance> = vec![];
    for d in res.balances {
        let free = d.free.parse::<f32>().unwrap();
        let locked = d.locked.parse::<f32>().unwrap();

        if free > 0.0 || locked > 0.0 {
            balances.push(Balance{
                asset: d.asset,
                free: free,
                locked: locked,
            });
        }
    }

    balances
}

fn get_tickers() -> Vec<Ticker> {
    let body = get_response("/ticker/price", "", false);
    let res: Vec<TickerRaw> = serde_json::from_str(&body).unwrap();

    let mut tickers: Vec<Ticker> = vec![];
    for d in res {
        tickers.push(Ticker{
            symbol: d.symbol,
            price: d.price.parse::<f32>().unwrap(),
        });
    }

    tickers
}

fn main() {
    let balances = get_balances();
    for b in balances {
        println!("{:?}", b);
    }

    let tickers = get_tickers();
    for b in tickers.into_iter() {
        println!("{:?}", b);
    }
}
