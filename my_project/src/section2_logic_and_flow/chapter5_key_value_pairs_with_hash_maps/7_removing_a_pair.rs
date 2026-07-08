/*
Removing a Pair



Sometimes you need to remove data from your hash map entirely. Rust provides the .remove() method to delete a key-value pair by specifying the key:

let mut inventory = HashMap::new();
inventory.insert("apples", 50);
inventory.insert("bananas", 30);

// Remove the "apples" entry completely
inventory.remove("apples");
The .remove() method takes the key as a parameter and removes both the key and its associated value from the hash map. After calling .remove("apples"), trying to look up "apples" would return None.

Like many hash map operations, .remove() returns an Option. It returns Some(value) containing the removed value if the key existed, or None if the key wasn't found:

let removed_value = inventory.remove("bananas");
match removed_value {
    Some(count) => println!("Removed {} bananas", count),
    None => println!("No bananas found to remove"),
}
This return value lets you know whether the removal was successful and gives you access to the value that was removed, which can be useful for logging or cleanup operations.

challenge icon
Challenge

Easy
You will receive an integer n indicating the number of items in an inventory. Then you will receive n pairs of inputs: an item name followed by its quantity (as an integer). After that, you will receive one more input with an item name to remove from the inventory. Create a hash map to store the inventory, insert all items, remove the specified item, and print the removal result along with the remaining inventory.

Requirements:

Import HashMap from std::collections
Create a mutable hash map with types HashMap<String, i32>
Read the first input and convert it to i32 to get the number of items
Use a loop to read n pairs of inputs (item name, then quantity)
Insert each item and its quantity into the hash map
Read one more input as the item name to remove
Use .remove() to remove the item from the hash map
Use match to handle the Option returned by .remove()
If the item was found and removed, print: Removed [quantity] [item_name]
If the item was not found, print: [item_name] not found
After handling the removal, iterate over the remaining hash map and print each item in the format: [item_name]: [quantity]
Input:

First line: An integer n (e.g., 3)
Next n pairs of lines:
Item name (e.g., apples)
Quantity as an integer (e.g., 50)
Last line: Item name to remove (e.g., bananas)
Output:

First line: Either Removed [quantity] [item_name] or [item_name] not found
Following lines: One line for each remaining item in the format: [item_name]: [quantity]
The order of remaining items may vary between test runs
*/ 

use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the number of items
    let n: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Create a mutable hash map to store inventory
    let mut inventory: HashMap<String, i32> = HashMap::new();
    
    // Read n pairs of inputs (item name and quantity)
    for _ in 0..n {
        let item_name = lines.next().unwrap().unwrap().trim().to_string();
        let quantity: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        
        // TODO: Insert the item and quantity into the hash map
        inventory.insert(item_name, quantity);
    }
    
    // Read the item name to remove
    let item_to_remove = lines.next().unwrap().unwrap().trim().to_string();
    
    // TODO: Remove the item from the hash map and handle the result using match
    let removed = inventory.remove(&item_to_remove);

    // Print the removal result (either "Removed [quantity] [item_name]" or "[item_name] not found")
    match removed {
        Some(count) => println!("Removed {} {}", count, item_to_remove),
        None => println!("{} not found", item_to_remove),
    }

    // TODO: Iterate over the remaining inventory and print each item in the format "[item_name]: [quantity]"
    for (item_name, quantity) in &inventory {
        println!("{}: {}", item_name, quantity);
    }
}