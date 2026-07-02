/*
Mutable Structs



By default, struct instances in Rust are immutable - once you create them, you cannot change their field values. However, just like with other variables, you can make a struct instance mutable by adding the mut keyword when declaring it.

When you declare a struct instance as mutable, you can modify any of its fields using dot notation:

let mut player = Player {
    name: String::from("Alice"),
    score: 0,
};

player.score = 100; // This works because player is mutable
Without the mut keyword, attempting to change a field would result in a compilation error. The mutability applies to the entire struct instance - you cannot make individual fields mutable while keeping others immutable.

Challenge

Easy
You will receive four inputs: a player name, an initial score (as an integer), a bonus score to add (as an integer), and a final score adjustment (as an integer). Define a Player struct with two fields: name of type String and score of type i32. Create a mutable instance of this struct, then modify the score field multiple times using the input values.

Requirements:

Define a Player struct with fields: name: String and score: i32
Read the first input as the player name
Read the second input and convert it to i32 for the initial score
Create a mutable instance of the Player struct with these values
Read the third input and convert it to i32 for the bonus score
Add the bonus score to the player's current score using dot notation
Read the fourth input and convert it to i32 for the final adjustment
Add the final adjustment to the player's current score
Print the player information in the exact format shown below
Input:

First line: Player name (e.g., Alice)
Second line: Initial score as an integer (e.g., 50)
Third line: Bonus score as an integer (e.g., 30)
Fourth line: Final adjustment as an integer (e.g., 20)
Output:

First line: Player: [name]
Second line: Final Score: [score]
*/ 

use std::io;

// TODO: Define the Player struct here with name and score fields
struct Player {
    name: String,
    score: i32
}

fn main() {
    // Read player name
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim().to_string();
    
    // Read initial score
    let mut initial_score = String::new();
    io::stdin().read_line(&mut initial_score).expect("Failed to read line");
    let initial_score: i32 = initial_score.trim().parse().expect("Invalid number");
    
    // Read bonus score
    let mut bonus = String::new();
    io::stdin().read_line(&mut bonus).expect("Failed to read line");
    let bonus: i32 = bonus.trim().parse().expect("Invalid number");
    
    // Read final adjustment
    let mut adjustment = String::new();
    io::stdin().read_line(&mut adjustment).expect("Failed to read line");
    let adjustment: i32 = adjustment.trim().parse().expect("Invalid number");
    
    // TODO: Create a mutable Player instance
    let mut player = Player {
        name: name,
        score: initial_score
    };
    
    // TODO: Add the bonus score to the player's score
    player.score += bonus;
    
    // TODO: Add the final adjustment to the player's score
    player.score += adjustment;
    
    // TODO: Print the output in the required format
    println!("Player: {}", player.name);
    println!("Final Score: {}", player.score);
}