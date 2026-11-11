/*
Recap - Find First Word

challenge icon
Challenge

Easy
You will receive a single input containing a sentence with multiple words separated by spaces. Find and print the first word from the sentence using string slicing.

Requirements:

Read the input sentence and trim whitespace
Find the position of the first space in the sentence
Create a string slice from the beginning of the sentence up to (but not including) the first space
Print the first word
Hint: You can use the .find() method to locate the first space character. Call it like this: sentence.find(' '), passing a space character as the argument. This method returns an Option that contains the index if a space is found. Use .unwrap_or(sentence.len()) to handle cases where there might be no space (single word), defaulting to the full length of the sentence.

Input:

A sentence with one or more words separated by spaces (e.g., Hello World from Rust)
Output:

The first word from the sentence
*/ 

use std::io::{self, BufRead};

fn main() {
    // Read input
    let stdin = io::stdin();
    let sentence = stdin.lock().lines().next().unwrap().unwrap().trim().to_string();
    
    // TODO: Write your code below
    // Find the position of the first space
    let first_space = sentence.find(' ').unwrap_or(sentence.len());

    // Create a string slice from the beginning to the first space
    // Store the first word in a variable
    let first_word = &sentence[0..first_space];
    
    // Print the first word
    println!("{}", first_word);
}