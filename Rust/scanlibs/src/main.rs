extern crate curl;
extern crate regex;
extern crate telegram_bot;
extern crate tokio_core;

use curl::easy::Easy;
use regex::Regex;

use std::env;

use tokio_core::reactor::Core;
use telegram_bot::{Api, GetMe};

fn main2() {
    let mut easy = Easy::new();

    easy.url("https://scanlibs.com/").unwrap();
    easy.write_function(|data| {
    	let body = match std::str::from_utf8(data) {
	        Ok(v)  => v,
	        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
	    };

		let re = Regex::new("(?s)\"alignleft\" src=\"(.+?)\" alt=\"(.+?)\".+?>(.+?)href=\"(.+?)\"").unwrap();
		for cap in re.captures_iter(body) {
		    println!("{} {} {} {}", &cap[1], &cap[2], &cap[3], &cap[4]);
		}

        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();
}

fn main() {
    let token = "";

    let mut core = Core::new().unwrap();

    let api = Api::configure(token).build(core.handle()).unwrap();
    let future = api.send(GetMe);

    println!("{:?}", core.run(future))
}
