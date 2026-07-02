use std::io;

fn main() {
    // Read the first input (comma-separated tasks)
    let mut existing_tasks = String::new();
    io::stdin().read_line(&mut existing_tasks).expect("Failed to read line");
    let existing_tasks = existing_tasks.trim();
    
    // Read the second input (new task to add)
    let mut new_task = String::new();
    io::stdin().read_line(&mut new_task).expect("Failed to read line");
    let new_task = new_task.trim();
    
    // TODO: Write your code below
    // Split existing_tasks by commas and create a mutable vector
    let mut tasks = Vec::new();
    let split_tasks = existing_tasks.trim().split(',');
    for task in split_tasks {
        tasks.push(task.to_string());
    }

    // Add the new task to the vector
    tasks.push(new_task.to_string());

    // Print the total number of tasks
    let total_tasks = tasks.len();
    println!("Total tasks: {}", total_tasks);

    // Print each task in the required format
    for task in tasks {
        println!("Task: {}", task);
    }
    
}