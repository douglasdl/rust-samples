use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the initial comma-separated tasks
    let initial_tasks = lines.next().unwrap().unwrap();
    
    // Read the new task to add
    let new_task = lines.next().unwrap().unwrap();
    
    // Read the task number to view (1-based)
    let view_number: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read the task number to remove (1-based)
    let remove_number: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // TODO: Write your code below
    let mut tasks = Vec::new();
    
    // 1. Split initial_tasks by commas and create a mutable vector
    let split_tasks = initial_tasks.trim().split(',');
    for task in split_tasks {
        tasks.push(task.to_string());
    }

    // 2. Add the new_task to the vector
    tasks.push(new_task.to_string());

    // 3. Remove the task at remove_number (convert to 0-based index)
    tasks.remove(remove_number - 1);

    // 4. Print the total tasks and viewing task information
    let total_tasks = tasks.len();
    println!("Total tasks: {}", total_tasks);
    println!("Viewing task: {}", view_number);

    // 5. Iterate through the final list and print each task
    let adjusted_view = if remove_number < view_number {
    view_number - 1
    } else if remove_number == view_number {
        0
    } else {
        view_number
    };
    
    for (index, task) in tasks.iter().enumerate() {
        if index + 1 == view_number {
            println!("[{}] {} (selected)", index + 1, task);
        } else {
            println!("[{}] {}", index + 1, task);
        }
    }
}