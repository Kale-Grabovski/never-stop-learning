fn main() {
    println!("{}", valid_braces("[({})]()"));
}

fn valid_braces(s: &str) -> bool {
	let mut open_braces = Vec::new();

	for c in s.chars() {
		match c {
			'{' | '(' | '[' => open_braces.push(c),
			'}' | ')' | ']' => {
				match open_braces.pop() {
					Some(t) => {
						if t == '{' && c != '}' || t == '[' && c != ']' || t == '(' && c != ')'  {
							return false;
						}
					},
					None => {
						return false;
					}
				}
			},
			_ => (),
		}
	}

	open_braces.is_empty()
}

#[test]
fn basic_tests() {
  assert_eq!(true, valid_braces("()"));
  assert_eq!(true, valid_braces("(){}[]"));
  assert_eq!(true, valid_braces("([{}])"));
  assert_eq!(false, valid_braces("[(])"));
  assert_eq!(false, valid_braces("(}"));
  assert_eq!(false, valid_braces("[(])"));
  assert_eq!(false, valid_braces("[({})](]"));
}