use std::io;

fn main() {
    let mut input_n = String::new();
    io::stdin().read_line(&mut input_n).unwrap();
    let n: usize = input_n.trim().parse().unwrap();

    for i in 1..=n {
        if(i % 2 == 1) {
            println!("{}", "*".repeat(i));
        }
    }
}