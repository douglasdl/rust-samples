use std::io;

fn main() {
    let mut age_input = String::new();
    let mut height_input = String::new();
    let mut adult_input = String::new();
    
    io::stdin().read_line(&mut age_input).unwrap();
    io::stdin().read_line(&mut height_input).unwrap();
    io::stdin().read_line(&mut adult_input).unwrap();
    
    let age: i32 = age_input.trim().parse().unwrap();
    let height: i32 = height_input.trim().parse().unwrap();
    let has_adult: bool = adult_input.trim().parse().unwrap();

    // Write your code below
    if (age >= 12) {
        if (height > 150) {
            if(age < 15) {
                if(has_adult) {
                    println!("You can ride with adult supervision!");
                } else {
                    println!("Sorry, you need an adult with you");
                }
            } else {
                println!("You can ride by yourself!");
            }
        } else {
            println!("Sorry, you're not tall enough");
        }
    } else {
        println!("Sorry, you're too young");
    }
}