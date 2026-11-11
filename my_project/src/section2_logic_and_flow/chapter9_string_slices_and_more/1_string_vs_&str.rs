/*
String vs. &str



Rust has two main string types that serve different purposes: String and &str. Understanding the difference between them is crucial for working effectively with text in Rust.

A String is an owned, heap-allocated string that you can modify. When you create a String, your program owns that data and is responsible for managing its memory. You can grow it, shrink it, and change its contents because it's mutable when declared with mut.

let mut my_string = String::from("Hello");
my_string.push_str(" World"); // This works because String is owned and mutable
In contrast, &str (pronounced "string slice") is a borrowed, immutable view into string data. It doesn't own the data it points to - it's just a reference to some string content that exists elsewhere. String literals like "Hello" are actually of type &str.

let my_slice: &str = "Hello World"; // This is a string slice
Think of String as owning a book that you can write in and modify, while &str is like borrowing someone else's book - you can read it, but you can't change it. This distinction is fundamental to Rust's ownership system and helps ensure memory safety.
*/ 