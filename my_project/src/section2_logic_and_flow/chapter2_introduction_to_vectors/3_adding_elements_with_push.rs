/*
Adding Elements with push



Once you have a vector, you'll often want to add new elements to it. Rust provides the .push() method to add elements to the end of a vector. This is one of the most common operations you'll perform with vectors.

Here's how to use .push():

let mut numbers = Vec::new();
numbers.push(10);
numbers.push(20);
numbers.push(30);
Notice the mut keyword - this is crucial! Since adding elements changes the vector's contents, the vector must be declared as mutable. Without mut, Rust won't allow you to modify the vector.

The .push() method always adds the new element to the end of the vector. Each call to .push() increases the vector's length by one:

let mut fruits = Vec::new();
fruits.push("apple");
fruits.push("banana");
// fruits now contains ["apple", "banana"]
You can also start with a vector that already has elements and continue adding to it:

let mut scores = vec![85, 92];
scores.push(78); // scores is now [85, 92, 78]
*/ 

use std::io;

fn main() {
    // Read three numbers from input
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let num1: i32 = input1.trim().parse().expect("Invalid input");
    
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let num2: i32 = input2.trim().parse().expect("Invalid input");
    
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Failed to read line");
    let num3: i32 = input3.trim().parse().expect("Invalid input");
    
    // TODO: Write your code below
    // Create a mutable vector and push the numbers to it
    let mut numbers = Vec::new();
    
    // Print each element of the vector on a separate line
    numbers.push(num1);
    numbers.push(num2);
    numbers.push(num3);

    for num in &numbers {
        println!("{}", num);
    }
}