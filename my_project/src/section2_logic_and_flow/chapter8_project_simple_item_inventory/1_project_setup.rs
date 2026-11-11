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
    // Ler a entrada
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();
    
    // TODO: Escreva seu código abaixo
    // Criar um vetor mutável vazio
    let mut tasks: Vec<String> = Vec::new();

    // Dividir a entrada por vírgulas
    // Adicionar cada tarefa ao vetor usando .push()
    for task in input.split(',') {
        tasks.push(task.to_string());
    }
    
    // Imprimir o número total de tarefas
    println!("Total tasks: {}", tasks.len());

    // Imprimir cada tarefa
    for task in tasks {
        println!("Task: {}", task);
    }
}