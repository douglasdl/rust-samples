use std::io;

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let age: i32 = input.trim().parse().expect("Please enter a valid number");
    
    // TODO: Write your code below using a match expression with range patterns
    let age_category = match age {
      0..=12 => "Child",
      13..=19 => "Teenager",
      20..=64 => "Adult",
      65..=120 => "Senior",
      _ => "Invalid age",
    };

    // Print the result
    println!("{}", age_category);
}