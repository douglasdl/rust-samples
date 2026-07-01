/*
Recap - Basic Score Tracker

challenge icon
Challenge

Easy
You will receive two inputs: first, a comma-separated list of test scores, and second, a score to add. Create a vector from the initial scores, add the new score, calculate the average of all scores, and print the results.

Requirements:

Read the first input containing comma-separated scores (e.g., 85,92,78)
Split the string by commas and trim each individual token before converting to an integer (e.g., use .trim() on each element after splitting)
Create a mutable vector from these scores
Read the second input and convert it to an integer (trim it first)
Use .push() to add the new score to the vector
Calculate the average of all scores in the vector
Print the total number of scores
Print the average score (as an integer)
Input:

First line: Comma-separated integers representing initial scores (e.g., 85,92,78)
Second line: An integer representing the new score to add
Output:

First line: The total number of scores
Second line: The average score (as an integer, truncated)
Note: Always call .trim() on the entire input line and on each individual score token after splitting by comma. Input strings may contain invisible whitespace or newline characters that will cause a panic when parsing if not removed.
*/ 

use std::io;

fn main() {
    // Read the comma-separated scores
    let mut scores_input = String::new();
    io::stdin().read_line(&mut scores_input).expect("Failed to read line");
    
    // Read the new score to add
    let mut new_score_input = String::new();
    io::stdin().read_line(&mut new_score_input).expect("Failed to read line");
    
    // TODO: Write your code below
    // 1. Split scores_input by commas and convert to integers
    let split_scores = scores_input.trim().split(',');
    // 2. Create a mutable vector from these scores
    let mut scores: Vec<i32> = split_scores
        .map(|s| s.trim().parse().unwrap())
        .collect();
    // 3. Convert new_score_input to an integer
    let new_score: i32 = new_score_input.trim().parse().unwrap();
    // 4. Push the new score to the vector
    scores.push(new_score);
    // 5. Calculate the average of all scores
    let mut sum = 0;
    for &score in &scores {
        sum += score;
    }
    
    // Print the total number of scores
    let total_scores = scores.len();
    println!("{}", total_scores);

    // Print the average score
    let mut average = sum / total_scores as i32;
    println!("{}", average);
}