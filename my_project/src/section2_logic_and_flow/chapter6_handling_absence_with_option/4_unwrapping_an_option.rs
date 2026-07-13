/*
Unwrapping an 'Option'



Sometimes you need to extract the actual value from an Option when you're confident it contains a Some. Rust provides the .unwrap() method for this purpose - it takes the value out of a Some and gives it to you directly.

let maybe_number = Some(42);
let actual_number = maybe_number.unwrap();
println!("The number is: {}", actual_number);
When you call .unwrap() on a Some(value), it returns the inner value. In the example above, actual_number becomes 42 - no longer wrapped in an Option.

However, there's a critical warning: if you call .unwrap() on a None, your program will panic and crash immediately. This makes .unwrap() a potentially dangerous method that should only be used when you're absolutely certain the Option contains a Some.

let empty_option: Option<i32> = None;
let value = empty_option.unwrap(); // This will panic!
Use .unwrap() sparingly and only in situations where you can guarantee the Option contains a value, such as in simple examples or when you've already checked with .is_some().

challenge icon
Challenge

Easy
You will receive two inputs. The first input is a number (as a string), and the second input indicates whether this number should be treated as valid or invalid. If the second input is valid, create an Option<i32> containing Some with the parsed number. If the second input is invalid, create an Option<i32> containing None. Use .unwrap() to extract the value from the Option and print it.

Requirements:

Read the first input and trim whitespace
Read the second input and trim whitespace
Parse the first input to i32 using .parse::<i32>()
Create an Option<i32> variable:
If the second input is valid, assign Some(parsed_number)
If the second input is invalid, assign None
Use .unwrap() to extract the value from the Option
Print the unwrapped value in the format: Value: [number]
Input:

First line: A number as a string (e.g., 42)
Second line: Either valid or invalid
Output:

If the second input is valid: Value: [number]
If the second input is invalid: The program will panic (this is expected behavior for this challenge)
Note: When the second input is invalid, calling .unwrap() on None will cause a panic. This demonstrates the dangerous nature of .unwrap() when you're not certain the Option contains a value.
*/ 

use std::io;

fn main() {
    // Read the first input (number as string)
    let mut number_input = String::new();
    io::stdin().read_line(&mut number_input).expect("Failed to read line");
    let number_input = number_input.trim();
    
    // Read the second input (valid or invalid)
    let mut validity_input = String::new();
    io::stdin().read_line(&mut validity_input).expect("Failed to read line");
    let validity_input = validity_input.trim();
    
    // Parse the number
    let parsed_number: i32 = number_input.parse().expect("Failed to parse number");
    
    // TODO: Write your code below
    // Create an Option<i32> based on the validity_input
    // If validity_input is "valid", assign Some(parsed_number)
    // If validity_input is "invalid", assign None
    let maybe_input: Option<i32> = if validity_input == "valid" {
        Some(parsed_number)
    } else {
        None
    };

    // Then use .unwrap() to extract the value and print it in the format: Value: [number]
    let value = maybe_input.unwrap();

    println!("Value: {}", value);
}