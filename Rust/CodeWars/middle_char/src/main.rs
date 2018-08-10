fn get_middle(s: &str) -> String {
	if s.len() % 2 == 0 {
		return s[s.len() / 2 - 1 .. s.len() / 2 + 1].to_string();
	}

	s[s.len() / 2 .. s.len() / 2 + 1].to_string()
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn example_tests() {
  assert_eq!(get_middle("test"),"es");
  assert_eq!(get_middle("testing"),"t");
  assert_eq!(get_middle("middle"),"dd");
  assert_eq!(get_middle("A"),"A");
  assert_eq!(get_middle("of"),"of");
}
