use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let limit: i32 = input.trim().parse().unwrap();

    let mut counter = 0;
    // Write your code below
    loop {
        counter += 1;
        println!("Count: {}", counter);

        if counter == limit {
            break;
        }
    }

}