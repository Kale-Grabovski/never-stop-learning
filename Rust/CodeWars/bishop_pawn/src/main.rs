fn main() {
	let t = bishop_and_pawn("a5", "c3");
	print!("{}", t);
}

fn bishop_and_pawn(bishop: &str, pawn: &str) -> bool {
	let b1 = bishop.chars().nth(0).unwrap();
	let b2 = bishop.chars().nth(1).unwrap();
	let p1 = pawn.chars().nth(0).unwrap();
	let p2 = pawn.chars().nth(1).unwrap();
    ((b1 as i8 - 96) - (p1 as i8 - 96)).abs() == ((b2 as i8 - 48) - (p2 as i8 - 48)).abs()
}

#[test]
fn basic_tests() {
    assert_eq!(bishop_and_pawn("a1", "c3"), true);
    assert_eq!(bishop_and_pawn("a1", "c3"), true);
    assert_eq!(bishop_and_pawn("h1", "h3"), false);
    assert_eq!(bishop_and_pawn("a5", "c3"), true);
    assert_eq!(bishop_and_pawn("g1", "f3"), false);
}
