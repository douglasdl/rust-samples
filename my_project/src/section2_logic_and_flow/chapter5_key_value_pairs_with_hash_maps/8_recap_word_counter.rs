/*
Recap - Word Counter

challenge icon
Challenge

Easy
You will receive a sentence as input. Count how many times each word appears in the sentence and store the counts in a hash map. Then print each unique word along with its count.

Requirements:

Import HashMap from std::collections
Create a mutable hash map with types HashMap<String, i32>
Read the input sentence as a single line
Split the sentence into individual words using .split_whitespace()
For each word, use .entry() with .or_insert(0) to initialize the count to 0 if the word doesn't exist
After using .or_insert(0), increment the count by 1 for that word
After processing all words, iterate over the hash map and print each word with its count in the format: [word]: [count]
Input:

A single line containing a sentence with words separated by spaces (e.g., hello world hello rust world hello)
Output:

One line for each unique word in the format: [word]: [count]
The order of output lines may vary between test runs
*/ 

use std::collections::HashMap;
use std::io;

fn main() {
    // Read input sentence
    let mut sentence = String::new();
    io::stdin().read_line(&mut sentence).expect("Failed to read line");
    let sentence = sentence.trim();
    
    // Create a mutable hash map to store word counts
    let mut word_count: HashMap<String, i32> = HashMap::new();
    
    // TODO: Write your code below
    // Split the sentence into words and count each word
    for word in sentence.split_whitespace() {
        *word_count.entry(word.to_string()).or_insert(0) += 1;
    }

    // Print each word with its count
    for (word, count) in &word_count {
        println!("{}: {}", word, count);
    }
}