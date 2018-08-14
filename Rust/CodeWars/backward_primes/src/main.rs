fn is_prime(d: u64) -> bool {
	for i in 2 .. (d as f64).sqrt() as u64 + 1 {
		if d % i == 0 {
			return false
		}
	}

	true
}

fn reverse_int(i: u64) -> u64 {
	let s: String = i.to_string().chars().rev().collect();
	s.parse::<u64>().unwrap()
}

fn backwards_prime(start: u64, stop: u64) -> Vec<u64> {
	let mut ret = Vec::new();
	for i in start..stop + 1 {
		let rev = reverse_int(i);
		if i > 10 && rev != i && is_prime(i) && is_prime(rev) {
			ret.push(i);
		}
	}

	ret
}

fn main() {
	print!("{:?}", backwards_prime(1, 100));
}

#[test]
fn tests_backwards_prime() {
    assert_eq!(backwards_prime(1, 100), vec![13, 17, 31, 37, 71, 73, 79, 97]);
    assert_eq!(backwards_prime(1, 31), vec![13, 17, 31]);
    assert_eq!(backwards_prime(9900, 10000), vec![9923, 9931, 9941, 9967]);
}
