use std::io::{self};
use crate::models::CreditCard;

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
pub fn add_credit_card() -> CreditCard {
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

  let mut balance_input: String = String::new(); // cast from string to type

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

  // due date
  println!("What is the monthly payment due date?");

  let mut due_date_input: String = String::new();

  io::stdin()
    .read_line(&mut due_date_input)
    .expect("Failed to get payment due date");

  due_date = due_date_input.trim().parse::<i32>().unwrap();

  // is_closed
  println!("Is this account closed? Type: y or n");   // need validation

  let mut is_closed_input: String = String::new();

  io::stdin()
    .read_line(&mut is_closed_input)
    .expect("Failed to get card status");

  if is_closed_input.trim() == "y" {
    is_closed = true;
  } else {
    is_closed = false;
  }

  // APR
  println!("What is the APR % (0.00)?");

  let mut apr_input: String = String::new();

  io::stdin()
    .read_line(&mut apr_input)
    .expect("Failed to get APR");

  apr = apr_input.trim().parse::<f32>().unwrap();

  // credit limit
  println!("What is the credit limit?");

  let mut credit_limit_input: String = String::new();

  io::stdin()
    .read_line(&mut credit_limit_input)
    .expect("Failed to get payment due date");

  credit_limit = credit_limit_input.trim().parse::<f32>().unwrap();

  return CreditCard {
    name: name.trim().to_string(),
    balance: balance,
    min_due: min_due,
    due_date: due_date,
    is_closed: is_closed,
    apr: apr,
    credit_limit: credit_limit
  };
}
