/*
Accessing Values



Once you have data stored in your hash map, you'll need a way to retrieve it. The .get() method allows you to look up a value using its key:

let value = my_map.get("some_key");
However, there's an important detail to understand: .get() doesn't directly return the value. Instead, it returns an Option type — a built-in Rust type that represents either a value being present (Some) or absent (None). This is because the key you're looking for might not exist in the hash map.

When the key exists, .get() returns Some(&value) - notice the & symbol, which means you get a reference to the value. When the key doesn't exist, it returns None:

match capitals.get("France") {
    Some(capital) => println!("The capital is {}", capital),
    None => println!("Country not found"),
}
In the Some(capital) match arm, Rust automatically handles the reference for you — capital here is bound to the inner referenced value, so you don't need to write & explicitly in the pattern.

This design prevents your program from crashing when you try to access a key that doesn't exist. Instead of causing an error, Rust forces you to handle both possibilities - finding the value or not finding it - making your code more robust and predictable.

challenge icon
Challenge

Easy
You will receive two inputs: a country name and a capital city name. Create a hash map that stores country names as keys (type String) and their capital cities as values (type String). Insert the country and its capital into the hash map. Then, you will receive a third input with another country name to look up. Use the .get() method to retrieve the capital for this country and handle both cases: when the country exists in the map and when it doesn't.

Note: .get() returns an Option — either Some(&value) (a reference to the value) when the key exists, or None when it doesn't. In a match arm, you can write Some(capital) and Rust will automatically handle the reference for you, so you can use capital directly in your print statement.

Requirements:

Import HashMap from std::collections
Create a mutable hash map with types HashMap<String, String>
Read the first input as the country name
Read the second input as the capital city name
Insert the country and capital into the hash map
Read the third input as the country to look up
Use .get() to retrieve the capital for the lookup country
Use match to handle the Option returned by .get()
If the country is found, print: The capital of [country] is [capital]
If the country is not found, print: [country] not found in the map
Input:

First line: Country name to insert (e.g., France)
Second line: Capital city name (e.g., Paris)
Third line: Country name to look up (e.g., France or Germany)
Output:

If the lookup country exists: The capital of [country] is [capital]
If the lookup country doesn't exist: [country] not found in the map


*/ 

use std::collections::HashMap;
use std::io;

fn main() {
    // Read the country name to insert
    let mut country = String::new();
    io::stdin().read_line(&mut country).expect("Failed to read line");
    let country = country.trim().to_string();
    
    // Read the capital city name
    let mut capital = String::new();
    io::stdin().read_line(&mut capital).expect("Failed to read line");
    let capital = capital.trim().to_string();
    
    // Read the country name to look up
    let mut lookup_country = String::new();
    io::stdin().read_line(&mut lookup_country).expect("Failed to read line");
    let lookup_country = lookup_country.trim().to_string();
    
    // TODO: Write your code below
    // Create a HashMap, insert the country and capital, then look up the country
    let mut capitals: HashMap<String, String> = HashMap::new();
    capitals.insert(country.clone(), capital);

    match capitals.get(&lookup_country) {
        Some(capital) => println!("The capital of {} is {}", country, capital),
        None => println!("{} not found in the map", lookup_country),
    }
}