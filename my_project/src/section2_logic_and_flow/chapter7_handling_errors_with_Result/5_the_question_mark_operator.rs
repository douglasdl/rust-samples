/*
The Question Mark Operator '?'



When working with functions that return Result, you often need to handle errors by either dealing with them locally or passing them up to the calling function. Rust provides an elegant operator for this common pattern: the question mark operator ?.

The ? operator provides a clean way to propagate errors without writing verbose match statements. When you place ? after a Result, it automatically unwraps an Ok value or immediately returns from the current function with the Err:

fn parse_and_double(s: &str) -> Result<i32, std::num::ParseIntError> {
    let number = s.parse::<i32>()?; // If parsing fails, return the error
    Ok(number * 2) // If successful, double the number
}
This is equivalent to writing a full match statement, but much more concise. If s.parse::<i32>() returns an Ok, the ? extracts the value and assigns it to number. If it returns an Err, the ? immediately returns that error from the entire function.

Note the ::<i32> syntax. This is called a turbofish, and it tells Rust which type to parse the string into. The function returns Result<i32, std::num::ParseIntError>, where i32 is the success type and std::num::ParseIntError is the error type returned when parsing fails.

The ? operator can only be used in functions that return a Result (or Option), since it needs to be able to return an error.

challenge icon
Challenge

Easy
You will receive two inputs. The first input is a string that should be parsed into an integer, and the second input is another string that should also be parsed into an integer. Create a function called add_numbers that takes two string slices as parameters and returns a Result<i32, std::num::ParseIntError>. Inside the function, use the ? operator to parse both strings and return their sum wrapped in Ok. Call the function with the two inputs and handle the result using match.

Requirements:

Read the first input (first number as string) and trim whitespace
Read the second input (second number as string) and trim whitespace
Create a function add_numbers that takes two &str parameters and returns Result<i32, std::num::ParseIntError>
Inside the function, parse the first string to i32 using the ? operator
Parse the second string to i32 using the ? operator
Return Ok(sum) where sum is the addition of both parsed numbers
Call the function with the two input strings
Use a match expression to handle the Result
In the Ok(value) arm, print: Sum: [value]
In the Err(_) arm, print: Parsing error
Input:

First line: A string representing a number (e.g., 15)
Second line: A string representing another number (e.g., 27)
Output:

If both strings are valid numbers: Sum: [result]
If either string cannot be parsed: Parsing error
*/ 

use std::io;

// TODO: Crie a função add_numbers aqui
// Ela deve receber dois parâmetros &str e retornar Result&lt;i32, std::num::ParseIntError&gt;
fn add_numbers(n1: &str, n2: &str) -> Result<i32, std::num::ParseIntError> {
    let number1 = n1.parse::<i32>()?;
    let number2 = n2.parse::<i32>()?;
    Ok(number1 + number2)
}

fn main() {
    // Lê a primeira entrada
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let input1 = input1.trim();
    
    // Lê a segunda entrada
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let input2 = input2.trim();
    
    // TODO: Chame add_numbers com input1 e input2
    let result = add_numbers(input1, input2);
    // TODO: Use match para tratar o Result
    match result {
        Ok(value) => println!("Sum: {}", value),
        Err(_) => println!("Parsing error"),
    }
    // Imprima "Sum: [value]" para Ok(value)
    // Print "Parsing error" for Err(_)
}