use std::io;
use std::convert::TryInto;

fn main() {
    let mut input_str_arr = String::new();
    let mut input_index = String::new();
    let mut new_element = String::new();
    io::stdin().read_line(&mut input_str_arr).unwrap();
    io::stdin().read_line(&mut input_index).unwrap();
    io::stdin().read_line(&mut new_element).unwrap();

    let new_element = new_element.trim().to_string();
    let index: usize = input_index.trim().parse().unwrap();
    let mut arr: [String; 5] = input_str_arr.split(',').map(String::from).collect::<Vec<String>>().try_into().unwrap();
    
    // Write your code below
    // Use arr, index and new_element to solve the challenge    
    arr[index] = new_element;


    for i in 0..arr.len() {
        print!("{} ", arr[i]);
    }
}