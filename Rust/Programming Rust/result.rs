use std::fs::File;

fn create(filename: &str) -> std::io::Result<()> {
	File::create("xxx")?; // is equivalent of

	match File::create(filename) {
		Ok(f)  => f,
		Err(e) => return Err(e),
	};

	Ok(())
}

fn main() {
	let t = create("/tmp/xxx");
	println!("{:?}", t);
}
