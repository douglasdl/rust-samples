use std::io;

// Method declaration
fn sum_numbers() {
    // Complete Method
    let mut sum = 0;
    for i in 1..=1000 {
        sum += i;
    }
    println!("{}", sum);
}


fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
    for _ in 0..n {
        // Call the method n times
        sum_numbers();
    }
}