mod models;
// mod database_service;
mod user_input;

use rusqlite::{params, Connection, Result};

fn main() {
    user_input::man_page();
}
