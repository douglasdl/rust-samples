/*
Structs Overview



Now that you understand what structs are, let's learn how to actually create and use them. This involves two steps: first defining the struct's structure, then creating instances of it.

To define a struct, you use the struct keyword followed by the struct's name and its fields inside curly braces. Each field has a name and a type:

struct User {
    username: String,
    active: bool,
}
This creates a blueprint for a User struct with two fields: username of type String and active of type bool.


Struct names in Rust follow PascalCase (also called CamelCase) — each word starts with a capital letter, with no underscores. For example: User, BlogPost, GamePlayer.
Once you've defined a struct, you can create instances of it by specifying values for each field:

let user1 = User {
    username: String::from("alice"),
    active: true,
};
Notice that you provide the struct name followed by curly braces containing each field name, a colon, and its value. The order of fields doesn't have to match the definition order.

To access a field's value from a struct instance, use dot notation — the instance name followed by a dot and the field name:

println!("{}", user1.username); // alice
println!("{}", user1.active);   // true
You can also update a field's value using dot notation, as long as the instance is declared with let mut:

let mut user1 = User {
    username: String::from("alice"),
    active: true,
};
user1.active = false;
challenge icon
Challenge

Easy
You will receive three inputs: a product name, a price (as a decimal number), and a stock quantity (as an integer). Define a Product struct with three fields: name of type String, price of type f64, and stock of type i32. Then create an instance of this struct using the input values and print the product information.

Requirements:

Define a Product struct with fields: name: String, price: f64, and stock: i32
Read the first input as the product name
Read the second input and convert it to f64 for the price
Read the third input and convert it to i32 for the stock quantity
Create an instance of the Product struct with these values
Print the product information in the exact format shown below
Input:

First line: Product name (e.g., Laptop)
Second line: Price as a decimal number (e.g., 999.99)
Third line: Stock quantity as an integer (e.g., 15)
Output:

First line: Product: [name]
Second line: Price: $[price]
Third line: Stock: [quantity]
*/ 

use std::io;

// TODO: Define your Product struct here
struct Product {
    name: String,
    price: f64,
    stock: i32
}

fn main() {
    // Read product name
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim().to_string();
    
    // Read price
    let mut price_input = String::new();
    io::stdin().read_line(&mut price_input).expect("Failed to read line");
    let price: f64 = price_input.trim().parse().expect("Failed to parse price");
    
    // Read stock quantity
    let mut stock_input = String::new();
    io::stdin().read_line(&mut stock_input).expect("Failed to read line");
    let stock: i32 = stock_input.trim().parse().expect("Failed to parse stock");

    // TODO: Create an instance of Product struct using the input values
    let product = Product {
        name: name,
        price: price,
        stock: stock
    };

    // TODO: Print the product information in the required format
    println!("Product: {}", product.name);
    println!("Price: ${}", product.price);
    println!("Stock: {}", product.stock);
}