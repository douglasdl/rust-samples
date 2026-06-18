use std::io;
use std::convert::TryInto;

fn prod(arr: &[i32]) -> i32 {
    // Write your code below
    let mut product = 1;
    for i in 0..arr.len() {
        product *= arr[i];
    }
    return product;
}

fn main() {
    let mut input_str_arr = String::new();
    io::stdin().read_line(&mut input_str_arr).unwrap();
    let numbers: [i32; 8] = input_str_arr.trim().split(',').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>().try_into().unwrap();
    let result = prod(&numbers);
    println!("Product of array elements: {}", result);
}