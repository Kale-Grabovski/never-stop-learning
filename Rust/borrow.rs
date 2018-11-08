#[derive(Debug)]
struct X {
	x: String,
	y: String,
}

fn get_d() -> Vec<X> {
	let mut y: Vec<X> = vec![];
	let x = vec![
		X{x: String::new(), y: String::new()},
		X{x: String::new(), y: String::new()},
	];
	for i in x.iter() {
		y.push(*i);
	}

	y
}

fn main() {
	let d = get_d();
	println!("{:?}", d);
}