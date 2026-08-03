/*
is_ok() and is_err()



Sometimes you need to quickly check whether a Result represents success or failure without extracting the actual values. Rust provides two convenient methods for this: .is_ok() and .is_err().

Both methods return a boolean value, making them perfect for simple conditional checks. The .is_ok() method returns true if the Result is Ok, and false if it's Err. The .is_err() method works in reverse:

let success: Result = Ok(42);
let failure: Result = Err("Something went wrong");

if success.is_ok() {
    println!("Operation succeeded!");
}

if failure.is_err() {
    println!("Operation failed!");
}
These methods are particularly useful when you only need to know the outcome status without caring about the specific values inside. Unlike match, they don't consume the Result, so you can still use it afterward.

challenge icon
Challenge

Easy
You will receive two inputs. The first input is a number (as a string), and the second input is a status indicator (success or failure). Create a Result<i32, &'static str> based on the status. If the status is success, create an Ok containing the parsed number. If the status is failure, create an Err with the message "Operation failed". Use .is_ok() and .is_err() to check the result status and print the appropriate message.

Requirements:

Read the first input (number) and trim whitespace
Parse the first input to i32
Read the second input (status) and trim whitespace
Create a Result<i32, &'static str> variable:
If the status is success, assign Ok(parsed_number)
If the status is failure, assign Err("Operation failed")
Use .is_ok() to check if the result is successful:
If true, print: Success detected
Use .is_err() to check if the result is an error:
If true, print: Error detected
Input:

First line: A number (e.g., 42)
Second line: Either success or failure
Output:

If the status is success: Success detected
If the status is failure: Error detected
*/ 

use std::io;

fn main() {
    // Read the first input (number)
    let mut number_input = String::new();
    io::stdin().read_line(&mut number_input).expect("Failed to read line");
    let number_input = number_input.trim();
    
    // Parse the number to i32
    let parsed_number: i32 = number_input.parse().expect("Failed to parse number");
    
    // Read the second input (status)
    let mut status = String::new();
    io::stdin().read_line(&mut status).expect("Failed to read line");
    let status = status.trim();
    
    // TODO: Create a Result<i32, &'static str> based on the status
    // If status is "success", create Ok(parsed_number)
    // If status is "failure", create Err("Operation failed")
    let result: Result<i32, &'static str> = if status == "success" {
        Ok(parsed_number)
    } else {
        Err("Operation failed")
    };
    
    // TODO: Use .is_ok() and .is_err() to check the result and print the appropriate message
    if result.is_ok() {
        println!("Success detected");
    }

    if result.is_err() {
        println!("Error detected");
    }
}