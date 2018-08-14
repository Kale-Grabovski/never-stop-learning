fn main() {
    println!("{}", order("is2 Thi1s T4est 3a"));
}

fn get_int_char(s: &str) -> Option<char> {
	for c in s.chars() {
		if c as u8 >= 48 && c as u8 <= 57 {
			return Some(c);
		}
	}

	None
}

fn order(sentence: &str) -> String {
	let mut s: Vec<&str> = sentence.split_whitespace().collect();
	s.sort_by(|l, r| {
			let t1 = get_int_char(l).unwrap();
			let t2 = get_int_char(r).unwrap();
			t1.cmp(&t2)
		});
	s.join(" ")
}

#[test]
fn returns_expected() {
    assert_eq!(order("is2 Thi1s T4est 3a"), "Thi1s is2 3a T4est");
    assert_eq!(order("is2 dd9 Thi1s T4est 3a"), "Thi1s is2 3a T4est dd9");
    assert_eq!(order(""), "");
}

// Better solution
// fn order(sentence: &str) -> String {
//    let mut ws: Vec<_> = sentence.split_whitespace().map(String::from).collect();
//    ws.sort_by_key(|s| s.chars().find(|c| c.is_digit(10)).unwrap());
//    ws.join(" ")
//}
