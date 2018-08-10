fn main() {
    print!("{:?}", remove_duplicate_words("za za iej ij za ifoi ii ee ii"));
}

fn remove_duplicate_words(s: &str) -> String {
	let mut ret = Vec::new();

    for t in s.split_whitespace() {
		if !exists(&ret, t) {
	    	&ret.push(t);
		}
    }

    ret.join(" ")
}

fn exists(v: &Vec<&str>, s: &str) -> bool {
	for i in v {
		if *i == s {
			return true
		}
	}

	false
}
