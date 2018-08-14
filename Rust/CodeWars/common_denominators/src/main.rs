fn main() {
	let i = convert_fracts(vec![(690, 1300), (87, 1310), (30, 40)]);
	print!("{:?}", i);
}

fn is_common_denominator(l: &[i64], denom: i64) -> bool {
	for t in l.iter() {
		if denom % t != 0 {
			return false;
		}
	}

	true
}

fn get_max_denominator(l: i64, k: i64) -> i64 {
	let mut d = 1;
	for i in 2 .. k / 2 as f64 as i64 + 1 {
		if l % i == 0 && k % i == 0 && i > d {
			d = i;
		}
	}

	d
}

fn convert_fracts(l: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
	let mut max_denom = l[0].1;
	let mut denoms = Vec::new();
	let l: Vec<(i64, i64)> = l.iter()
		.map(|i| {
			let max = get_max_denominator(i.0, i.1);
			(i.0 / max, i.1 / max)
		})
		.collect();

	for t in l.iter() {
		if t.1 > max_denom {
			max_denom = t.1;
		}
		denoms.push(t.1);
	}

	let mut mult = 1;
	loop {
		if is_common_denominator(&denoms, max_denom * mult) {
			break;
		}

		mult += 1;
	}

	let mut ret = Vec::new();
	for t in l.iter() {
		ret.push((t.0 * max_denom * mult / t.1, max_denom * mult));
	}

	ret
}

#[test]
fn basics_convert_fracts() {
    assert_eq!(convert_fracts(vec![(69, 130), (87, 1310), (3, 4)]), vec![(18078, 34060), (2262, 34060), (25545, 34060)]);
    assert_eq!(convert_fracts(vec![(690, 1300), (87, 1310), (30, 40)]), vec![(18078, 34060), (2262, 34060), (25545, 34060)]);
}
