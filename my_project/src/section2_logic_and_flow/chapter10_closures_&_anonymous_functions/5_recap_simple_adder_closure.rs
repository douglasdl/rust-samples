/*
Recap - Simple Adder Closure

challenge icon
Challenge

Easy
Write a function create_adder that takes base_value and returns the result of adding 10 to it using a closure.
Inside the function, create a closure that captures base_value from the environment and adds 10 to it. Call the closure and return its result.

Parameters:

base_value (i32): The number to which 10 will be added
Returns: The result of adding 10 to base_value (i32)



*/ 

fn create_adder(base_value: i32) -> i32 {
  // Write code here
  let add_ten = || base_value + 10;

  add_ten()
}

