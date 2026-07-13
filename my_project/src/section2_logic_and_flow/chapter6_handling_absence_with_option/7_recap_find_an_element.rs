/*
Recap - Find an Element

challenge icon
Challenge

Easy
You will receive two inputs. The first input is a list of numbers separated by commas (e.g., 10,20,30,40), and the second input is a target number to search for. Parse the comma-separated numbers into a vector, then search for the target number. Use .iter().position() to find the index of the target number, which returns an Option<usize>. Handle both cases using match and print the appropriate message.

Requirements:

Read the first input (comma-separated numbers) and trim whitespace
Split the input by commas and parse each part into i32 to create a vector
Read the second input (target number) and trim whitespace
Parse the target number to i32
Use .iter().position(|&x| x == target) to search for the target in the vector
Use a match expression to handle the Option returned by .position()
In the Some(index) arm, print: Found at index [index]
In the None arm, print: Not found
Input:

First line: Comma-separated numbers (e.g., 10,20,30,40)
Second line: A target number to search for (e.g., 30)
Output:

If the target is found: Found at index [index]
If the target is not found: Not found
*/ 

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the comma-separated numbers
    let numbers_input = lines.next().unwrap().unwrap().trim().to_string();
    
    // Read the target number
    let target_input = lines.next().unwrap().unwrap().trim().to_string();
    
    // Parse the comma-separated numbers into a vector
    let numbers: Vec<i32> = numbers_input
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();
    
    // Parse the target number
    let target: i32 = target_input.parse().unwrap();
    
    // TODO: Write your code below
    // Use .iter().position() to find the target and match to handle the result
    match numbers.iter().position(|&x| x == target) {
        Some(index) => println!("Found at index {}", index),
        None => println!("Not found"),
    }
}