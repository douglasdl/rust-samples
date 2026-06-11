use std::io;

fn sigma(n: i32) -> i32 {
    // Write your code below
    let mut sum = 0;
    for i in 1..=n {
        sum += i;
    }  
    return sum;
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
    let res = sigma(n);
    println!("{}", res);
}