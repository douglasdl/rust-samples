/*
Iterating Over a Vector



Now that you know how to create vectors and access individual elements, let's learn how to process every element in a vector systematically. The for loop is perfect for this task, allowing you to visit each element in sequence.

Here's the basic syntax for iterating over a vector:

let names = vec!["Alice", "Bob", "Charlie"];

for name in &names {
    println!("Hello, {}!", name);
}
Notice the & before names - this creates a reference to the vector, allowing you to read each element without taking ownership of the vector itself. Each iteration gives you access to one element at a time through the loop variable name.

The for loop automatically handles the bounds checking for you. Unlike manual indexing, you'll never accidentally try to access an element that doesn't exist. The loop will visit exactly as many elements as the vector contains, making it both safe and convenient.

You can use this pattern with any type of vector:

let scores = vec![85, 92, 78, 90];

for score in &scores {
    println!("Score: {}", score);
}
This approach is the most common and idiomatic way to process all elements in a vector when you need to read their values.

Challenge

Easy
Write a function sum_all_elements that takes a vector numbers and returns the sum of all its elements.
Use a for loop to iterate over the vector and accumulate the sum of all elements.

Parameters:

numbers (Vec<i32>): The vector of integers to sum
Returns: The sum of all elements in the vector (i32)
*/ 

fn sum_all_elements(numbers: Vec<i32>) -> i32 {
  // Write code here
  let mut sum = 0;
  for number in numbers {
      sum += number;
  }
  sum
}
