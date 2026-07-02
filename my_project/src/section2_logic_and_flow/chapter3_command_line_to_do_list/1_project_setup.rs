/*
Project Setup

challenge icon
Challenge

Easy
You will receive a single input: a comma-separated list of task descriptions. Create an empty mutable vector to store the tasks, split the input by commas, add each task to the vector, and print the total number of tasks followed by each task on a separate line.

Requirements:

Read the input containing comma-separated task descriptions (e.g., Buy groceries,Call dentist,Finish homework)
Create an empty mutable vector of type Vec<String>
Split the input string by commas to get individual tasks
Use .push() to add each task to the vector
Print the total number of tasks in the format: Total tasks: X
Print each task on a separate line in the format: Task: [task description]
Input:

A single line containing comma-separated task descriptions (e.g., Buy groceries,Call dentist,Finish homework)
Output:

First line: Total tasks: X where X is the number of tasks
Following lines: Each task printed as Task: [task description]
*/ 

use std::io;

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();
    
    // TODO: Write your code below
    // Create an empty mutable vector
    let mut tasks = Vec::new();

    // Split the input by commas
    let split_tasks = input.trim().split(',');
    
    // Add each task to the vector using .push()
    for task in split_tasks {
        tasks.push(task.to_string());
    }

    // Print the total number of tasks
    let total_tasks = tasks.len();
    println!("Total tasks: {}", total_tasks);

    // Print each task
    for task in tasks {
        println!("Task: {}", task);
    }
}