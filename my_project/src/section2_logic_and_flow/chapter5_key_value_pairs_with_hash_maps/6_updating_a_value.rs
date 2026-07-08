/*
Updating a Value



Sometimes you need to update values in a hash map in different ways. You've already learned that .insert() will overwrite an existing value, but what if you only want to add a value when the key doesn't already exist?

Rust provides the .entry() API for more sophisticated value updates. The most useful method in this API is .or_insert(), which only inserts a value if the key is not already present:

let mut scores = HashMap::new();
scores.insert("Alice", 85);

// This will NOT overwrite Alice's score
scores.entry("Alice").or_insert(90);

// This WILL add Bob's score since he's not in the map
scores.entry("Bob").or_insert(90);
The .entry() method returns an Entry enum that represents either an occupied or vacant spot in the hash map. When you call .or_insert() on it, it only inserts the new value if the entry is vacant (the key doesn't exist).
*/ 

use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the number of player names
    let n: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Create a mutable HashMap to store player scores
    let mut player_scores: HashMap<String, i32> = HashMap::new();
    
    // TODO: Write your code below
    // Read n player names and use .entry().or_insert(100) to add them to the map
    for _ in 0..n {
        let name = lines.next().unwrap().unwrap().trim().to_string();
        
        player_scores.entry(name).or_insert(100);
    }
    
    
    // Print each player and their score in the format: [name]: [score]
    for (name, score) in &player_scores {
        println!("{}: {}", name, score);
    }
}