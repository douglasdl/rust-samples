/*
Putting it all together

challenge icon
Challenge

Easy
You will receive six inputs. The first input is a command that can be either add, check, or sell. The second input is an item name. The third input is a number representing either the quantity to sell (for sell command) or the initial quantity (for add command). The fourth input is a comma-separated list of item names in the inventory. The fifth input is a comma-separated list of prices corresponding to each item. The sixth input is a comma-separated list of quantities corresponding to each item.

Build a complete inventory management system that handles all three operations: adding items, checking stock, and selling items. Use a HashMap to store the inventory data where the key is the item name and the value is a tuple containing the price and quantity (f64, i32).

Requirements:

Import HashMap from std::collections
Read the first input (command) and trim whitespace
Read the second input (item name) and trim whitespace
Read the third input (quantity/price value) and trim whitespace
Parse the third input to f64
Read the fourth input (comma-separated item names) and trim whitespace
Read the fifth input (comma-separated prices) and trim whitespace
Read the sixth input (comma-separated quantities) and trim whitespace
Create a mutable HashMap<String, (f64, i32)> for the inventory
Split the item names by commas
Split the prices by commas and parse each to f64
Split the quantities by commas and parse each to i32
Insert each item into the inventory with its corresponding price and quantity as a tuple
Use a match expression on the command to determine which operation to perform
For add command:
Use .entry() and .or_insert() to add the item if it doesn't exist, using the third input as the price and 0 as the initial quantity
Print: Added [item_name] to inventory
Print: Price: $[price]
Print: Quantity: 0
For check command:
Use .get() to check if the item exists
Use a match expression to handle the Option
If Some, print:
[item_name] is in stock
Price: $[price]
Quantity: [quantity]
If None, print: [item_name] is not in stock
For sell command:
Parse the third input to i32 as the quantity to sell
Use .get() to check if the item exists
Use a match expression to handle the Option
If Some:
Check if current quantity is sufficient
If yes, update the inventory with the new quantity and print:
Sold [quantity_to_sell] [item_name]
Total: $[total_price]
Remaining stock: [new_quantity]
If no, print:
Insufficient stock for [item_name]
Available: [current_quantity]
If None, print: [item_name] is not in stock
For any other command, print: Invalid command
Input:

First line: Command (add, check, or sell)
Second line: Item name (e.g., Tablet)
Third line: Number (price for add, quantity for sell, ignored for check)
Fourth line: Comma-separated item names (e.g., Laptop,Mouse,Keyboard)
Fifth line: Comma-separated prices (e.g., 999.99,25.50,75.00)
Sixth line: Comma-separated quantities (e.g., 15,50,30)
Output:

For add command:
Added [item_name] to inventory
Price: $[price]
Quantity: 0
For check command (item exists):
[item_name] is in stock
Price: $[price]
Quantity: [quantity]
For check command (item doesn't exist): [item_name] is not in stock
For sell command (sufficient stock):
Sold [quantity_to_sell] [item_name]
Total: $[total_price]
Remaining stock: [new_quantity]
For sell command (insufficient stock):
Insufficient stock for [item_name]
Available: [current_quantity]
For sell command (item doesn't exist): [item_name] is not in stock
For invalid command: Invalid command
*/ 

use std::collections::HashMap;
use std::io;

fn main() {
    // Read inputs
    let mut command = String::new();
    io::stdin().read_line(&mut command).expect("Failed to read line");
    let command = command.trim();
    
    let mut item_name = String::new();
    io::stdin().read_line(&mut item_name).expect("Failed to read line");
    let item_name = item_name.trim();
    
    let mut value_input = String::new();
    io::stdin().read_line(&mut value_input).expect("Failed to read line");
    let value_input = value_input.trim();
    
    let mut items_input = String::new();
    io::stdin().read_line(&mut items_input).expect("Failed to read line");
    let items_input = items_input.trim();
    
    let mut prices_input = String::new();
    io::stdin().read_line(&mut prices_input).expect("Failed to read line");
    let prices_input = prices_input.trim();
    
    let mut quantities_input = String::new();
    io::stdin().read_line(&mut quantities_input).expect("Failed to read line");
    let quantities_input = quantities_input.trim();
    
    // Create inventory HashMap
    let mut inventory: HashMap<String, (f64, i32)> = HashMap::new();
    
    // TODO: Write your code below
    // Parse the comma-separated inputs and populate the inventory
    let items: Vec<&str> = items_input.split(',').map(str::trim).collect();

    let prices: Vec<f64> = prices_input
        .split(',')
        .map(|p| p.trim().parse::<f64>().expect("Invalid price"))
        .collect();

    let quantities: Vec<i32> = quantities_input
        .split(',')
        .map(|q| q.trim().parse::<i32>().expect("Invalid quantity"))
        .collect();

    for i in 0..items.len() {
        inventory.insert(
            items[i].to_string(),
            (prices[i], quantities[i]),
        );
    }

    // Use match expression on command to handle add, check, and sell operations
    // Remember to output the appropriate messages based on the command
    match command {
        "add" => {
            let price = value_input.parse::<f64>().expect("Invalid price");

            let entry = inventory.entry(item_name.to_string()).or_insert((price, 0));

            println!("Added {} to inventory", item_name);
            println!("Price: ${}", entry.0);
            println!("Quantity: {}", entry.1);
        }

        "check" => {
            match inventory.get(item_name) {
                Some((price, quantity)) => {
                    println!("{} is in stock", item_name);
                    println!("Price: ${}", price);
                    println!("Quantity: {}", quantity);
                }
                None => {
                    println!("{} is not in stock", item_name);
                }
            }
        }

        "sell" => {
            let quantity_to_sell = value_input
                .parse::<i32>()
                .expect("Invalid quantity");

            match inventory.get(item_name) {
                Some((price, quantity)) => {
                    if *quantity >= quantity_to_sell {
                        let total_price = price * quantity_to_sell as f64;
                        let new_quantity = quantity - quantity_to_sell;

                        inventory.insert(
                            item_name.to_string(),
                            (*price, new_quantity),
                        );

                        println!("Sold {} {}", quantity_to_sell, item_name);
                        println!("Total: ${}", total_price);
                        println!("Remaining stock: {}", new_quantity);
                    } else {
                        println!("Insufficient stock for {}", item_name);
                        println!("Available: {}", quantity);
                    }
                }
                None => {
                    println!("{} is not in stock", item_name);
                }
            }
        }

        _ => {
            println!("Invalid command");
        }
    }
}