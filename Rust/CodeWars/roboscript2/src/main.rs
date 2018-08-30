extern crate regex;
use regex::{Regex};

fn get_dims(s: &str) -> String {
	if s == "" {
		return "*".to_string();
	}

	let re = Regex::new(r"([FLR])(\d*)").unwrap();
	let mut angle = 0;
	let mut offset = (0, 0); // (right, down)
	let mut max_offset = (0, 0, 0, 0); // (right, down, left, top)
	let mut first_direction: (i32, i32) = (0, 0);

	for (i, r) in re.captures_iter(s).enumerate() {
		let st: &str = &r[1];
		let c: i32 = match &r[2] {
			"" => 1,
			_  => (&r[2]).parse().unwrap(), 
		};

		match st {
			"F" => {
				let direction: (i32, i32) = match angle {
					90  | -270 => (0, 1),
					180 | -180 => (-1, 0),
					270 | -90  => (0, -1),
					_          => (1, 0),
				};

				if i == 0 {
					first_direction = direction;
					offset.0 += direction.0;
					offset.1 += direction.1;
				}

				offset.0 += direction.0 * c;
				offset.1 += direction.1 * c;

				if offset.0 > max_offset.0 {
					max_offset.0 = offset.0;
				}
				if offset.0 < max_offset.2 {
					max_offset.2 = offset.0;
				}
				if offset.1 > max_offset.1 {
					max_offset.1 = offset.1;
				}
				if offset.1 < max_offset.3 {
					max_offset.3 = offset.1;
				}
			},
			"L" => {
				for _ in 0..c {
					angle -= 90;
					if angle == -360 {
						angle = 0;
					}
				}
			}
			"R" => {
				for _ in 0..c {
					angle += 90;
					if angle == 360 {
						angle = 0;
					}
				}
			}
			_ => (),
		};
	}

	let dims: (usize, usize) = ((max_offset.1 - max_offset.3).abs() as usize, (max_offset.0 - max_offset.2).abs() as usize);
	let mut state = vec![vec!['-'; (dims.0 * 3) as usize]; (dims.1 * 3) as usize];

	let mut angle = 0;
	let mut coord: (usize, usize) = (dims.0, dims.1);

	state[coord.1 as usize][coord.0 as usize] = '*';
	coord.0 += first_direction.0 as usize;
	coord.1 += first_direction.1 as usize;
	state[coord.1 as usize][coord.0 as usize] = '*';

	for r in re.captures_iter(s) {
		let st: &str = &r[1];
		let c: i32 = match &r[2] {
			"" => 1,
			_  => (&r[2]).parse().unwrap(), 
		};

		match st {
			"F" => {
				let direction = match angle {
					90  | -270 => (0, 1),
					180 | -180 => (-1, 0),
					270 | -90  => (0, -1),
					_          => (1, 0),
				};

				coord.0 += (direction.0 * c) as usize;
				coord.1 += (direction.1 * c) as usize;
				state[coord.1 as usize][coord.0 as usize] = '*';
			},
			"L" => {
				for _ in 0..c {
					angle -= 90;
					if angle == -360 {
						angle = 0;
					}
				}
			}
			"R" => {
				for _ in 0..c {
					angle += 90;
					if angle == 360 {
						angle = 0;
					}
				}
			}
			_ => (),
		};
	}

	let mut rows = vec![];
	for (r, row) in (&state).iter().enumerate() {
		let mut is = true;
		for i in row {
			if *i != '-' {
				is = false;
			}
		}

		if is {
			rows.push(r);
		}
	}
	for r in rows {
		for i in 0..state[0].len() {
			state[r][i] = '$';
		}
	}

	let mut cols = vec![];
	for c in 0..state[0].len() {
		let mut is = true;
		for i in 0..state.len() {
			if state[i][c] != '-' && state[i][c] != '$' {
				is = false;
			}
		}

		if is {
			cols.push(c);
		}
	}
	for c in cols {
		for r in 0..state.len() {
			state[r][c] = '$';
		}
	}

	let mut res = String::new();
	let result = state.iter().flat_map(|x| x).filter(|&x| *x != '$').collect::<Vec<&char>>();
	for c in result.chunks(dims.1 + 1).collect::<Vec<_>>() {
		for i in c {
			res.push(**i);
		} 
		res.push('\n');
	}

	res.trim().to_string()
}

fn main() {
	//println!("{}", get_dims("LF5RF3RF3RF7"));
	println!("{}", get_dims("FFRF"));
}
