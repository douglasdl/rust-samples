use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let month: i32 = input.trim().parse().unwrap();
    // Write your code below
    let season = match month {
        12 | 1 | 2 => "Winter",
        3 | 4 | 5 => "Spring",
        6 | 7 | 8 => "Summer",
        9 | 10 | 11 => "Autumn",
        _ => "Invalid month",
    };

    // Don't change the line below
    println!("status = {}", season);
}