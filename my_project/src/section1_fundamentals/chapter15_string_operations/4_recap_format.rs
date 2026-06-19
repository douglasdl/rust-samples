use std::io;

fn main() {
    // Declare your final_password variable here
    let mut final_password;
    
    {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();
        // Add your code here to handle the first input
        final_password = format!("{input}");

    }
    
    {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();
        // Add your code here to handle the second input
        final_password = format!("{final_password}{input}");
    }
    
    {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();
        // Add your code here to handle the third input
        final_password = format!("{final_password}{input}");
    }

    println!("Generated password: {}", final_password);
}