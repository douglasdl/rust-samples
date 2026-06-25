use std::io;

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let start_number: i32 = input.trim().parse().expect("Invalid input");
    
    // TODO: Write your code below
    // Use a loop expression to find the first number divisible by 7
    let mut counter = start_number;
    let result = loop {
    if counter % 7 == 0 {
        break counter;
    }
    counter += 1;
};
    
    // Print the result
    println!("{}", result);
}