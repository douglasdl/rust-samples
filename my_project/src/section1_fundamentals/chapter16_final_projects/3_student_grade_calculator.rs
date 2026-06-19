use std::io;
use std::convert::TryInto;


fn calculate_average_grade(grades: [i32; 8]) -> String {
    // Write your code below
    let mut sum = 0;
    for grade in &grades {
        sum += *grade;
    }

    let average = sum as f64 / grades.len() as f64;

    let letter_grade = if average >= 90.0 {
        "A"
    } else if average >= 80.0 {
        "B"
    } else if average >= 70.0 {
        "C"
    } else if average >= 60.0 {
        "D"
    } else {
        "F"
    };

    format!("Average grade: {:.2} - {}", average, letter_grade)
    
}
fn main() {
    let mut input_str_arr = String::new();
    io::stdin().read_line(&mut input_str_arr).unwrap();
    let arr: [i32; 8] = input_str_arr.trim().split(',').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>().try_into().unwrap();
    let res = calculate_average_grade(arr);
    println!("{}", res);
}