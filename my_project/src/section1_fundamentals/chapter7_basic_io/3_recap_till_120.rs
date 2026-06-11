use std::io;

fn main() {
    // Write your code below
    let mut age = String::new();
    io::stdin().read_line(&mut age).unwrap();

    let age: i32 = age.trim().parse().unwrap();

    println!("{} years till 120", 120 - age);
}