/*
The expect



While .unwrap() gets the job done, it has a major drawback: when it panics on a None, the error message is generic and unhelpful. This is where .expect() comes to the rescue.

The .expect() method works exactly like .unwrap() - it extracts the value from a Some and panics on a None. The key difference is that .expect() lets you provide a custom panic message:

let maybe_score = Some(95);
let score = maybe_score.expect("Score should always be present");
println!("Your score is: {}", score);
If the Option contains a Some, .expect() returns the inner value just like .unwrap(). But if it's a None, instead of a generic panic message, you'll see your custom message, making it much easier to understand what went wrong and where.

Challenge

Easy
You will receive two inputs. The first input is a product name, and the second input indicates whether the product is in stock (yes) or out of stock (no). Create an Option<String> that contains Some(product_name) if the product is in stock, or None if it's out of stock. Use .expect() with a descriptive message to extract the product name and print it.

Requirements:

Read the first input (product name) and trim whitespace
Read the second input (stock status) and trim whitespace
Create an Option<String> variable:
If the stock status is yes, assign Some(product_name.to_string())
If the stock status is no, assign None
Use .expect() with the message "Product should be in stock" to extract the product name
Print the extracted product name in the format: Product available: [product_name]
Input:

First line: A product name (e.g., Laptop)
Second line: Either yes or no
Output:

If the stock status is yes: Product available: [product_name]
If the stock status is no: The program will panic with the custom message "Product should be in stock" (this is expected behavior for this challenge)
Note: When the stock status is no, calling .expect() on None will cause a panic with your custom message. This demonstrates how .expect() provides more helpful error information compared to .unwrap().



*/ 

use std::io;

fn main() {
    // Read product name
    let mut product_name = String::new();
    io::stdin().read_line(&mut product_name).expect("Failed to read line");
    let product_name = product_name.trim();
    
    // Read stock status
    let mut stock_status = String::new();
    io::stdin().read_line(&mut stock_status).expect("Failed to read line");
    let stock_status = stock_status.trim();
    
    // TODO: Write your code below
    // Create an Option<String> based on stock_status
    let maybe_available: Option<String> = if stock_status == "yes" {
        Some(product_name.to_string())
    } else {
        None
    };

    // Use .expect() to extract the product name
    let product = maybe_available.expect("Product should be in stock");

    // Print the result in the format: Product available: [product_name]
    println!("Product available: {}", product);
}