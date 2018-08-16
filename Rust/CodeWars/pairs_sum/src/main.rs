// Failed. Crashed on 10M sequence =(
fn main() {
    let l1 = [10, 5, 2, 3, 7, 5];
    println!("{:?}", sum_pairs(&l1, 10));
}

fn sum_pairs(ints: &[i8], s: i8) -> Option<(i8, i8)> {
	type X = (u32, (i8, i8));
	let mut y: Vec<X> = vec![];
	let mut max_k: u32 = ints.len() as u32 - 1;
	let mut got_lower_k = false;

	'outer: for (i, k) in ints.iter().enumerate() {
		for (i1, k1) in ints.iter().enumerate() {
			if i1 > i && *k as i8 + *k1 as i8 == s {
				if got_lower_k && max_k < (*k1 as u32) {
					break 'outer;
				}

				y.push((i1 as u32, (*k, *k1)));

				if (*k1 as u32) < max_k {
					max_k = *k1 as u32;
					got_lower_k = true;
				}
			}
		}
	}

	if y.len() > 0 {
		let mut min_y: u32 = y[0].0;
		let mut ret = y[0].1;

		for i in y.iter() {
			if i.0 < min_y {
				min_y = i.0;
				ret = i.1;
			}
		}

		return Some(ret);
	}

	None
}

#[test]
fn returns_expected() {
    let l1 = [1, 4, 8, 7, 3, 15];
    let l2 = [1, -2, 3, 0, -6, 1];
    let l3 = [20, -13, 40];
    let l4 = [1, 2, 3, 4, 1, 0];
    let l5 = [10, 5, 2, 3, 7, 5];
    let l6 = [4, -2, 3, 3, 4];
    let l7 = [0, 2, 0];
    let l8 = [5, 9, 13, -3];
    assert_eq!(sum_pairs(&l1, 8), Some((1, 7)));
    assert_eq!(sum_pairs(&l2, -6), Some((0, -6)));
    assert_eq!(sum_pairs(&l3, -7), None);
    assert_eq!(sum_pairs(&l4, 2), Some((1, 1)));
    assert_eq!(sum_pairs(&l5, 10), Some((3, 7)));
    assert_eq!(sum_pairs(&l6, 8), Some((4, 4)));
    assert_eq!(sum_pairs(&l7, 0), Some((0, 0)));
    assert_eq!(sum_pairs(&l8, 10), Some((13, -3)));
}
