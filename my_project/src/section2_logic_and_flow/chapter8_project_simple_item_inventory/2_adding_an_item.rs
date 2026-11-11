/*
Adding an Item

challenge icon
Challenge

Easy
You will receive three inputs. The first input is an item name, the second input is the item's price, and the third input is the initial quantity in stock. Create a HashMap to store inventory data where the key is the item name and the value is a tuple containing the price and quantity (f64, i32). Add the item to the inventory and print a confirmation message followed by the current inventory details.

Requirements:

Import HashMap from std::collections
Read the first input (item name) and trim whitespace
Read the second input (price) and trim whitespace
Parse the price to f64
Read the third input (quantity) and trim whitespace
Parse the quantity to i32
Create a mutable HashMap<String, (f64, i32)> for the inventory
Insert the item into the inventory with the price and quantity as a tuple
Print: Added [item_name] to inventory
Print: Price: $[price]
Print: Quantity: [quantity]
Input:

First line: Item name (e.g., Laptop)
Second line: Price as a number (e.g., 999.99)
Third line: Quantity as a number (e.g., 15)
Output:

First line: Added [item_name] to inventory
Second line: Price: $[price]
Third line: Quantity: [quantity]
*/ 

use std::collections::HashMap;
use std::io;

fn main() {
    // Read item name
    let mut item_name = String::new();
    io::stdin().read_line(&mut item_name).expect("Failed to read line");
    let item_name = item_name.trim().to_string();
    
    // Read price
    let mut price_input = String::new();
    io::stdin().read_line(&mut price_input).expect("Failed to read line");
    let price: f64 = price_input.trim().parse().expect("Failed to parse price");
    
    // Read quantity
    let mut quantity_input = String::new();
    io::stdin().read_line(&mut quantity_input).expect("Failed to read line");
    let quantity: i32 = quantity_input.trim().parse().expect("Failed to parse quantity");
    
    // TODO: Write your code below
    // Create a HashMap to store inventory
    let mut inventory: HashMap<String, (f64, i32)> = HashMap::new();
    // Add the item to the inventory
    inventory.insert(item_name.clone(), (price, quantity));

    // Print the confirmation messages
    println!("Added {} to inventory", item_name);
    println!("Price: ${}", price);
    println!("Quantity: {}", quantity);
}