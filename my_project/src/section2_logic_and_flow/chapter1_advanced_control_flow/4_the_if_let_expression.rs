use std::io;

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let number: i32 = input.trim().parse().expect("Please enter a valid number");
    
    // TODO: Write your code below
    // Use if let to check if number equals 42
    if let 42 = number {
        println!("The answer!");
    }
}