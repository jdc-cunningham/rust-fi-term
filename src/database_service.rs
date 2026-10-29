use rusqlite::{params, Connection, Result};

pub fn db_exists() -> bool {

}

pub fn create_db() -> Result<()> {
  let conn = Connection::open("rust_fi_term.db")?;

  conn.execute(
      "CREATE TABLE credit_cards (
          id INTEGER PRIMARY KEY,
          name TEXT NOT NULL,
          balance REAL NOT NULL,
          min_due INTEGER NOT NULL,
          due_date INTEGER NOT NULL,
          is_closed INTEGER NOT NULL,
          apr REAL NOT NULL,
          credit_limit REAL NOT NULL
      ) STRICT;",
  )?;
}