use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let count: i32 = input.trim().parse().unwrap();
    // Write your code below
    let mut sum = 0;

    for _ in 0..count {
        input.clear();

        io::stdin().read_line(&mut input).unwrap();
        let num: i32 = input.trim().parse().unwrap();

        sum += num;
    }

    println!("{}", sum);

}