use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut number: f64 = input.trim().parse().unwrap();
    // Write your code below
    
    while number >= 3.5 {
        number /= 2.0;
    }
    println!("{}", number);
    
}