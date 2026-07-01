/*
Accessing Vector Elements



Once you have a vector with elements, you'll need to access individual values. Rust provides two different approaches for getting elements from a vector, each with its own trade-offs between convenience and safety.

The first method uses index syntax with square brackets, similar to arrays:

let numbers = vec![10, 20, 30, 40];
let first = numbers[0]; // gets 10
let third = numbers[2]; // gets 30
While this syntax is familiar and concise, it has a significant risk: if you try to access an index that doesn't exist, your program will panic and crash:

let numbers = vec![10, 20, 30];
let invalid = numbers[5]; // This will panic!
The second method uses the .get() method, which provides safe access:

let numbers = vec![10, 20, 30, 40];
let first = numbers.get(0); // returns Some(10)
let invalid = numbers.get(5); // returns None
The .get() method returns an Option - either Some(value) if the index exists, or None if it doesn't. This forces you to handle the possibility that the element might not exist, preventing crashes and making your code more robust.

challenge icon
Challenge

Easy
Write a function get_element_at that takes a vector numbers and an index idx, and returns the element at that index using safe access.
Use the .get() method to safely access the element. If the index exists, return the value. If the index doesn't exist, return -1 as a default value.

Parameters:

numbers (Vec<i32>): The vector to access
idx (i32): The index to retrieve
Returns: The element at the given index, or -1 if the index doesn't exist (i32)
*/ 

fn get_element_at(numbers: Vec<i32>, idx: i32) -> i32 {
  // Write code here
  if let Some(value) = numbers.get(idx as usize) {
      *value
  } else {
      -1
  }
}
