/*
Providing a Default: unwrap_or



While .unwrap() and .expect() can extract values from an Option, they both have a major flaw: they panic when encountering a None. For safer code, Rust provides .unwrap_or() - a method that never panics.

The .unwrap_or() method takes a default value as an argument. If the Option contains a Some, it returns the inner value. If it's a None, it returns the default value you provided:

let maybe_score = Some(85);
let score = maybe_score.unwrap_or(0);
println!("Score: {}", score); // Prints: Score: 85

let no_score: Option = None;
let default_score = no_score.unwrap_or(0);
println!("Score: {}", default_score); // Prints: Score: 0
This approach is much safer than .unwrap() because your program will never crash - you always get a usable value back.

challenge icon
Challenge

Easy
You will receive two inputs. The first input is a score (as a number), and the second input indicates whether the score is available (yes) or missing (no). Create an Option<i32> based on the availability status. Use .unwrap_or() to safely extract the score with a default value of 50, then print the final score.

Requirements:

Read the first input (score) and trim whitespace
Parse the first input to i32
Read the second input (availability status) and trim whitespace
Create an Option<i32> variable:
If the availability status is yes, assign Some(score)
If the availability status is no, assign None
Use .unwrap_or(50) to extract the score with a default value of 50
Print the final score in the format: Final score: [score]
Input:

First line: A number representing the score (e.g., 85)
Second line: Either yes or no
Output:

If the availability status is yes: Final score: [score] (the actual score)
If the availability status is no: Final score: 50 (the default value)
*/ 

use std::io;

fn main() {
    // Read the score
    let mut score_input = String::new();
    io::stdin().read_line(&mut score_input).expect("Failed to read line");
    let score: i32 = score_input.trim().parse().expect("Invalid number");
    
    // Read the availability status
    let mut availability = String::new();
    io::stdin().read_line(&mut availability).expect("Failed to read line");
    let availability = availability.trim();
    
    // TODO: Write your code below
    // Create an Option<i32> based on availability status
    let maybe_available: Option<i32> = if availability == "yes" {
        Some(score)
    } else {
        None
    };

    // Use .unwrap_or(50) to get the final score
    let final_score = maybe_available.unwrap_or(50);
    
    // Print the result
    println!("Final score: {}", final_score);
}