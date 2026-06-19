use std::io;

fn main() {
    let mut input_str_arr_1 = String::new();
    let mut input_str_arr_2 = String::new();
    io::stdin().read_line(&mut input_str_arr_1).unwrap();
    io::stdin().read_line(&mut input_str_arr_2).unwrap();
    let arr1: Vec<String> = input_str_arr_1.trim().split(',').map(String::from).collect();
    let arr2: Vec<String> = input_str_arr_2.trim().split(',').map(String::from).collect();
    
    let mut result = false;
    // Write your code below
    let str1 = arr1.join("");
    let str2 = arr2.join("");
    
    result = str1.contains(&str2);

    println!("{}", result);
}