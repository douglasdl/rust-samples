use std::io;

fn convert_and_print(s: &str, n: f64, to_string: bool) {
    // Write your code here
    if to_string == true {
        let number_str = n.to_string();
        let count = number_str.chars().filter(|c| c.is_ascii_digit()).count();
        println!("Number: {}, Digits: {}", number_str, count);

    } else {
        let num_s: f64 = s.parse().unwrap();
        let num_i = num_s as i32;
        println!("String as number: {}", num_i);
    }
}

fn main() {
    let mut input_number_str = String::new();
    let mut input_n = String::new();

    io::stdin().read_line(&mut input_number_str).unwrap();
    io::stdin().read_line(&mut input_n).unwrap();

    let n: f64 = input_n.trim().parse().unwrap();
    let number_str = input_number_str.trim();

    // Call convert_and_print with to_string = false
    convert_and_print(number_str, n, false);
    
    // Call convert_and_print with to_string = true
    convert_and_print(number_str, n, true);
}