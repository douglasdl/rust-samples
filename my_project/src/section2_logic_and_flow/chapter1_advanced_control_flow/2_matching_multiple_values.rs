use std::io;

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let day: i32 = input.trim().parse().expect("Please enter a number");
    
    // TODO: Write your code below using match expression with | operator
    let day_of_the_week = match day {
      1 | 2 | 3 | 4 | 5 => "Weekday",
      6 | 7 => "Weekend",
      _ => "Invalid day",
    };

    println!("{}", day_of_the_week);
}