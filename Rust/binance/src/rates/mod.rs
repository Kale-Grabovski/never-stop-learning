extern crate postgres;

use postgres::{Connection};

pub struct Rates<'a> {
    conn: &'a Connection,
}

#[derive(Debug)]
pub struct Pair {
    symbol: String,
    id: i32,
}

impl<'a> Rates<'a> {
    pub fn new(conn: &Connection) -> Rates {
        Rates{conn}
    }

    pub fn get_save_pair(&self, symbol: &str) -> Result<Pair> {
        let row = self.conn.query("SELECT id, symbol FROM pair WHERE symbol = $1", &[&symbol])?;
        println!("{:?}", row);
        Ok(Pair{symbol: symbol.to_string(), id: 0})
    }
}
