/*
What is a Closure?



A closure is an anonymous function that you can store in a variable or pass as an argument to other functions. Unlike regular functions that you define with the fn keyword, closures are created inline and don't need a name.

The basic syntax for a closure uses vertical bars to define parameters, followed by the function body:

|param1, param2| { 
    // function body goes here
}
Here's a simple example of a closure that adds two numbers:

let add = |x, y| x + y;
let result = add(5, 3); // result is 8
*/ 