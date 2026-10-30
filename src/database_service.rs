use rusqlite::{params, Connection, Result};
use std::path::Path;

pub fn db_exists() -> bool {
    return Path::new("src/rust_fi_term.db").exists();
}

pub fn check_table_exists(conn: &Connection, table_name: &str) -> Result<bool> {
    let mut stmt = conn.prepare(
        "SELECT name from rust_fi_term WHERE type='table' AND name=?1"
    )?;

    let exists: bool = stmt.exists([table_name])?;
    Ok(exists)
}

pub fn create_db() -> Result<()> {
  let conn = Connection::open("src/rust_fi_term.db")?;

  conn.execute(
      "CREATE TABLE credit_cards (
          id INTEGER PRIMARY KEY,
          name TEXT NOT NULL,
          balance REAL NOT NULL,
          min_due INTEGER NOT NULL,
          due_date INTEGER NOT NULL,
          is_closed INTEGER NOT NULL,
          apr INTEGER NOT NULL,
          credit_limit REAL NOT NULL
      ) STRICT;",
       ()
  )?;

  Ok(())
}

pub fn init_db() -> Result<()> {
    if !db_exists() {
        let _ = create_db();
    } else {
        let conn = Connection::open("rust_fi_term.db")?;
        let _ = check_table_exists(&conn, "credit_cards");
    }

    Ok(())
}
