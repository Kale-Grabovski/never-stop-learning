use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
}

fn trotter(n: i32)-> i32{
	let mut m = HashSet::new();

	for i in 1.. {
		if n * (i + 1) > std::i32::MAX {
			break;
		}

		for c in (n * i).to_string().chars() {
			m.insert(c);
		}

		if m.len() == 10 {
			return n * i
		}
	}

	-1
}

#[test]
fn returns_expected() {
  assert_eq!(trotter(1692), 5076);
  assert_eq!(trotter(2), 90);
  assert_eq!(trotter(7), 70);
}
