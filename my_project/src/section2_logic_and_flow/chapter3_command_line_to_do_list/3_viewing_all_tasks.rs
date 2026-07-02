use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the comma-separated tasks
    let tasks_input = lines.next().unwrap().unwrap();
    
    // Read the task number to highlight
    let task_number: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // TODO: Write your code below
    let mut tasks = Vec::new();

    // Split the tasks_input by commas and create a vector
    let split_tasks = tasks_input.trim().split(',');

    for task in split_tasks {
        tasks.push(task.to_string());
    }
    // Print the total number of tasks
    let total_tasks = tasks.len();
    println!("Total tasks: {}", total_tasks);

    // Iterate through the tasks and print each one with its number
    // Highlight the selected task by appending " (selected)"
    for (index, task) in tasks.iter().enumerate() {
        if index + 1 == task_number {
            println!("[{}] {} (selected)", index + 1, task);
        } else {
            println!("[{}] {}", index + 1, task);
        }
    }
}