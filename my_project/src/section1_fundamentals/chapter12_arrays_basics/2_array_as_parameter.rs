/*
Array as Parameter



In Rust, you can accept arrays as parameters in the following way:

Fixed-size arrays:

fn process_array(arr: [i32; 5]) {}
This allows you to pass only arrays with the length 5 because of the 5 in [i32; 5].

Array reference using the & sign:

fn process_array(arr: &[i32]) {}
A reference is like a pointer to data instead of the data itself. Using & creates a reference. This allows to pass array of any size.

Use fixed-size arrays ([i32; 5]) when you need to work with arrays of a specific size only.

Use array references (&[i32]) when you want more flexibility - it can accept arrays of any size, and it's more memory efficient.

For example here is how to call a function that accepts a reference array:

let numbers = [1, 2, 3, 4, 5];
process_array(&numbers);
*/ 