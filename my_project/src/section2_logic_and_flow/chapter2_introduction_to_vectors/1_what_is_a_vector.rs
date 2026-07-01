/*
What is a Vector?



In Rust, you have two main ways to store collections of data: arrays and vectors. While arrays have a fixed size that must be known at compile time, vectors are dynamic collections that can grow and shrink during program execution.

A vector, written as Vec<T>, is Rust's growable array type. The T represents the type of elements the vector will hold - for example, Vec<i32> for integers or Vec<String> for strings. Unlike arrays where you must specify the exact number of elements upfront, vectors can start empty and have elements added or removed as your program runs.

Here's when you'd choose a vector over an array:

// Array - fixed size, known at compile time
let scores = [85, 92, 78, 90]; // always exactly 4 elements

// Vector - dynamic size, can change during runtime
let mut grades = Vec::new(); // starts empty, can grow
Vectors are perfect when you don't know how many items you'll need in advance, such as storing user input, reading data from files, or building lists that change based on program logic. They provide the flexibility that arrays cannot offer while maintaining efficient access to elements.
*/ 