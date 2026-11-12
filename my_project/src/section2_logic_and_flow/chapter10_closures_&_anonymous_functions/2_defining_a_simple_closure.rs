/*
Defining a Simple Closure



Now that you understand what closures are, let's learn how to define and use them in your code. The simplest way to create a closure is to assign it to a variable, which allows you to call it later.

Here's how you define a basic closure that takes no parameters:

let my_closure = || {
    println!("Hello from the closure!");
};
Notice the empty vertical bars || - this indicates that the closure takes no arguments. The curly braces contain the code that runs when you call the closure.

To call your closure, you use it just like a function by adding parentheses after the variable name:

let my_closure = || {
    println!("Hello from the closure!");
};

my_closure(); // This calls the closure and prints the message


Defining a Simple Closure


challenge icon
Challenge

Easy
You will receive a single input containing a message. Create a closure that takes no parameters and prints this message. Store the closure in a variable, then call it to display the message.

Requirements:

Read the input message and trim whitespace
Create a closure using || syntax that prints the message
Store the closure in a variable
Call the closure to execute it
Input:

A single line containing a message (e.g., Welcome to Rust!)
Output:

The message printed by the closure


*/ 

use std::io;

fn main() {
    // Read input
    let mut message = String::new();
    io::stdin().read_line(&mut message).expect("Failed to read line");
    let message = message.trim();
    
    // TODO: Create a closure that prints the message and store it in a variable
    let my_closure = || {
        println!("{}", message);
    };
    
    
    // TODO: Call the closure to print the message
    my_closure();
}