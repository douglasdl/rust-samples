/*
Other Slices



While string slices are very useful, slicing isn't limited to just strings. Rust's slice concept extends to any sequence of elements, making it a powerful tool for working with collections without taking ownership.

You can create slices from arrays and vectors using the same syntax you learned for strings. For example, if you have an array of numbers, you can create a slice that references part or all of it:

let numbers = [1, 2, 3, 4, 5];
let slice: &[i32] = &numbers[1..4]; // This gives us [2, 3, 4]
let full_slice: &[i32] = &numbers;  // This references the entire array
The same principle works with vectors. You can create slices from vectors to work with portions of the data without moving or copying it:

let mut vec = vec![10, 20, 30, 40];
let vec_slice: &[i32] = &vec[0..2]; // This gives us [10, 20]

Challenge

Easy
You will receive three inputs. The first input is a comma-separated list of numbers. The second input is two numbers separated by a space representing the start and end indices for creating a slice. The third input is a single number representing an index to access from the slice.

Create a vector from the comma-separated numbers, then create a slice using the specified range. Access the element at the given index from the slice and print it.

Requirements:

Read the first input (comma-separated numbers) and trim whitespace
Split the input by commas and parse each number to i32
Create a vector from these numbers
Read the second input (two numbers separated by a space) and trim whitespace
Split the second input by space to get the start and end indices
Parse both indices to usize
Create a slice from the vector using the range syntax &vec[start..end]
Read the third input (index to access) and trim whitespace
Parse the index to usize
Access the element at the specified index from the slice
Print the accessed element
Input:

First line: Comma-separated numbers (e.g., 10,20,30,40,50)
Second line: Two numbers separated by a space (e.g., 1 4)
Third line: An index number (e.g., 2)
Output:

The element at the specified index from the slice
*/ 

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the first input (comma-separated numbers)
    let first_input = lines.next().unwrap().unwrap();
    
    // Read the second input (start and end indices)
    let second_input = lines.next().unwrap().unwrap();
    
    // Read the third input (index to access)
    let third_input = lines.next().unwrap().unwrap();
    
    // TODO: Write your code below
    // Parse the comma-separated numbers and create a vector
    let vec: Vec<i32> = first_input
    .trim()
    .split(',')
    .map(|n| n.trim().parse().unwrap())
    .collect();

    let parts: Vec<&str> = second_input.trim().split_whitespace().collect();

    // Parse the start and end indices
    let start: usize = parts[0].parse().unwrap();
    let end: usize = parts[1].parse().unwrap();
    
    // Create a slice from the vector
    let slice: &[i32] = &vec[start..end];
    
    // Access the element at the specified index from the slice
    let index: usize = third_input.trim().parse().unwrap();
    let result = slice[index];
    
    // Print the result
    println!("{}", result);
}