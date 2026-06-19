/*
String Methods



Rust has a rich set of built-in string methods that allow you to perform various operations on strings. These methods provide functionalities for manipulating, searching, and transforming strings. Here are some commonly used string methods:

len(): Returns the number of characters in the string.
chars().nth(index): Returns the character at the specified index (0-based). This method returns an Option<char> — a special Rust type that represents either Some value (if the index exists) or None (if it doesn't). Calling .unwrap() on it extracts the character value directly.
contains(substring): Checks if the string contains the specified substring.
starts_with(prefix): Checks if the string starts with the specified prefix.
ends_with(suffix): Checks if the string ends with the specified suffix.
to_uppercase(): Converts the string to uppercase.
to_lowercase(): Converts the string to lowercase.
trim(): Removes leading and trailing whitespace from the string.
For example:

let message = "Hello, World!";
let length = message.len();
let firstChar = message.chars().nth(0).unwrap();
let contains_world = message.contains("World");
let starts_with_hello = message.starts_with("Hello");
let lower = message.to_lowercase();
*/ 

use std::io;

fn analyze_string(s: &str) {
    // Write your code here
    let length = s.len();
    println!("Length: {}", length);

    let charAt4 = s.chars().nth(4).unwrap();
    println!("Char at 4: {}", charAt4);

    let contains_rust = s.contains("Rust");
    println!("Contains Rust: {}", contains_rust);

    let ends_with_dot = s.ends_with(".");
    println!("Ends with dot: {}", ends_with_dot);
    
    let upper = s.to_uppercase();
    println!("Uppercase: {}", upper);
}

fn main() {
    let mut message = String::new();
    io::stdin().read_line(&mut message).unwrap();
    let message = message.trim();
    analyze_string(message);
}