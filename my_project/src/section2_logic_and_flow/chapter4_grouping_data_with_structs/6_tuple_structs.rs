/*
Tuple Structs



While regular structs use named fields, Rust offers another type called tuple structs. These provide a more concise way to create custom types when you don't need to name each field individually.

A tuple struct combines the benefits of creating a new type with the simplicity of a tuple. You define it using the struct keyword followed by the type name and the field types in parentheses:

struct Color(i32, i32, i32);
This creates a Color type that holds three i32 values, representing RGB color values. Unlike regular structs, you don't specify field names - you access the values by their position, just like with tuples.

To create an instance of a tuple struct, you provide values in parentheses:

let red = Color(255, 0, 0);
You can access individual fields using dot notation with the field index: red.0 gives you the first value, red.1 the second, and so on.
*/ 

use std::io;

// TODO: Define your Color tuple struct here
struct Color(i32, i32, i32);

fn main() {
    // Read red value
    let mut red_input = String::new();
    io::stdin().read_line(&mut red_input).expect("Failed to read line");
    let red: i32 = red_input.trim().parse().expect("Invalid input");
    
    // Read green value
    let mut green_input = String::new();
    io::stdin().read_line(&mut green_input).expect("Failed to read line");
    let green: i32 = green_input.trim().parse().expect("Invalid input");
    
    // Read blue value
    let mut blue_input = String::new();
    io::stdin().read_line(&mut blue_input).expect("Failed to read line");
    let blue: i32 = blue_input.trim().parse().expect("Invalid input");
    
    // TODO: Create an instance of Color tuple struct using the input values
    let color = Color(red, green, blue);

    // TODO: Access and print each color component using index notation
    // Format: Red: [value], Green: [value], Blue: [value]
    println!("Red: {}", color.0);
    println!("Green: {}", color.1);
    println!("Blue: {}", color.2);
}