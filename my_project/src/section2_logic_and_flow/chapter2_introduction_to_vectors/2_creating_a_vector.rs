/*
Creating a Vector



Now that you understand what vectors are, let's learn how to create them. Rust provides two primary ways to create a vector, each suited for different situations.

The first method uses Vec::new() to create an empty vector that you can add elements to later:

let mut numbers = Vec::new();
Notice that we use mut because we'll likely want to add elements to this empty vector later. Rust can't always determine the type of an empty vector, so you might need to specify it explicitly:

let mut numbers: Vec<i32> = Vec::new();
The second method uses the vec![] macro to create a vector with initial values:

let numbers = vec![1, 2, 3, 4, 5];
The vec![] macro is more convenient when you know the starting values. Rust can automatically determine the type from the values you provide, so no type annotation is needed. This vector contains five integers and is ready to use immediately.

The vec![] macro also supports a repeat syntax to create a vector filled with the same value a specific number of times:

let zeros = vec![0; 5]; // [0, 0, 0, 0, 0]
The syntax is vec![value; n], where value is the element to repeat and n is how many times to repeat it. This is useful when you need a vector of a known size pre-filled with a default value.

Both approaches create the same type of vector - the choice depends on whether you have initial values or plan to build the vector gradually.
*/ 

use std::io;

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid input");
    
    // TODO: Write your code below
    // Create a vector with n elements, each having the value 10
    let numbers = vec![10; n];
    
    // Print each element on a separate line
    for num in &numbers {
        println!("{}", num);
    }
}