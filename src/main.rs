mod models;
mod database_service;

use rusqlite::{params, Connection, Result};

fn main() -> Result<()> {
    // connect to sqlite
    let conn = Connection::open("rust_fi_term.db")?;

    Ok(())
}
