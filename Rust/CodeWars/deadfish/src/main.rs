fn main() {
    println!("Hello, world!");
}

fn parse(code: &str) -> Vec<i32> {
	let mut r = 0;
	let mut ret = Vec::new();

	for c in code.chars() {
		match c {
			'i' => r += 1,
			'd' => r -= 1,
			's' => r *= r,
			'o' => ret.push(r),
			_   => (),
		}
	}

	ret
}

#[test]
fn sample_tests() {
    assert_eq!(parse("iiisdoso"), vec![8, 64]);
    assert_eq!(parse("iiisdosodddddiso"), vec![8, 64, 3600]);
}
