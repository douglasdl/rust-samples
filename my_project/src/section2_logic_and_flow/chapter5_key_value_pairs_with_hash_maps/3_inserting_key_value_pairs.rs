/*
Inserting Key-Value Pairs



Now that you have an empty hash map, it's time to add some data to it. You can insert key-value pairs into a hash map using the .insert() method:

let mut capitals = HashMap::new();
capitals.insert("France", "Paris");
capitals.insert("Japan", "Tokyo");
The .insert() method takes two parameters: the key and the value. In this example, we're storing country names as keys and their capital cities as values.

There's an important behavior to remember: if you insert a key that already exists in the hash map, the new value will overwrite the old one:

capitals.insert("France", "Lyon");  // This overwrites "Paris"
After this operation, looking up "France" would return "Lyon" instead of "Paris". This overwriting behavior makes .insert() useful for both adding new entries and updating existing ones in your hash map.

challenge icon
Challenge

Easy
You will receive three inputs: a product name, its price (as a decimal number), and another product name. Create a mutable hash map to store product names as keys (type String) and their prices as values (type f64). Insert the first product with its price into the hash map. Then insert the second product with a price of 0.0. Finally, update the second product's price to 15.99 by inserting it again with the new price. Print all the information in the exact format shown below.

Note on .clone(): When inserting a String variable as a key into a HashMap, Rust's ownership rules mean the variable would be moved into the map and no longer usable afterward. To keep using the variable (e.g., for printing or inserting again), call .clone() on it when passing it to insert(). For example: map.insert(name.clone(), price).

Requirements:

Import HashMap from std::collections
Create a mutable hash map with types HashMap<String, f64>
Read the first input as the first product name
Read the second input and convert it to f64 for the first product's price
Insert the first product and its price into the hash map using .clone() on the product name
Read the third input as the second product name
Insert the second product with a price of 0.0 using .clone() on the product name
Insert the second product again with a price of 15.99 using .clone() (this will overwrite the previous value)
Print the information for both products in the exact format shown below
Input:

First line: First product name (e.g., Laptop)
Second line: First product price as a decimal number (e.g., 999.99)
Third line: Second product name (e.g., Mouse)
Output:

First line: Inserted [first_product] at $[price]
Second line: Inserted [second_product] at $0.00
Third line: Updated [second_product] to $15.99

Hints icon
Hints

Hint 1
Revealed
Toggle hint
In Rust, when you insert a variable into a HashMap, the value is moved into the map. If you need to use that variable again after inserting it, you'll need to make a copy of it first.
Hint 2
Revealed
Toggle hint
Rust String values don't implement Copy, so inserting a String key into a HashMap moves ownership. To keep using the original variable afterward, you can call .clone() on it to create an independent copy.
Hint 3
Revealed
Toggle hint
When inserting product1 and product2 as keys, use .clone() so you can still reference those variables in your println! calls — for example: products.insert(product1.clone(), price1);
Hint 4
Revealed
Toggle hint
To print a value from a HashMap you don't need to look it up — since you already know the values being inserted (price1, 0.0, 15.99), you can print them directly alongside the product name variable in your println! statements.

*/ 

use std::collections::HashMap;
use std::io;

fn main() {
    // Read inputs
    let mut product1 = String::new();
    io::stdin().read_line(&mut product1).expect("Failed to read line");
    let product1 = product1.trim().to_string();
    
    let mut price1_input = String::new();
    io::stdin().read_line(&mut price1_input).expect("Failed to read line");
    let price1: f64 = price1_input.trim().parse().expect("Failed to parse price");
    
    let mut product2 = String::new();
    io::stdin().read_line(&mut product2).expect("Failed to read line");
    let product2 = product2.trim().to_string();
    
    // TODO: Write your code below
    // Create a mutable HashMap, insert products, and print the required output
    let mut products: HashMap<String, f64> = HashMap::new();
    products.insert(product1.clone(), price1);
    products.insert(product2.clone(), 0.0);
    products.insert(product2.clone(), 15.99);
    
    println!("Inserted {} at ${:.2}", product1, price1);
    println!("Inserted {} at ${:.2}", product2, 0.0);
    println!("Updated {} to ${:.2}", product2, 15.99);
}