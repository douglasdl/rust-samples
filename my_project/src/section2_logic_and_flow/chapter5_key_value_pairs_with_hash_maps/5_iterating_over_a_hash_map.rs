/*
Iterating Over a Hash Map



Sometimes you need to examine every key-value pair in your hash map, rather than looking up specific keys. Rust provides a simple way to loop through all the data using a for loop:

for (key, value) in &my_map {
    println!("{}: {}", key, value);
}
Notice the & before my_map - this creates a reference to the hash map so you can iterate without taking ownership. The loop gives you each key-value pair as a tuple that you can destructure directly in the loop declaration.

Here's a practical example with a price list:

let mut prices = HashMap::new();
prices.insert("apple", 1.20);
prices.insert("banana", 0.80);

for (item, price) in &prices {
    println!("{} costs ${:.2}", item, price);
}
There's one important detail to remember: hash maps don't guarantee any specific iteration order. The pairs might come out in a different sequence each time you run your program. This is because hash maps prioritize fast lookups over maintaining insertion order, so don't rely on the items appearing in any particular sequence when you iterate.

challenge icon
Challenge

Easy
You will receive an integer n indicating the number of student-score pairs to process. Then you will receive n pairs of inputs: a student name followed by their test score (as an integer). Create a hash map to store the student names as keys and their scores as values. After inserting all pairs, iterate over the hash map and print each student's name and score.

Requirements:

Import HashMap from std::collections
Create a mutable hash map with types HashMap<String, i32>
Read the first input and convert it to i32 to get the number of pairs
Use a loop to read n pairs of inputs (student name, then score)
Insert each student name and score into the hash map
Use a for loop to iterate over the hash map with &map
Print each student's information in the format: [name]: [score]
Print the pairs in any order (hash maps don't guarantee order)
Input:

First line: An integer n (e.g., 3)
Next n pairs of lines:
Student name (e.g., Alice)
Test score as an integer (e.g., 95)
Output:

One line for each student in the format: [name]: [score]
The order of output lines may vary between test runs

Hints

Hint 1
Revealed
Toggle hint
To read multiple inputs inside a loop, you can't use a simple one-time read — you need a way to call next() on an iterator of lines. The starter code already sets up lines as a line iterator from stdin.lock(). Each call to lines.next() reads the next line from input, so you can call it repeatedly inside your loop.
Hint 2
Revealed
Toggle hint
Each call to lines.next() returns an Option wrapping a Result — that's why you need .unwrap().unwrap() twice: the first .unwrap() handles the Option (in case there are no more lines), and the second handles the Result (in case reading failed). After both unwraps you have a String you can work with.
Hint 3
Revealed
Toggle hint
Inside your loop, read the student name and score on two separate lines.next().unwrap().unwrap() calls — one for each line of input. Use .trim().to_string() to clean up the name, and .trim().parse().unwrap() to convert the score to i32. Then insert both into the hash map using map.insert(name, score).
Hint 4
Revealed
Toggle hint
Your loop structure should look like this: for each of the n iterations, call lines.next().unwrap().unwrap().trim().to_string() to get the name, then call lines.next().unwrap().unwrap().trim().parse().unwrap() to get the score as i32, then insert the pair into the hash map. To iterate and print afterwards, use for (name, score) in &students and print with println!("{}: {}", name, score).
*/ 

use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the number of student-score pairs
    let n: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Create a mutable hash map to store student names and scores
    let mut students: HashMap<String, i32> = HashMap::new();
    
    // TODO: Write your code below
    // Read n pairs of inputs (student name and score) and insert them into the hash map
    for _ in 0..n {
        let name = lines.next().unwrap().unwrap().trim().to_string();
        let score: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();

        students.insert(name, score);
    }
    
    // TODO: Iterate over the hash map and print each student's name and score
    for (name, score) in &students {
        println!("{}: {}", name, score);
    }
}