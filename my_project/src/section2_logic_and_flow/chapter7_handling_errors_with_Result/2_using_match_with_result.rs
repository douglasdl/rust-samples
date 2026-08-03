/*
Using 'match' with 'Result'



Now that you understand what a Result is, let's learn the most explicit and powerful way to handle both success and failure cases: using a match expression.

Just like with Option, match forces you to handle both variants of a Result. You create one arm for Ok to handle success, and another arm for Err to handle failure:

let result = divide(10, 2);

match result {
    Ok(value) => println!("Division successful: {}", value),
    Err(error) => println!("Division failed: {}", error),
}
When the Result is Ok, the match extracts the successful value and executes the first arm. When it's Err, it extracts the error information and executes the second arm. This pattern ensures you never accidentally ignore a potential error.

Challenge

Easy
You will receive two inputs. The first input is a number (as a string), and the second input is another number (as a string). Create a function that attempts to divide the first number by the second number and returns a Result<f64, &'static str>. If the second number is zero, return an Err with the message "Division by zero". Otherwise, return Ok with the division result. Use a match expression to handle both the Ok and Err cases and print the appropriate message.

Requirements:

Read the first input (dividend) and trim whitespace
Parse the first input to f64
Read the second input (divisor) and trim whitespace
Parse the second input to f64
Create a function that takes two f64 parameters and returns Result<f64, &'static str>
Inside the function, check if the divisor is 0.0:
If yes, return Err("Division by zero")
If no, return Ok(dividend / divisor)
Call the function with the two parsed numbers
Use a match expression to handle the Result
In the Ok(value) arm, print: Result: [value]
In the Err(error) arm, print: Error: [error]
Input:

First line: A number representing the dividend (e.g., 10.0)
Second line: A number representing the divisor (e.g., 2.0)
Output:

If division is successful: Result: [result]
If division by zero: Error: Division by zero
*/ 

use std::io;

// TODO: Create your divide function here that returns Result<f64, &'static str>
fn divide(dividend: f64, divisor: f64) -> Result<f64, &'static str> {
    if divisor == 0.0 {
        Err("Division by zero")
    } else {
        Ok(dividend / divisor)
    }
}

fn main() {
    // Read first input (dividend)
    let mut dividend_input = String::new();
    io::stdin().read_line(&mut dividend_input).expect("Failed to read line");
    let dividend: f64 = dividend_input.trim().parse().expect("Invalid number");
    
    // Read second input (divisor)
    let mut divisor_input = String::new();
    io::stdin().read_line(&mut divisor_input).expect("Failed to read line");
    let divisor: f64 = divisor_input.trim().parse().expect("Invalid number");
    
    // TODO: Call your divide function and use match to handle the Result
    // Print "Result: [value]" for Ok case
    // Print "Error: [error]" for Err case
    let result = divide(dividend, divisor);

    match result {
        Ok(value) => println!("Result: {}", value),
        Err(error) => println!("Error: {}", error),
    }
}