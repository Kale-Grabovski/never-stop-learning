extern crate regex;
use regex::{Regex};

fn print_state(state: &Vec<Vec<char>>) {
	let x: Vec<String> = state.iter().map(|x| x.into_iter().collect()).collect();
	println!("{}", x.join("\n"));
}

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

	let tt: Vec<Vec<char>> = state.iter()
		.filter(|&x| x.iter()
			.filter(|&t| *t != '-')
			.map(|x| *x)
			.collect::<Vec<char>>()
			.into_iter()
			.collect::<String>() != ""
		)
		.map(|&x| *x)
		.collect();
	print_state(&tt);

	let mut result = vec![vec![' '; (dims.0 + 1) as usize]; (dims.1 + 1) as usize];
	for r in 0..dims.0 {
		for c in 0..dims.1 {
			//println!("{} {}", r + row, c + col);
		}
	}

	let x: Vec<String> = result.iter().map(|x| x.into_iter().collect()).collect();
	x.join("\n")
}

fn main() {
	//println!("{}", get_dims("LF5RF3RF3RF7"));
	println!("{}", get_dims("FFRFFF"));
}
