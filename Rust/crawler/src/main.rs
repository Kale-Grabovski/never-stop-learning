extern crate curl;
extern crate regex;

use curl::easy::Easy;
use regex::Regex;

fn main() {
    let mut easy = Easy::new();
    let mut body: &str;

    easy.url("https://scanlibs.com/").unwrap();
    easy.write_function(|data| {
    	body = match std::str::from_utf8(data) {
	        Ok(v)  => v,
	        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
	    };

        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();

	let re = Regex::new("<img class=\"alignleft\" src=\"(.+?)\"").unwrap();
	for cap in re.captures_iter(body) {
	    println!("{}", &cap[1]);
	}
}
