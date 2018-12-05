extern crate postgres;

use postgres::{Connection, Error};

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

    pub fn get_save_pair(&self, symbol: &str) -> Result<Pair, Error> {
        let mut rows = self.conn.query("SELECT id, symbol FROM pair WHERE symbol = $1", &[&symbol])?;
        if rows.len() == 0 {
            rows = self.conn.query("INSERT INTO pair (symbol) VALUES ($1) RETURNING id", &[&symbol])?;
            return Ok(Pair{symbol: symbol.to_string(), id: rows.get(0).get(0)});
        }

        let row = rows.get(0);
        Ok(Pair{symbol: row.get(1), id: row.get(0)})
    }
}
