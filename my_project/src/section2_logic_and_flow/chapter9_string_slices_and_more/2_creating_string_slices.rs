/*
Creating String Slices



Now that you understand the difference between String and &str, let's learn how to create string slices from existing strings. There are two main ways to create a &str from a String.

The simplest way is to use the reference operator & with your String. This creates a slice that references the entire string:

let my_string = String::from("Hello World");
let slice: &str = &my_string; // This creates a slice of the entire string
You can also create slices of specific parts of a string using range syntax. The format is &string[start..end], where start is the beginning index and end is one past the last character you want to include:

let my_string = String::from("Hello World");
let first_word = &my_string[0..5]; // This gives us "Hello"
let second_word = &my_string[6..11]; // This gives us "World"
The indices used in range syntax are of type usize, which is Rust's unsigned integer type for representing sizes and indices. It's the standard type used whenever you need to index into a collection or specify a length. When parsing index values from input, you'll use .parse::<usize>() to convert them to the correct type.


Challenge

Easy
You will receive two inputs. The first input is a full sentence, and the second input is two numbers separated by a space representing the start and end indices for creating a slice. Create a String from the first input, then create a string slice using the range specified by the two indices. Print the original string on the first line and the extracted slice on the second line.

Requirements:

Read the first input (the full sentence) and trim whitespace
Create a String from this input
Read the second input (two numbers separated by a space) and trim whitespace
Split the second input by space to get the start and end indices
Parse both indices to usize
Create a string slice using the range syntax &string[start..end]
Print the original string
Print the extracted slice
Input:

First line: A sentence (e.g., Hello World)
Second line: Two numbers separated by a space (e.g., 0 5)
Output:

First line: The original string
Second line: The extracted slice
*/ 

use std::io;

fn main() {
    // Read the first input (the full sentence)
    let mut sentence = String::new();
    io::stdin().read_line(&mut sentence).expect("Failed to read line");
    let sentence = sentence.trim();
    
    // Read the second input (two numbers separated by a space)
    let mut indices = String::new();
    io::stdin().read_line(&mut indices).expect("Failed to read line");
    let indices = indices.trim();
    
    // TODO: Write your code below
    // Parse the indices and create the string slice
    let parts: Vec<&str> = indices.split_whitespace().collect();
    let start: usize = parts[0].parse().unwrap();
    let end: usize = parts[1].parse().unwrap();

    let string_slice = &sentence[start..end];
    
    
    // Print the original string and the slice
    println!("{}", sentence);
    println!("{}", string_slice);
}