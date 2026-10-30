mod models;
// mod database_service;
mod user_input;

use rusqlite::{params, Connection, Result};

fn main() {
    user_input::man_page();
    let user_choice = user_input::wait_for_man_page_choice();

    if user_choice == "list" {
        println!("list selected");
    }

    if user_choice == "add credit card" {
        println!("add credit card selected");
    }
}
