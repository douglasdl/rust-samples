use std::io;
use std::convert::TryInto;

fn main() {
    let mut input_str_arr = String::new();
    io::stdin().read_line(&mut input_str_arr).unwrap();
    let arr: [String; 5] = input_str_arr.split(',').map(String::from).collect::<Vec<String>>().try_into().unwrap();

    print!("[");
    // Write your code below
    for i in 0..5 {
        if i != arr.len() - 1 {
            print!("{}, ", arr[i]);
        } else {
            print!("{}", arr[i]);
        }
    }

    println!("]");
}