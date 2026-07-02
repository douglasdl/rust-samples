/*
Structs as Function Parameters



Now that you know how to create and modify structs, let's explore how to use them with functions. Just like any other data type in Rust, you can pass struct instances to functions and return them from functions.

To pass a struct to a function, you simply include it as a parameter in the function definition:

fn calculate_area(rect: Rectangle) -> i32 {
    rect.width * rect.height
}
This function takes a Rectangle struct as a parameter and returns an i32. Inside the function, you can access the struct's fields using dot notation, just like you would anywhere else.

You can also return structs from functions by specifying the struct type as the return type:

fn create_square(size: i32) -> Rectangle {
    Rectangle {
        width: size,
        height: size,
    }
}
Using structs with functions makes your code more organized and meaningful. Instead of passing multiple individual parameters, you can group related data together and pass it as a single, well-defined unit.

Challenge

Easy
You will receive four inputs: a rectangle's width (as an integer), a rectangle's height (as an integer), a square's side length (as an integer), and a choice number (1 or 2). Define a Rectangle struct with two fields: width: i32 and height: i32. Create two functions: one that takes a Rectangle and calculates its area, and another that takes a side length and returns a Rectangle representing a square. Based on the choice input, calculate and print the appropriate area.

Requirements:

Define a Rectangle struct with fields: width: i32 and height: i32
Create a function calculate_area that takes a Rectangle as a parameter and returns an i32 (the area)
Create a function create_square that takes an i32 (side length) as a parameter and returns a Rectangle with equal width and height
Read the first input and convert it to i32 for the rectangle's width
Read the second input and convert it to i32 for the rectangle's height
Create a Rectangle instance with these dimensions
Read the third input and convert it to i32 for the square's side length
Read the fourth input and convert it to i32 for the choice (1 or 2)
If choice is 1: calculate and print the area of the rectangle
If choice is 2: create a square using create_square, then calculate and print its area
Input:

First line: Rectangle width as an integer (e.g., 8)
Second line: Rectangle height as an integer (e.g., 5)
Third line: Square side length as an integer (e.g., 6)
Fourth line: Choice as an integer, either 1 or 2 (e.g., 1)
Output:

If choice is 1: Rectangle area: [area]
If choice is 2: Square area: [area]
*/ 

use std::io;

// TODO: Define the Rectangle struct here
struct Rectangle {
    width: i32,
    height: i32
}

// TODO: Define the calculate_area function here
fn calculate_area(rect: Rectangle) -> i32 {
    rect.width * rect.height
}

// TODO: Define the create_square function here
fn create_square(size: i32) -> Rectangle {
    Rectangle {
        width: size,
        height: size
    }
}

fn main() {
    // Read rectangle width
    let mut width_input = String::new();
    io::stdin().read_line(&mut width_input).expect("Failed to read line");
    let width: i32 = width_input.trim().parse().expect("Invalid input");
    
    // Read rectangle height
    let mut height_input = String::new();
    io::stdin().read_line(&mut height_input).expect("Failed to read line");
    let height: i32 = height_input.trim().parse().expect("Invalid input");
    
    // Read square side length
    let mut side_input = String::new();
    io::stdin().read_line(&mut side_input).expect("Failed to read line");
    let side: i32 = side_input.trim().parse().expect("Invalid input");
    
    // Read choice
    let mut choice_input = String::new();
    io::stdin().read_line(&mut choice_input).expect("Failed to read line");
    let choice: i32 = choice_input.trim().parse().expect("Invalid input");
    
    // TODO: Write your code below
    // Create a Rectangle instance, check the choice, and calculate the appropriate area
    let rectangle = Rectangle {
        width: width,
        height: height
    };

    let square = create_square(side);


    // Output the result based on choice
    // Use: println!("Rectangle area: {}", area); or println!("Square area: {}", area);
    let mut area = 0;
    if(choice == 1) {
        area = calculate_area(rectangle);
        println!("Rectangle area: {}", area);
    } else {
        area = calculate_area(square);
        println!("Square area: {}", area);
    }
    
}