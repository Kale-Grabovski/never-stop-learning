fn main() {
    println!("{}", zoom(103));
}

fn zoom(n: i32) -> String {
	let k = n as usize;
	let mut state: Vec<Vec<String>> = vec![vec!["□".to_string(); k]; k];
	let mut tl = n / 2;
	let mut rb = n / 2;
	state[k / 2][k / 2] = "■".to_string();

	while tl >= 0 {
		for c in tl..rb + 1 {
			state[tl as usize][c as usize] = "■".to_string();
			state[c as usize][rb as usize] = "■".to_string();
			state[rb as usize][c as usize] = "■".to_string();
			state[c as usize][tl as usize] = "■".to_string();
		}

		tl -= 2;
		rb += 2;
	}

	let x: Vec<String> = state.iter().map(|x| x.join("")).collect();
	x.join("\n")
}

#[test]
fn basic_test_1() {
  assert_eq!(zoom(1), "■");
}

#[test]
fn basic_test_2() {
  assert_eq!(zoom(3), "\
□□□
□■□
□□□"
  );
}

#[test]
fn basic_test_3() {
  assert_eq!(zoom(5), "\
■■■■■
■□□□■
■□■□■
■□□□■
■■■■■"
  );
}

#[test]
fn basic_test_4() {
  assert_eq!(zoom(7), "\
□□□□□□□
□■■■■■□
□■□□□■□
□■□■□■□
□■□□□■□
□■■■■■□
□□□□□□□"
  );
}

#[test]
fn basic_test_5() {
  assert_eq!(zoom(9), "\
■■■■■■■■■
■□□□□□□□■
■□■■■■■□■
■□■□□□■□■
■□■□■□■□■
■□■□□□■□■
■□■■■■■□■
■□□□□□□□■
■■■■■■■■■"
  );
}
