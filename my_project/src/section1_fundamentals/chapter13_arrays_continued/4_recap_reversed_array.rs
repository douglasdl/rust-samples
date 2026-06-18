use std::io;
use std::convert::TryInto;

fn rev_arr(arr: &[i32]) -> [i32; 8] {
    let mut result = [0; 8];

    for i in 0..8 {
        result[i] = arr[7 - i];
    }

    return result;
}

fn main() {
    let mut input_str_arr = String::new();
    io::stdin().read_line(&mut input_str_arr).unwrap();
    let numbers: [i32; 8] = input_str_arr.trim().split(',').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>().try_into().unwrap();
    let result = rev_arr(&numbers);
    println!("The reversed array is: {:?}", result);
}