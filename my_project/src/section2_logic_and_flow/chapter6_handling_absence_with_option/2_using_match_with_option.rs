/*
Using 'match' with 'Option'



Now that you understand what Option is, let's learn the most common and explicit way to handle it: using a match expression. This approach forces you to consider both possible cases - when there's a value and when there isn't.

The match expression works perfectly with Option because you can create arms for both variants:

let maybe_name = Some("Alice".to_string());

match maybe_name {
    Some(name) => println!("Hello, {}!", name),
    None => println!("Hello, stranger!"),
}
Notice how the Some(name) arm extracts the value from inside the Some variant. The variable name now contains the actual String, not the Option. This is called destructuring - you're pulling the inner value out of the wrapper.

The None arm handles the case where there's no value, allowing you to provide alternative behavior like showing a default message or taking a different action entirely.

Challenge

Easy
You will receive a student name as input. The name might be present in the system or it might be missing. Create an Option<String> that contains Some(name) if the name is not empty, or None if the name is empty. Use a match expression to handle both cases and print the appropriate message.

Requirements:

Read the input as a string (student name)
Trim any whitespace from the input using .trim()
Create an Option<String> variable:
If the trimmed name is not empty, assign Some(name.to_string())
If the trimmed name is empty, assign None
Use a match expression to handle both variants of the Option
In the Some(name) arm, print: Welcome, [name]!
In the None arm, print: No name provided
Input:

A single line containing either a student name or an empty line
Output:

If a name is provided: Welcome, [name]!
If no name is provided (empty input): No name provided
*/ 

use std::io;

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let name = input.trim();
    
    // TODO: Create an Option<String> based on whether the name is empty or not
    let maybe_name = if !name.is_empty() {
        Some(name.to_string())
    } else {
        None
    };

    // TODO: Use a match expression to handle Some and None cases and print the appropriate message
    match maybe_name {
        Some(student_name) => println!("Welcome, {}!", student_name),
        None => println!("No name provided"),
    }
}