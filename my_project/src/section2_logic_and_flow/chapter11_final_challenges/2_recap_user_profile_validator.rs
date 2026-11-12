/*
Recap - User Profile Validator

challenge icon
Challenge

Easy
You will receive three inputs representing a user profile. The first input is a username, the second is an age, and the third is an email address. Validate the profile according to the following rules and print the appropriate message:

Validation Rules:

Username must be at least 3 characters long
Age must be between 13 and 120 (inclusive)
Email must contain the @ symbol
Requirements:

Read the username input and trim whitespace
Read the age input, trim whitespace, and parse it to i32
Read the email input and trim whitespace
Check if the username length is at least 3 characters
Check if the age is in the range 13 to 120 (use match with ranges)
Check if the email contains the @ character (use the .contains() method)
If all validations pass, print: Valid profile
If the username is too short, print: Invalid username
If the age is out of range, print: Invalid age
If the email doesn't contain @, print: Invalid email
Check validations in order: username first, then age, then email
Input:

First line: A username (e.g., alice123)
Second line: An age (e.g., 25)
Third line: An email address (e.g., alice@example.com)
Output:

One of: Valid profile, Invalid username, Invalid age, or Invalid email
*/ 

use std::io;

fn main() {
    // Read username input
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Failed to read line");
    let username = username.trim();
    
    // Read age input
    let mut age_input = String::new();
    io::stdin().read_line(&mut age_input).expect("Failed to read line");
    let age: i32 = age_input.trim().parse().expect("Failed to parse age");
    
    // Read email input
    let mut email = String::new();
    io::stdin().read_line(&mut email).expect("Failed to read line");
    let email = email.trim();
    
    // TODO: Write your code below to validate the profile
    // Check username length, age range, and email format
    // Print the appropriate message based on validation results
    if username.len() < 3 {
        println!("Invalid username");
        return;
    }

    match age {
        13..=120 => {},
        _ => {
            println!("Invalid age");
            return;
        }
    }

    if !email.contains('@') {
        println!("Invalid email");
        return;
    }

    println!("Valid profile");    
}