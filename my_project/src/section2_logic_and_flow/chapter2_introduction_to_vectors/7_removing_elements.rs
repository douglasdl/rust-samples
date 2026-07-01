/*
Removing Elements



Sometimes you need to remove elements from a vector, not just add them. Rust provides two main methods for removing elements, each designed for different situations.

The .pop() method removes and returns the last element from the vector:

let mut numbers = vec![10, 20, 30, 40];
let last = numbers.pop(); // removes 40, returns Some(40)
// numbers is now [10, 20, 30]
Notice that .pop() returns an Option - it gives you Some(value) if there was an element to remove, or None if the vector was already empty. This makes it safe to use even on empty vectors.

The .remove() method removes an element at a specific index and returns that element:

let mut fruits = vec!["apple", "banana", "cherry"];
let removed = fruits.remove(1); // removes "banana"
// fruits is now ["apple", "cherry"]
When you use .remove(), all elements after the removed index shift left to fill the gap. This operation can be slower for large vectors when removing from the beginning or middle, since many elements need to be moved.

Both methods require the vector to be mutable since they modify its contents. Choose .pop() when you want to remove from the end, and .remove() when you need to remove from a specific position.

challenge icon
Challenge

Easy
You will receive two inputs: first, a comma-separated list of numbers, and second, an index to remove. Read both inputs, create a vector from the numbers, remove the element at the specified index, and print the remaining elements on separate lines.

Requirements:

Read the first input containing comma-separated numbers (e.g., 10,20,30,40,50)
Split the string by commas and convert each number to an integer
Create a mutable vector from these numbers
Read the second input and convert it to an integer representing the index
Use .remove() to remove the element at that index
Print each remaining element on a separate line
Input:

First line: Comma-separated integers (e.g., 10,20,30,40,50)
Second line: An integer representing the index to remove
Output: Print each remaining element of the vector on a separate line


Hints icon
Hints

Hint 1
Revealed
Toggle hint
This challenge requires parsing a comma-separated string into a vector of integers. Start by using .trim() on your input string to remove any trailing newline, then use .split(',') to break it into individual number strings.
Hint 2
Revealed
Toggle hint
To convert each piece of the split string into an integer, chain .map(|s| s.trim().parse().unwrap()) after your .split(',') call. This applies a small function to each element, trimming whitespace and parsing it as a number. Finally, chain .collect() to gather the results into a vector.
Hint 3
Revealed
Toggle hint
When declaring your vector, Rust needs to know what type it holds. Write let mut numbers: Vec<i32> = ... so the compiler knows you want a vector of 32-bit integers. Without the <i32> type annotation, Rust may not be able to infer the type and will give a compile error.
Hint 4
Revealed
Toggle hint
For the index input, remember to parse it as usize (not i32), since .remove() expects a usize argument. Use let index: usize = index_input.trim().parse().unwrap();, then call numbers.remove(index); to remove the element at that position.
*/ 

use std::io;

fn main() {
    // Read the comma-separated numbers
    let mut numbers_input = String::new();
    io::stdin().read_line(&mut numbers_input).expect("Failed to read line");
    
    // Read the index to remove
    let mut index_input = String::new();
    io::stdin().read_line(&mut index_input).expect("Failed to read line");
    
    // TODO: Write your code below
    // 1. Split the numbers_input by commas and convert to integers
    let split_numbers = numbers_input.trim().split(',');
    // 2. Create a mutable vector from these numbers
    let mut numbers: Vec<i32> = split_numbers
        .map(|s| s.trim().parse().unwrap())
        .collect();
    // 3. Convert index_input to an integer
    let index: usize = index_input.trim().parse().unwrap();
    // 4. Remove the element at the specified index
    numbers.remove(index);
    // 5. Print each remaining element on a separate line
    for number in numbers {
        println!("{}", number);
    }
}