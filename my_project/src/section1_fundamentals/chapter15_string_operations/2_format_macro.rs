/*
format! Macro



In Rust, the format! macro allows you to combine text with variables in a readable and customizable way. This macro is similar to println!, but instead of printing the output to the console, it returns a formatted string that you can store in a variable or use in other operations.

The simplest syntax uses unnamed placeholders {}:

let name = "Alice";
let age = 30;

let formatted = format!("Name: {}, Age: {}", name, age);
println!("{}", formatted);
// Output: Name: Alice, Age: 30
You can use variable names directly in the placeholders:

let name = "Bob";
let age = 25;

let formatted = format!("Name: {name}, Age: {age}");
println!("{}", formatted);
// Output: Name: Bob, Age: 25
You can specify the position of arguments using numbers:

let formatted = format!("{1} {0} {2}", "first", "second", "third");
println!("{}", formatted);
// Output: second first third
You can control how values are formatted using format specifiers:

// Number formatting
let pi = 3.1415926;
let formatted_float = format!("{:.2}", pi);
// "3.14"

// Padding with zeros
let padded = format!("{:0>5}", "123");
// "00123"
challenge icon
Challenge

Easy
Create a program that generates a secret code from a name and birth year using these exact rules:

Takes the first letter of the name and makes it uppercase
Reverses the year digits
Wraps the first letter with "⭐" symbols
Adds a "-" between the wrapped letter and reversed year
To reverse a string use the following code: .to_string().chars().rev().collect::<String>()
*/ 

use std::io;


fn main() {
    let mut input_name = String::new();
    let mut input_year = String::new();
    io::stdin().read_line(&mut input_name).unwrap();
    io::stdin().read_line(&mut input_year).unwrap();
    let name = input_name.trim();
    let year: i32 = input_year.trim().parse().unwrap();

    // Write your code below
    let first_letter_uppercase = name.chars().nth(0).unwrap().to_uppercase();
    let mut reverse_year = year.to_string().chars().rev().collect::<String>();   
    
    let secret_code = format!("⭐{first_letter_uppercase}⭐-{reverse_year}");
    println!("{}", secret_code);




}