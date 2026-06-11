/* 
There are two main ways to create strings in Rust:

Method 1: Simple string literals (str):

let str1 = "hello";
let str2 = "hello";
let str3 = "Hello";
These are simple, fixed strings that are built into your program.

Method 2: Full Strings (String):

let string1: String = "hello".to_string();
let string2: String = String::from("hello");
let string3 = "hello".to_owned();
These create a more flexible type of string that you can change later. All three lines do basically the same thing, they're just different ways to write it.

The :: in String::from is Rust's syntax for calling an associated function directly on a type — think of it as String's own built-in way to create a new string value.

For most string comparisons, both types work the same way:

let result1 = str1 == str2;  // true
let result2 = str1 == str3;  // false (case-sensitive)

// Comparing String with String
let result3 = string1 == string2;  // true

// You can even compare String with &str
let result4 = string1 == str1;     // true

*/