extern crate reqwest;
extern crate hmac;
extern crate sha2;
extern crate hex;
extern crate serde;

#[macro_use]
extern crate serde_derive;

use sha2::Sha256;
use hmac::{Hmac, Mac};
use std::time::{SystemTime, UNIX_EPOCH};
use reqwest::header;

#[derive(Serialize, Deserialize, Debug)]
struct Balance {
    asset: String,
    free: String,
    locked: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Balances {
    balances: Vec<Balance>,
}

type HmacSha256 = Hmac<Sha256>;

fn format_request(path: &str, query: &str) -> String {
    let uri = "https://api.binance.com/api/v3".to_string();
    let secret = "bM04FLpBLyftHUXajCGDs8i3yqGcUyBLkv1PtxK6W8ZzZw3OXpIv9ACWtOsgy5Bs";
    let mut mac = HmacSha256::new_varkey(secret.as_bytes()).expect("HMAC error");
    let start = SystemTime::now();
    let ts = start.duration_since(UNIX_EPOCH).expect("Time error");
    let ts = ts.as_secs() * 1000 + ts.subsec_nanos() as u64 / 1_000_000;

    let input = query.to_owned() + "&timestamp=" + &ts.to_string().to_owned();

    mac.input(input.as_bytes());

    let result = mac.result();
    let code_bytes = result.code();

    uri.to_string()
        + path
        + &"?".to_string()
        + &input
        + &"&signature=".to_string()
        + &hex::encode(code_bytes)
}

fn get_response(path: &str, query: &str) -> String {
    let mut headers = header::HeaderMap::new();
    headers.insert("X-MBX-APIKEY", header::HeaderValue::from_str("VQlJwQ5dr3BtXHcmE9ZPWNwJVibyuRnlc39xDHnSmegFHgvLTtziLgBzTApdGSr9").unwrap());

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    let url = format_request(path, query);
    client.get(&url)
        .send()
        .unwrap()
        .text()
        .unwrap()
}

fn get_balances<'a>() -> Vec<&'a Balance> {
    let mut balances: Vec<&Balance> = vec![];

    let body = get_response("/account", "");

    let res: Balances = serde_json::from_str(&body).unwrap();
    for d in res.balances.iter() {
        balances.push(d);
    }

    balances
}

fn main() {
    let balances = get_balances();
    println!("{:?}", balances);
}
