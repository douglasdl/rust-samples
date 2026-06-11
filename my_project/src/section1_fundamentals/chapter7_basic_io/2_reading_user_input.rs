use std::io;

fn main() {
    // Create a String object
    let mut name = String::new();
    
    // Prompt the user to enter their name (use println)
    println!("Enter your name:");
    
    // Read the user's name
    io::stdin().read_line(&mut name).unwrap();
    
    // Print the greeting message
    println!("Hello, {}!", name);
}