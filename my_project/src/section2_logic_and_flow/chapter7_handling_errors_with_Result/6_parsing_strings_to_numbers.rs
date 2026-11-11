/*
Parsing Strings to Numbers



One of the most common real-world applications of Result is converting strings to numbers. In Rust, this is done using the .parse() method, which demonstrates perfectly why Result exists - because parsing can fail.

The .parse() method attempts to convert a string into a number, but not every string represents a valid number. For example, "42" can be parsed into the number 42, but "hello" cannot be converted to any number:

let valid_number = "42";
let invalid_number = "hello";

let result1: Result = valid_number.parse();
let result2: Result = invalid_number.parse();
When parsing succeeds, .parse() returns Ok(number) containing the converted value. When it fails, it returns Err(error) with information about why the parsing failed. This forces you to handle both possibilities explicitly, preventing your program from crashing on invalid input.

You can handle the parsing result using any of the Result techniques you've learned - match for explicit handling, unwrap() if you're certain it will succeed, or the ? operator to propagate errors.

challenge icon
Challenge

Easy
Write a function parse_to_integer that takes a string input and returns the parsed integer value. If the string cannot be parsed into a valid integer, return 0 as a default value.
Use the .parse() method to convert the string to an i32, and handle the Result appropriately to provide a safe return value.

Parameters:

input (String): The string to parse into an integer
Returns: The parsed integer if successful, or 0 if parsing fails (i32)
*/ 

fn parse_to_integer(input: String) -> i32 {
  // Escreva o código aqui
  let result: Result<i32, _> = input.parse();
  
  match result {
      Ok(value) => value,
      Err(_) => 0,
  }
}
