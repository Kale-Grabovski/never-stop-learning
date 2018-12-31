struct BattleField {
	alien_ships: Vec<Ship>,
	mine_ships: Vec<Ship>,
	sea_width: u8,
	sea_height: u8,
}

impl BattleField {
	pub fn new(w: u8, h: u8, aliens: Vec<Ship>, mine: Vec<Ship>) -> BattleField {
        BattleField {
        	sea_width: w,
        	sea_height: h,
        	alien_ships: aliens,
        	mine_ships: mine,
        }
    }

    pub fn display(&self) {
    	for i in 0..self.sea_height {
	    	for k in 0..self.sea_width {
		        print!("□ ");
		    }
	    	println!("");
    	}
    }
}

struct Coord {
	x: u8,
	y: u8,
}

struct Ship {
	size: u8,
	coord: Coord,
}

fn main() {
	let aliens = vec![];
	let mine = vec![];
	let field = BattleField::new(10, 10, aliens, mine);
	field.display();
}
