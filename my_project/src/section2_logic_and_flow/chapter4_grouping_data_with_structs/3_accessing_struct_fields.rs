/*
Accessing Struct Fields



Once you've created a struct instance, you need a way to retrieve the values stored inside it. Rust uses dot notation to access individual fields within a struct.

The syntax is straightforward: you write the struct instance name, followed by a dot, then the field name you want to access:

let product = Product {
    name: String::from("Laptop"),
    price: 999,
};

println!("Product: {}", product.name);
println!("Price: ${}", product.price);
This dot notation works just like accessing properties in other programming languages. You can use the accessed values in expressions, assign them to variables, or pass them to functions - they behave exactly like any other value of that type.

challenge icon
Challenge

Easy
You will receive three inputs: a car model name, the car's year (as an integer), and its mileage (as an integer). Define a Car struct with three fields: model of type String, year of type i32, and mileage of type i32. Create an instance of this struct using the input values, then access and print each field.

Requirements:

Define a Car struct with fields: model: String, year: i32, and mileage: i32
Read the first input as the car model
Read the second input and convert it to i32 for the year
Read the third input and convert it to i32 for the mileage
Create an instance of the Car struct with these values
Use dot notation to access each field and print the car information in the exact format shown below
Input:

First line: Car model (e.g., Tesla Model 3)
Second line: Year as an integer (e.g., 2022)
Third line: Mileage as an integer (e.g., 15000)
Output:

First line: Car: [model]
Second line: Year: [year]
Third line: Mileage: [mileage] km
*/ 

use std::io;

// TODO: Define the Car struct here with fields: model, year, and mileage
struct Car {
    model: String, 
    year: i32, 
    mileage: i32
}

fn main() {
    // Read the car model
    let mut model = String::new();
    io::stdin().read_line(&mut model).expect("Failed to read line");
    let model = model.trim().to_string();
    
    // Read the year
    let mut year_input = String::new();
    io::stdin().read_line(&mut year_input).expect("Failed to read line");
    let year: i32 = year_input.trim().parse().expect("Invalid year");
    
    // Read the mileage
    let mut mileage_input = String::new();
    io::stdin().read_line(&mut mileage_input).expect("Failed to read line");
    let mileage: i32 = mileage_input.trim().parse().expect("Invalid mileage");
    
    // TODO: Create an instance of the Car struct using the input values
    let car = Car {
        model: model, 
        year: year, 
        mileage: mileage
    };

    // TODO: Access and print each field in the required format
    // Car: [model]
    println!("Car: {}", car.model);
    // Year: [year]
    println!("Year: {}", car.year);
    // Mileage: [mileage] km
    println!("Mileage: {} km", car.mileage);
}