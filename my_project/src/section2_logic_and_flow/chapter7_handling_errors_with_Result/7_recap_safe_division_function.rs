/*
Recap - Safe Division Function

challenge icon
Challenge

Easy
You will receive two inputs. The first input is a number representing the dividend, and the second input is a number representing the divisor. Create a function safe_divide that takes two f64 parameters and returns a Result<f64, &'static str>. The function should return Err("Cannot divide by zero") when the divisor is zero, and Ok with the division result otherwise. Use a match expression to handle the result and print the appropriate output.

Requirements:

Read the first input (dividend) and trim whitespace
Parse the first input to f64
Read the second input (divisor) and trim whitespace
Parse the second input to f64
Create a function safe_divide that takes two f64 parameters and returns Result<f64, &'static str>
Inside the function, check if the divisor is 0.0:
If yes, return Err("Cannot divide by zero")
If no, return Ok(dividend / divisor)
Call the function with the two parsed numbers
Use a match expression to handle the Result
In the Ok(value) arm, print: Division result: [value]
In the Err(error) arm, print: Error: [error]
Input:

First line: A number representing the dividend (e.g., 20.0)
Second line: A number representing the divisor (e.g., 4.0)
Output:

If division is successful: Division result: [result]
If division by zero: Error: Cannot divide by zero
*/ 

use std::io;

// TODO: Crie a função safe_divide aqui
fn safe_divide(dividend: f64, divisor: f64) -> Result<f64, &'static str> {
    // Seu código aqui
    if divisor == 0.0 {
        Err("Cannot divide by zero")
    } else {
        Ok(dividend / divisor)
    }
}

fn main() {
    // Lê a primeira entrada (dividendo)
    let mut dividend_input = String::new();
    io::stdin().read_line(&mut dividend_input).expect("Failed to read line");
    let dividend: f64 = dividend_input.trim().parse().expect("Invalid number");
    
    // Lê a segunda entrada (divisor)
    let mut divisor_input = String::new();
    io::stdin().read_line(&mut divisor_input).expect("Failed to read line");
    let divisor: f64 = divisor_input.trim().parse().expect("Invalid number");
    
    // TODO: Chame safe_divide e use match para lidar com o Result
    // Imprima a saída apropriada com base no resultado
    let result = safe_divide(dividend, divisor);
    match result {
        Ok(value) => println!("Division result: {}", value),
        Err(error) => println!("Error: {}", error),
    }
}