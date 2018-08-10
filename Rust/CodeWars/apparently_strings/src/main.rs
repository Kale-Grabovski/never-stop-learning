fn apparently(string: &str) -> String {
	let ret: Vec<&str> = string
		.split_whitespace()
		.collect();

	let mut r: Vec<&str> = vec![];

	for (i, x) in ret.iter().enumerate() {
		if r.len() < 2 {
			r.push(x);
		} else if *x != "apparently" || (*x == "apparently" && (ret[i - 1] != "and" && ret[i - 2] != "but")) {
			r.push(x);
		}

		if *x == "and" || *x == "but" {
			if i < ret.len() - 1 && ret[i + 1] != "apparently" {
				r.push("apparently");
				continue;
			}

			r.push("apparently");
		}
	}

	r.join(" ")
}

fn main() {
	print!("{}", apparently("a xx and eiii but, eee but"));
}

#[test]
fn test_apparently() {
    assert_eq!(apparently("It was great and I have never been on live television before but sometimes I dont watch this."), "It was great and apparently I have never been on live television before but apparently sometimes I dont watch this.");
    assert_eq!(apparently("and"), "and apparently");
    assert_eq!(apparently("and apparently"), "and apparently");
    assert_eq!(apparently("apparently"), "apparently");
}