use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let wind: i32 = input.trim().parse().unwrap();
    // Replace the line below with an if-else expression that assigns status based on wind
    let status = if wind < 8 {
        "Calm"
    } else if wind >= 8 && wind <= 31 {
        "Breeze"
    } else if wind >= 32 && wind <= 63 {
        "Gale"
    } else {
        "Storm"
    };
    
    // Don't change the line below
    println!("status = {}", status);
}