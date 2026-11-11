/*
Shortcuts: unwrap and expect



You've already learned about .unwrap() and .expect() with Option, and these same methods work with Result too. However, their behavior is slightly different when dealing with success and failure cases.

The .unwrap() method extracts the value from an Ok variant, but if the Result is an Err, your program will panic and crash. Similarly, .expect() does the same thing but allows you to provide a custom panic message that makes debugging easier:

let success: Result = Ok(42);
let value = success.expect("This should work!");
println!("Value: {}", value); // Prints: Value: 42

// This would panic with the custom message:
// let failure: Result = Err("Something went wrong");
// let value = failure.expect("This should work!"); // Panics!
While these methods provide quick access to successful values, they should be used carefully. They're most appropriate when you're absolutely certain the operation will succeed, or during development and testing.

challenge icon
Challenge

Easy
You will receive two inputs. The first input is a number (as a string), and the second input is a status indicator (ok or error). Create a Result<i32, &'static str> based on the status. If the status is ok, create an Ok containing the parsed number. If the status is error, create an Err with the message "Invalid operation". Use .expect() with a custom message to extract the value from the Result and print it.

Requirements:

Read the first input (number) and trim whitespace
Parse the first input to i32
Read the second input (status) and trim whitespace
Create a Result<i32, &'static str> variable:
If the status is ok, assign Ok(parsed_number)
If the status is error, assign Err("Invalid operation")
Use .expect("Failed to get value") to extract the value from the Result
Print the extracted value in the format: Value: [value]
Input:

First line: A number (e.g., 75)
Second line: Either ok or error
Output:

If the status is ok: Value: [number]
If the status is error: The program will panic with the custom message (not tested in this challenge)
Note: For this challenge, test cases will only use the ok status to ensure successful execution. The error case would cause a panic, which is the expected behavior of .expect() when encountering an Err.
*/ 

use std::io;

fn main() {
    // Lê a primeira entrada (número)
    let mut number_input = String::new();
    io::stdin().read_line(&mut number_input).expect("Failed to read line");
    let number_input = number_input.trim();
    
    // Converte o número para i32
    let parsed_number: i32 = number_input.parse().expect("Failed to parse number");
    
    // Lê a segunda entrada (status)
    let mut status = String::new();
    io::stdin().read_line(&mut status).expect("Failed to read line");
    let status = status.trim();
    
    // TODO: Escreva seu código abaixo
    // Crie um Result<i32, &'static str> baseado no status
    let result: Result<i32, &'static str> = if status == "ok" {
        Ok(parsed_number)
    } else {
        Err("Invalid operation")
    };

    // Use .expect() para extrair o valor
    let value = result.expect("Failed to get value");
    // Imprima o resultado no formato: Value: [value]
    println!("Value: {}", value);
}