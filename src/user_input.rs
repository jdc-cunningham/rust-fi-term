use std::io::{self, Write};

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
