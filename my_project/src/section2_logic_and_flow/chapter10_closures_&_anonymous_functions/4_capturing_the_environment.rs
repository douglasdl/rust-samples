/*
Capturing the Environment



One of the most powerful features of closures is their ability to capture and use variables from their surrounding environment. Unlike regular functions, closures can "see" and access variables that exist in the same scope where they're defined.

Here's how environment capture works:

fn main() {
    let x = 10;
    
    let my_closure = || {
        println!("The value of x is: {}", x);
    };
    
    my_closure(); // This will print "The value of x is: 10"
}
In this example, the closure captures the variable x from the surrounding scope and uses it inside its body. The closure doesn't need x to be passed as a parameter - it automatically has access to it.

You can also capture variables and use them in calculations:

let multiplier = 3;
let multiply_by_three = |num| num * multiplier;
let result = multiply_by_three(5); // result is 15

Capturing the Environment


challenge icon
Challenge

Easy
You will receive two inputs. The first input is a number representing a base price. The second input is a number representing a discount percentage. Create a closure that captures the discount percentage from the environment and calculates the final price after applying the discount. Call the closure with the base price and print the final price.

Requirements:

Read the first input (base price) and trim whitespace
Parse it to f64
Read the second input (discount percentage) and trim whitespace
Parse it to f64
Create a closure that takes one parameter (the base price) and captures the discount percentage from the surrounding environment
The closure should calculate the final price using the formula: base_price - (base_price * discount / 100.0)
Call the closure with the base price
Print the final price
Input:

First line: A base price (e.g., 100.0)
Second line: A discount percentage (e.g., 20.0)
Output:

The final price after applying the discount
*/ 

use std::io;

fn main() {
    // Read the base price
    let mut base_price_input = String::new();
    io::stdin().read_line(&mut base_price_input).expect("Failed to read line");
    let base_price: f64 = base_price_input.trim().parse().expect("Invalid number");
    
    // Read the discount percentage
    let mut discount_input = String::new();
    io::stdin().read_line(&mut discount_input).expect("Failed to read line");
    let discount: f64 = discount_input.trim().parse().expect("Invalid number");
    
    // TODO: Create a closure that captures the discount and calculates the final price
    let discount_price = |base_price: f64| {
        base_price - (base_price * discount / 100.0)
    };
    // TODO: Call the closure with the base price
    let final_price = discount_price(base_price);
    
    // Print the final price
    println!("{}", final_price);
}