/*
Selling an Item

challenge icon
Challenge

Easy
You will receive five inputs. The first input is the item name to sell. The second input is the quantity to sell. The third input is a comma-separated list of item names in the inventory. The fourth input is a comma-separated list of prices corresponding to each item. The fifth input is a comma-separated list of quantities corresponding to each item. Build a HashMap to store the inventory data where the key is the item name and the value is a tuple containing the price and quantity (f64, i32). Then attempt to sell the requested quantity of the item.

Requirements:

Import HashMap from std::collections
Read the first input (item name to sell) and trim whitespace
Read the second input (quantity to sell) and trim whitespace
Parse the quantity to sell to i32
Read the third input (comma-separated item names) and trim whitespace
Read the fourth input (comma-separated prices) and trim whitespace
Read the fifth input (comma-separated quantities) and trim whitespace
Create a mutable HashMap<String, (f64, i32)> for the inventory
Split the item names by commas
Split the prices by commas and parse each to f64
Split the quantities by commas and parse each to i32
Insert each item into the inventory with its corresponding price and quantity as a tuple
Use .get() to check if the requested item exists in the inventory
Use a match expression to handle the Option returned by .get()
In the Some arm:
Extract the price and current quantity from the tuple
Check if the current quantity is greater than or equal to the quantity to sell
If yes:
Calculate the new quantity by subtracting the quantity to sell from the current quantity
Calculate the total price by multiplying the item price by the quantity to sell
Use .insert() to update the inventory with the new quantity
Print: Sold [quantity_to_sell] [item_name]
Print: Total: $[total_price]
Print: Remaining stock: [new_quantity]
If no (insufficient stock):
Print: Insufficient stock for [item_name]
Print: Available: [current_quantity]
In the None arm, print: [item_name] is not in stock
Input:

First line: Item name to sell (e.g., Mouse)
Second line: Quantity to sell (e.g., 10)
Third line: Comma-separated item names (e.g., Laptop,Mouse,Keyboard)
Fourth line: Comma-separated prices (e.g., 999.99,25.50,75.00)
Fifth line: Comma-separated quantities (e.g., 15,50,30)
Output:

If the item exists and there is sufficient stock:
First line: Sold [quantity_to_sell] [item_name]
Second line: Total: $[total_price]
Third line: Remaining stock: [new_quantity]
If the item exists but there is insufficient stock:
First line: Insufficient stock for [item_name]
Second line: Available: [current_quantity]
If the item does not exist: [item_name] is not in stock
*/ 

use std::collections::HashMap;
use std::io;

fn main() {
    // Read input
    let mut item_to_sell = String::new();
    io::stdin().read_line(&mut item_to_sell).expect("Failed to read line");
    let item_to_sell = item_to_sell.trim();
    
    let mut quantity_to_sell_input = String::new();
    io::stdin().read_line(&mut quantity_to_sell_input).expect("Failed to read line");
    let quantity_to_sell: i32 = quantity_to_sell_input.trim().parse().expect("Invalid number");
    
    let mut item_names_input = String::new();
    io::stdin().read_line(&mut item_names_input).expect("Failed to read line");
    let item_names_input = item_names_input.trim();
    
    let mut prices_input = String::new();
    io::stdin().read_line(&mut prices_input).expect("Failed to read line");
    let prices_input = prices_input.trim();
    
    let mut quantities_input = String::new();
    io::stdin().read_line(&mut quantities_input).expect("Failed to read line");
    let quantities_input = quantities_input.trim();
    
    // TODO: Write your code below
    // Create a HashMap to store inventory
    let mut inventory: HashMap<String, (f64, i32)> = HashMap::new();

    // Process the inputs and build the inventory
    let items: Vec<&str> = item_names_input.split(',').collect();
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

    // Check if the item exists and handle the sale
    match inventory.get(item_to_sell) {
        Some((price, current_quantity)) => {
            if *current_quantity >= quantity_to_sell {
                let new_quantity = *current_quantity - quantity_to_sell;
                let total_price = *price * quantity_to_sell as f64;

                inventory.insert(
                    item_to_sell.to_string(),
                    (*price, new_quantity),
                );

                println!("Sold {} {}", quantity_to_sell, item_to_sell);
                println!("Total: ${}", total_price);
                println!("Remaining stock: {}", new_quantity);
            } else {
                println!("Insufficient stock for {}", item_to_sell);
                println!("Available: {}", current_quantity);
            }
        }
        None => {
            println!("{} is not in stock", item_to_sell);
        }
    }
}