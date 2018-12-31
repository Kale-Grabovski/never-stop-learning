#[derive(PartialEq, Debug, Clone)]
enum Direction {NORTH, SOUTH, EAST, WEST}

use Direction::*;

fn dir_reduc(arr: &[Direction]) -> Vec<Direction> {
	let mut res: Vec<Direction> = vec![];

	for r in arr {
	// better: for &s in arr {, no need to clone
		res.push((*r).clone());
		let l = res.len();
		if l > 1 {
			match (&res[l - 1], &res[l - 2]) {
				(&NORTH, &SOUTH) | (&SOUTH, &NORTH) | (&EAST, &WEST) | (&WEST, &EAST) => {res.pop(); res.pop();}
				_ => (),
			}
		}
	}

	res
}

#[test]
fn returns_expected() {
    let a = [];
    assert_eq!(dir_reduc(&a), []);
    let a = [NORTH, SOUTH, SOUTH, EAST, WEST, NORTH, WEST, WEST, WEST, EAST, NORTH, NORTH];
    assert_eq!(dir_reduc(&a), [WEST, WEST, NORTH, NORTH]);
    let a = [NORTH, SOUTH, SOUTH, EAST, WEST, NORTH, WEST];
    assert_eq!(dir_reduc(&a), [WEST]);
    let a = [NORTH, WEST, SOUTH, EAST];
    assert_eq!(dir_reduc(&a), [NORTH, WEST, SOUTH, EAST]);
    let a = [NORTH, SOUTH, SOUTH, EAST, WEST, NORTH];
    assert_eq!(dir_reduc(&a), []);
}
