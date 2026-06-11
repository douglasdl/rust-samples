use std::io;

// Function declaration
fn multiply(a: i32, b: i32) {
    println!("{}", a * b);
}

fn main() {
    let mut input_a = String::new();
    let mut input_b = String::new();

    io::stdin().read_line(&mut input_a).unwrap();
    io::stdin().read_line(&mut input_b).unwrap();

    let a: i32 = input_a.trim().parse().unwrap();
    let b: i32 = input_b.trim().parse().unwrap();
    // Call the function with a and b as arguments
    multiply(a, b);
}