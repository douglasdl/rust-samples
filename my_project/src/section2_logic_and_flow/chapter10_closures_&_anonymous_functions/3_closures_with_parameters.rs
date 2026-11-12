/*
Closures with Parameters



While closures that take no parameters are useful, most of the time you'll want to pass data into your closures to make them more flexible and powerful. Adding parameters to closures follows the same pattern you learned earlier, but with values inside the vertical bars.

Here's how you define a closure that accepts one parameter:

let add_one = |x: i32| x + 1;
Notice that the parameter x goes between the vertical bars, just like function parameters. You can then call this closure by passing a value:

let add_one = |x: i32| x + 1;
let result = add_one(5); // result is 6
For closures with multiple parameters, separate them with commas:

let multiply = |x: i32, y: i32| x * y;
let result = multiply(3, 4); // result is 12

Closures with Parameters


challenge icon
Challenge

Easy
You will receive two inputs. The first input is a number, and the second input is also a number. Create a closure that takes two parameters and calculates the power (first parameter raised to the second parameter). Store the closure in a variable, call it with the input values, and print the result.

Requirements:

Read the first input (base number) and trim whitespace
Parse it to i32
Read the second input (exponent) and trim whitespace
Parse it to u32
Create a closure with two parameters that calculates the power using the .pow() method
Store the closure in a variable
Call the closure with both input values
Print the result
Input:

First line: A base number (e.g., 2)
Second line: An exponent (e.g., 3)
Output:

The result of the power calculation
*/ 

use std::io;

fn main() {
    // Read the first input (base number)
    let mut base_input = String::new();
    io::stdin().read_line(&mut base_input).expect("Failed to read line");
    let base: i32 = base_input.trim().parse().expect("Invalid number");
    
    // Read the second input (exponent)
    let mut exp_input = String::new();
    io::stdin().read_line(&mut exp_input).expect("Failed to read line");
    let exponent: u32 = exp_input.trim().parse().expect("Invalid number");
    
    // TODO: Create a closure that calculates the power and store it in a variable
    let power = |x: i32, y: u32| x.pow(y);

    // TODO: Call the closure with base and exponent
    let result = power(base, exponent);
    
    // Print the result
    println!("{}", result);
}