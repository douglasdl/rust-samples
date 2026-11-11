/*
Slices as Function Parameters



When writing functions that work with strings, using &str as a parameter type instead of String makes your functions much more flexible and useful.

A function that takes a String parameter can only accept owned strings, which limits how you can call it:

fn print_message(text: String) {
    println!("{}", text);
}

// This works, but takes ownership
let my_string = String::from("Hello");
print_message(my_string); // my_string is no longer usable after this
However, a function that takes &str can accept both String references and string literals, making it much more versatile:

fn print_message(text: &str) {
    println!("{}", text);
}

// Now you can call it with either type
let my_string = String::from("Hello");
print_message(&my_string); // Pass a reference to String
print_message("World");     // Pass a string literal directly
This flexibility means your functions work in more situations without forcing callers to convert their data or give up ownership.

Challenge

Easy
You will receive two inputs. The first input is a text message, and the second input is the maximum length allowed for the message. Create a function called truncate_message that takes a string slice (&str) and a maximum length (usize) as parameters. If the message length exceeds the maximum, the function should return a slice of the message up to the maximum length. Otherwise, it should return the entire message. Print the result.

Requirements:

Read the first input (the text message) and trim whitespace
Read the second input (the maximum length) and trim whitespace
Parse the maximum length to usize
Define a function truncate_message that takes two parameters:
A string slice &str for the message
A usize for the maximum length
Inside the function, check if the message length is greater than the maximum length
If yes, return a slice of the message from index 0 to the maximum length using &message[0..max_len]
If no, return the entire message
Call the function with the input message and maximum length
Print the returned result
Input:

First line: A text message (e.g., Hello World from Rust)
Second line: Maximum length as a number (e.g., 11)
Output:

The truncated message if it exceeds the maximum length, or the full message otherwise
*/ 

use std::io;

// TODO: Define the truncate_message function here
fn truncate_message(text: &str, max_len: usize) -> &str {
    if text.len() > max_len {
        &text[0..max_len]
    } else {
        text
    }
}

fn main() {
    // Read the text message
    let mut message = String::new();
    io::stdin().read_line(&mut message).expect("Failed to read line");
    let message = message.trim();
    
    // Read the maximum length
    let mut max_length_input = String::new();
    io::stdin().read_line(&mut max_length_input).expect("Failed to read line");
    let max_len: usize = max_length_input.trim().parse().expect("Invalid number");
    
    // TODO: Call the truncate_message function and store the result
    let result = truncate_message(message, max_len);
    
    // TODO: Print the result
    println!("{}", result);
}