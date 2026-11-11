/*
Checking Stock

challenge icon
Challenge

Easy
You will receive four inputs. The first input is an item name to check. The second input is a comma-separated list of item names in the inventory. The third input is a comma-separated list of prices corresponding to each item. The fourth input is a comma-separated list of quantities corresponding to each item. Build a HashMap to store the inventory data where the key is the item name and the value is a tuple containing the price and quantity (f64, i32). Then check if the requested item exists in the inventory using .get() and print the appropriate stock information.

Requirements:

Import HashMap from std::collections
Read the first input (item name to check) and trim whitespace
Read the second input (comma-separated item names) and trim whitespace
Read the third input (comma-separated prices) and trim whitespace
Read the fourth input (comma-separated quantities) and trim whitespace
Create a mutable HashMap<String, (f64, i32)> for the inventory
Split the item names by commas
Split the prices by commas and parse each to f64
Split the quantities by commas and parse each to i32
Insert each item into the inventory with its corresponding price and quantity as a tuple
Use .get() to check if the requested item exists in the inventory
Use a match expression to handle the Option returned by .get()
In the Some arm, extract the price and quantity from the tuple and print:
[item_name] is in stock
Price: $[price]
Quantity: [quantity]
In the None arm, print: [item_name] is not in stock
Input:

First line: Item name to check (e.g., Laptop)
Second line: Comma-separated item names (e.g., Laptop,Mouse,Keyboard)
Third line: Comma-separated prices (e.g., 999.99,25.50,75.00)
Fourth line: Comma-separated quantities (e.g., 15,50,30)
Output:

If the item exists in inventory:
First line: [item_name] is in stock
Second line: Price: $[price]
Third line: Quantity: [quantity]
If the item does not exist: [item_name] is not in stock
*/ 

use std::collections::HashMap;
use std::io;

fn main() {
    // Read the item name to check
    let mut item_to_check = String::new();
    io::stdin().read_line(&mut item_to_check).expect("Failed to read line");
    let item_to_check = item_to_check.trim();
    
    // Read the comma-separated item names
    let mut items_input = String::new();
    io::stdin().read_line(&mut items_input).expect("Failed to read line");
    let items_input = items_input.trim();
    
    // Read the comma-separated prices
    let mut prices_input = String::new();
    io::stdin().read_line(&mut prices_input).expect("Failed to read line");
    let prices_input = prices_input.trim();
    
    // Read the comma-separated quantities
    let mut quantities_input = String::new();
    io::stdin().read_line(&mut quantities_input).expect("Failed to read line");
    let quantities_input = quantities_input.trim();
    
    // TODO: Write your code below
    // Create a HashMap to store inventory data
    let mut inventory: HashMap<String, (f64, i32)> = HashMap::new();

    // Split the inputs and populate the HashMap
    let items: Vec<&str> = items_input.split(',').collect();
    let prices: Vec<f64> = prices_input
        .split(',')
        .map(|price| price.trim().parse().expect("Invalid price"))
        .collect();
    let quantities: Vec<i32> = quantities_input
        .split(',')
        .map(|quantity| quantity.trim().parse().expect("Invalid quantity"))
        .collect();

    for i in 0..items.len() {
        inventory.insert(
            items[i].trim().to_string(),
            (prices[i], quantities[i]),
        );
    }

    // Use .get() to check if the item exists
    // Use match to handle the Option and print the appropriate output
    match inventory.get(item_to_check) {
        Some((price, quantity)) => {
            println!("{} is in stock", item_to_check);
            println!("Price: ${}", price);
            println!("Quantity: {}", quantity);
        }
        None => {
            println!("{} is not in stock", item_to_check);
        }
    }
}