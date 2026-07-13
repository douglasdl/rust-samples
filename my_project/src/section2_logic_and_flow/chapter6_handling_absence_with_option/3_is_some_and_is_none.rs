/*
is_some() and is_none()



Sometimes you need to check whether an Option contains a value without actually extracting that value. Rust provides two convenient methods for this: .is_some() and .is_none().

Both methods return a boolean value, making them perfect for use in if conditions:

let maybe_score = Some(85);

if maybe_score.is_some() {
    println!("We have a score!");
}

if maybe_score.is_none() {
    println!("No score available");
}
The .is_some() method returns true if the Option contains a Some value, and false if it's None. The .is_none() method works in the opposite way - it returns true for None and false for Some.

Challenge

Easy
You will receive a number as input. Create an Option<i32> that contains Some(number) if the number is positive, or None if the number is zero or negative. Use .is_some() and .is_none() to check the Option and print the appropriate message.

Requirements:

Read the input and convert it to i32
Create an Option<i32> variable:
If the number is positive (greater than 0), assign Some(number)
If the number is zero or negative, assign None
Use .is_some() to check if the Option contains a value
If .is_some() returns true, print: Positive number detected
Use .is_none() to check if the Option is empty
If .is_none() returns true, print: No positive number
Input:

A single integer (e.g., 42, 0, or -5)
Output:

If the number is positive: Positive number detected
If the number is zero or negative: No positive number
*/ 

use std::io;

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let number: i32 = input.trim().parse().expect("Invalid input");
    
    // TODO: Write your code below
    // Create an Option<i32> based on whether the number is positive
    let maybe_number = if number > 0 {
        Some(number)
    } else {
        None
    };

    // Check if the Option is Some and print the appropriate message
    if maybe_number.is_some() {
        println!("Positive number detected");
    }

    if maybe_number.is_none() {
        println!("No positive number");
    }
}