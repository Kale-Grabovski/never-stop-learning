fn main() {
    println!("{}", highlight("F33RR(F)F"));
}

fn get_color(c: char) -> String {
	match c {
		'F' => "pink".to_string(),
		'L' => "red".to_string(),
		'R' => "green".to_string(),
		'0'...'9' => "orange".to_string(),
		_   => [c].iter().collect::<String>(),
	}
}

fn get_wrapped(color: &str, last_seq: &Vec<char>) -> String {
	let seq: String = last_seq.into_iter().collect();
	match color {
		"pink" | "red" | "green" | "orange" => [
			"<span style=\"color: ".to_string(),
			color.to_string(),
			"\">".to_string(),
			seq,
			"</span>".to_string(),
		].join(""),
		_ => seq,
	}
}

pub fn highlight(code: &str) -> String {
	let mut result = String::from("");
	let mut last_seq: Vec<char> = Vec::new();
	let mut last_seq_colors: Vec<String> = Vec::new();

	for c in code.chars() {
		let color = get_color(c);

		if last_seq.len() > 0 && color != last_seq_colors.iter().last().unwrap().clone() {
			let xxx = get_wrapped(&last_seq_colors.iter().last().unwrap().clone(), &last_seq);
			result.push_str(&xxx);

			last_seq = Vec::new();
			last_seq_colors = Vec::new();
		}

		last_seq.push(c);
		last_seq_colors.push(color);
	}

	if last_seq.len() > 0 {
		let xxx = get_wrapped(&last_seq_colors.iter().last().unwrap().clone(), &last_seq);
		result.push_str(&xxx);
	}

	result
}


/// Better
///extern crate regex;
///use regex::{Regex, Captures};
///pub fn highlight(code: &str) -> String {
///    let re = Regex::new(r"F+|L+|R+|\d+").unwrap();
///    re.replace_all(code, |c: &Captures| match c[0].chars().next().unwrap() {
///        'F' => format!(r#"<span style="color: pink">{}</span>"#, &c[0]),
///        'L' => format!(r#"<span style="color: red">{}</span>"#, &c[0]),
///        'R' => format!(r#"<span style="color: green">{}</span>"#, &c[0]),
///        _ => format!(r#"<span style="color: orange">{}</span>"#, &c[0]),
///    }).to_string()
///}

#[cfg(test)]
macro_rules! assert_highlight {
  ($code:expr , $expected:expr $(,)*) => {{
    let actual = highlight($code);
    let expected = $expected;
    println!("Code without syntax highlighting: {}", $code);
    println!("Your code with syntax highlighting: {}", actual);
    println!("Expected syntax highlighting: {}", expected);
    assert_eq!(actual, expected);
  }};
}

#[test]
fn examples_in_description() {
  assert_highlight!(
    "F3RF5LF7",
    r#"<span style="color: pink">F</span><span style="color: orange">3</span><span style="color: green">R</span><span style="color: pink">F</span><span style="color: orange">5</span><span style="color: red">L</span><span style="color: pink">F</span><span style="color: orange">7</span>"#,
  );
  assert_highlight!(
    "F33RR(F)F",
    r#"<span style="color: pink">F</span><span style="color: orange">33</span><span style="color: green">RR</span>(<span style="color: pink">F</span>)<span style="color: pink">F</span>"#,
  );
  assert_highlight!(
    "FFFR345F2LL",
    r#"<span style="color: pink">FFF</span><span style="color: green">R</span><span style="color: orange">345</span><span style="color: pink">F</span><span style="color: orange">2</span><span style="color: red">LL</span>"#,
  );
}
