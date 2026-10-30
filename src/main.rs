mod models;
mod database_service;
mod user_input;

use rusqlite::{params, Connection, Result};

fn main() {
    // init DB
    

    // ready for user input
    user_input::man_page();
    let user_choice = user_input::wait_for_man_page_choice();

    if user_choice == "list" {
        println!("list selected");
    }

    if user_choice == "add credit card" {
        let credit_card = user_input::add_credit_card();
        println!("Card info:");
        println!("{:?}", credit_card);
    }
}
