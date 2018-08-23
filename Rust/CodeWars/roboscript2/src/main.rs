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

	for r in re.captures_iter(s) {
		let st: &str = &r[1];
		let c: u32 = match &r[2] {
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

				for _ in 0..c {
					offset.0 += direction.0;
					offset.1 += direction.1;
				}

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

	let dims = (((max_offset.0 - max_offset.2) as i32).abs(), ((max_offset.1 - max_offset.3) as i32).abs());
	let mut state = vec![vec!['-'; (dims.1 * 3) as usize]; (dims.0 * 3) as usize];

	let mut angle = 0;
	let mut coord = (dims.0 + 1, dims.1 + 1);
	state[coord.1 as usize][coord.0 as usize] = '*';

	for r in re.captures_iter(s) {
		let st: &str = &r[1];
		let c: u32 = match &r[2] {
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

				for _ in 0..c {
					coord.0 += direction.0;
					coord.1 += direction.1;
					state[coord.1 as usize][coord.0 as usize] = '*';
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

	let mut row = 0;
	'outer: for x in 0..dims.0 * 3 {
		for u in 0..dims.1 * 3 {
			if state[x as usize][u as usize] == '*' {
				row = x;
				break 'outer;
			}
		}
	}

	let mut col = 0;
	'out: for u in 0..dims.1 * 3 {
		for x in 0..dims.0 * 3 {
			if state[x as usize][u as usize] == '*' {
				col = u;
				break 'out;
			}
		}
	}

	let mut result = vec![vec![' '; (dims.0 + 1) as usize]; (dims.1 + 1) as usize];
	for r in 0..dims.1 + 1 {
		for c in 0..dims.0 + 1 {
			result[r as usize][c as usize] = state[(r + row) as usize][(c + col) as usize];
			println!("{} {}", r + row, c + col);
		}
	}

	let x: Vec<String> = result.iter().map(|x| x.into_iter().collect()).collect();
	x.join("\n")
}

fn main() {
	//println!("{}", get_dims("LF5RF3RF3RF7"));
	println!("{}", get_dims("F"));
}
