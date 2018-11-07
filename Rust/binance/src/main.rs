extern crate reqwest;
extern crate hmac;
extern crate sha2;
extern crate hex;

use sha2::Sha256;
use hmac::{Hmac, Mac};

type HmacSha256 = Hmac<Sha256>;

fn main() {
    //let uri = "https://api.binance.com/api/v3";
    // /ticker/price

    let body = reqwest::get("https://www.rust-lang.org")
        .unwrap()
        .text()
        .unwrap();

    // Create HMAC-SHA256 instance which implements `Mac` trait
    let mut mac = HmacSha256::new_varkey(b"NhqPtmdSJYdKjVHjA7PZj4Mge3R5YNiP1e3UZjInClVN65XAbvqqM6A7H5fATj0j")
        .expect("HMAC can take key of any size");
    mac.input(b"symbol=LTCBTC&side=BUY&type=LIMIT&timeInForce=GTC&quantity=1&price=0.1&recvWindow=5000&timestamp=1499827319559");

    // `result` has type `MacResult` which is a thin wrapper around array of
    // bytes for providing constant time equality check
    let result = mac.result();
    // To get underlying array use `code` method, but be carefull, since
    // incorrect use of the code value may permit timing attacks which defeat
    // the security provided by the `MacResult`
    let code_bytes = result.code();
    let hash = String::from_utf8_lossy(&code_bytes);

    println!("body = {:?}, {:?}", body, hex::encode(code_bytes));
}
