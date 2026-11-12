/*
Recap - Unique Items

challenge icon
Challenge

Easy
Write a function get_unique_items that takes numbers and returns a vector containing only the unique values from the input.
Use a hash map to track which numbers you've already seen, then build a result vector containing only numbers that appear exactly once in the input.

Logic:

Create a hash map to count occurrences of each number
Iterate through the input vector and update the count for each number
Create a result vector
Iterate through the input vector again and add numbers that have a count of 1 to the result
Return the result vector
Parameters:

numbers (Vec<i32>): A vector of integers that may contain duplicates
Returns: A vector containing only the numbers that appear exactly once in the input, in the order they first appeared (Vec<i32>)
*/ 

use std::collections::HashMap;

fn get_unique_items(numbers: Vec<i32>) -> Vec<i32> {
    // Write code here
    let mut counts = HashMap::new();

    for &number in &numbers {
        *counts.entry(number).or_insert(0) += 1;
    }

    let mut result = Vec::new();

    for number in numbers {
        if counts[&number] == 1 {
            result.push(number);
        }
    }

    result
}
