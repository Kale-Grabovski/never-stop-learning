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

pub struct Binance{
    api_key: String,
    secret_key: String,
}

#[derive(Debug)]
pub struct Ticker{
    pub symbol: String,
    pub price: f32,
}
#[derive(Debug)]
pub struct Balance{
    asset: String,
    free: f32,
    locked: f32,
}

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
#[derive(Serialize, Deserialize)]
struct TickerRaw{
    symbol: String,
    price: String,
}

type HmacSha256 = Hmac<Sha256>;

impl Binance {
    pub fn new(api_key: String, secret_key: String) -> Binance {
        Binance{api_key, secret_key}
    }

    pub fn get_balances(&self) -> Vec<Balance> {
        let body = self.get_response("/account", "", true);
        let res: BalancesRaw = serde_json::from_str(&body).unwrap();

        res.balances
            .iter()
            .filter_map(|x| {
                let free = x.free.parse::<f32>().unwrap();
                let locked = x.locked.parse::<f32>().unwrap();

                if free > 0.0 || locked > 0.0 {
                    return Some(Balance{asset: x.asset.clone(), free, locked});
                }

                None
            })
            .collect::<Vec<Balance>>()
    }

    pub fn get_tickers(&self) -> Vec<Ticker> {
        let body = self.get_response("/ticker/price", "", false);
        let res: Vec<TickerRaw> = serde_json::from_str(&body).unwrap();

        res
            .iter()
            .map(|x| Ticker{
                symbol: x.symbol.clone(),
                price: x.price.parse::<f32>().unwrap(),
            })
            .collect::<Vec<Ticker>>()
    }

    fn format_request(&self, path: &str, query: &str) -> String {
        let mut mac = HmacSha256::new_varkey(self.secret_key.as_bytes()).expect("HMAC error");
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

    fn get_response(&self, path: &str, query: &str, sign_request: bool) -> String {
        let uri = "https://api.binance.com/api/v3".to_string();
        let mut headers = header::HeaderMap::new();
        headers.insert("X-MBX-APIKEY", header::HeaderValue::from_str(&self.api_key.to_owned()).unwrap());

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        let url = if sign_request {uri + &self.format_request(path, query)} else {uri + &path};
        client.get(&url)
            .send()
            .unwrap()
            .text()
            .unwrap()
    }
}
