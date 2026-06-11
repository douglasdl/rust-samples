use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
    // Write your code below
    for a in 1..n {
        for b in a..n {
            for c in b..n {
                if a + b + c == n {
                    println!("{} {} {}", a, b, c);
                }
            }
        }
    }
}