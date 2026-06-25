use std::io;

fn main() {
    // Read the command from input
    let mut command = String::new();
    io::stdin().read_line(&mut command).expect("Failed to read line");
    let command = command.trim();
    
    // TODO: Write your code below
    // Use a match expression to handle different commands
    let cmd = match command {
      "start" => "System starting...",
      "stop" => "System stopping...",
      "pause" | "wait" => "System paused",
      "status" => "System running",
      _ => "Unknown command",
    };

    println!("{}", cmd);
}