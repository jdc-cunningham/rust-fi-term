use std::io::{self, Write};
use crate::models;

// temporary until UI esablished
pub fn man_page() {
  println!("Rust Finance Terminal");
  println!("Available commands:");
  println!("- list");
  println!("- add credit card");
}

pub fn wait_for_man_page_choice() -> String {
  let mut input = String::new();

  io::stdin()
    .read_line( &mut input)
    .expect("Failed to read line");

  return input.trim().to_string();
}

// this is nasty probably way to loop this, go from a model directly
pub fn add_credit_card() {
  let mut name: String = String::new();
  let mut balance: f32;
  let mut min_due: i32;
  let mut due_date: i32;
  let mut is_closed: bool;
  let mut apr: f32;
  let mut credit_limit: f32;

  // name
  println!("What is the name of the credit card?");

  io::stdin()
    .read_line(&mut name)
    .expect("Failed to get name");

  // balance
  println!("What is the current balance?");

  let mut balance_input: String = String::new();

  io::stdin()
    .read_line(&mut balance_input)
    .expect("Failed to get balance");

  balance = balance_input.trim().parse::<f32>().unwrap();

  // min_due
  println!("What is the min payment due?");

  let mut min_due_input: String = String::new();

  io::stdin()
    .read_line(&mut min_due_input)
    .expect("Failed to get min due");

  min_due = min_due_input.trim().parse::<i32>().unwrap();

}
