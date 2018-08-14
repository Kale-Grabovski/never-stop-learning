fn main() {
    println!("{}", chessboard_cell_color("A1", "A2"));
}

fn chessboard_cell_color(cell1: &str, cell2: &str) -> bool {
	let b1 = cell1.chars().nth(0).unwrap();
	let b2 = cell1.chars().nth(1).unwrap();
	let p1 = cell2.chars().nth(0).unwrap();
	let p2 = cell2.chars().nth(1).unwrap();

	(b1 as i16 + b2 as i16).abs() % 2 == (p1 as i16 + p2 as i16).abs() % 2
}

#[test]
fn basic_tests() {
    assert_eq!(chessboard_cell_color("A1", "C3"), true);
    assert_eq!(chessboard_cell_color("A1", "H3"), false);
    assert_eq!(chessboard_cell_color("A1", "A2"), false);
    assert_eq!(chessboard_cell_color("A1", "C1"), true);
    assert_eq!(chessboard_cell_color("A1", "A1"), true);
}
