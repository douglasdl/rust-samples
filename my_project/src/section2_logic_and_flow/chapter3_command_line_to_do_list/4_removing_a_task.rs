use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the comma-separated tasks
    let tasks_input = lines.next().unwrap().unwrap();
    
    // Read the task number to remove (1-based indexing)
    let task_to_remove = lines.next().unwrap().unwrap();
    let task_number: usize = task_to_remove.trim().parse().unwrap();
    
    // Read the new task to add
    let new_task = lines.next().unwrap().unwrap();
    
    // TODO: Write your code below
    let mut tasks = Vec::new();

    // Split the tasks_input by commas and create a mutable vector
    let split_tasks = tasks_input.trim().split(',');
    for task in split_tasks {
        tasks.push(task.to_string());
    }

    // Remove the task at the specified index (convert 1-based to 0-based)
    tasks.remove(task_number - 1);
    
    // Add the new task to the vector
    tasks.push(new_task.to_string());
    
    // Output the results
    // Print total tasks in format: Total tasks: X
    let total_tasks = tasks.len();
    println!("Total tasks: {}", total_tasks);

    // Print each task in format: Task: [task description]
    for task in tasks {
        println!("Task: {}", task);
    }
}





