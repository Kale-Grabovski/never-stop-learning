extern crate reqwest;
extern crate hmac;
extern crate sha2;
extern crate hex;

use sha2::Sha256;
use hmac::{Hmac, Mac};
use std::time::{SystemTime, UNIX_EPOCH};
use reqwest::header;

type HmacSha256 = Hmac<Sha256>;

fn format_request(query: &str) -> String {
    let secret = "bM04FLpBLyftHUXajCGDs8i3yqGcUyBLkv1Ptx";
    let mut mac = HmacSha256::new_varkey(secret.as_bytes()).expect("HMAC error");
    let start = SystemTime::now();
    let ts = start.duration_since(UNIX_EPOCH).expect("Time error");
    let ts = ts.as_secs() * 1000 + ts.subsec_nanos() as u64 / 1_000_000;

    let input = query.to_owned() + "timestamp=" + &ts.to_string().to_owned();
    mac.input(input.as_bytes());

    let result = mac.result();
    let code_bytes = result.code();

    return input + &"&signature=".to_string() + &hex::encode(code_bytes);
}

fn main() {
    let uri = "https://api.binance.com/api/v3".to_string();
    let mut headers = header::HeaderMap::new();
    headers.insert("X-MBX-APIKEY", header::HeaderValue::from_str("VQlJwQ5dr3BtXHcmE9ZPWNwJVibyuRnlc3").unwrap());

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();
    let url = uri + "/account?" + &format_request("");
    println!("body = {:?}", url);
    let body = client.get(&url)
        .send()
        .unwrap()
        .text()
        .unwrap();

    println!("body = {:?}", body);
}
