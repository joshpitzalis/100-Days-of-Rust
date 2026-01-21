use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

fn main() {
    let mut tasks: Vec<Todo> = load_todos();

    loop {
        println!("Todo List Menu:");
        println!("1. Add Todo");
        println!("2. View Todos");
        println!("3. Mark Todo Complete");
        println!("4. Delete Todo");
        println!("5. Exit");

        let choice = get_user_choice();

        match choice {
            1 => add_todo(&mut tasks),
            2 => view_todos(&tasks),
            3 => mark_todo_completed(&mut tasks),
            4 => delete_todo(&mut tasks),
            5 => {
                save_todos(&tasks);
                println!("Todos saved successfully. Goodbye!");
                break;
            }
            _ => println!("Invalid choice"),
        }
    }
}

fn get_user_choice() -> u32 {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().parse().expect("Please type a number!")
}

fn load_todos() -> Vec<Todo> {
    let file =
        File::open("src/todos.json").unwrap_or_else(|_| File::create("src/todos.json").unwrap());
    serde_json::from_reader(file).unwrap_or_else(|_| Vec::new())
}

fn save_todos(tasks: &Vec<Todo>) {
    let file = File::create("src/todos.json").unwrap();
    serde_json::to_writer_pretty(file, tasks).unwrap();
}

fn add_todo(tasks: &mut Vec<Todo>) {
    println!("Enter the title of the todo:");
    let title = get_user_input();
    let id = tasks.len() as u32 + 1;
    let todo = Todo {
        id,
        title,
        completed: false,
    };
    tasks.push(todo);
}

fn view_todos(tasks: &Vec<Todo>) {
    if tasks.is_empty() {
        println!("No todos found");
        return;
    } else {
        println!();
        for task in tasks {
            let status = match task.completed {
                true => "O",
                false => "❌",
            };
            println!("{} - {} - {}", task.id, task.title, status);
        }
        println!();
    }
}

fn mark_todo_completed(tasks: &mut Vec<Todo>) {
    println!("Enter the ID of the todo to mark as completed:");
    let id = get_user_choice();

    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.completed = true;
    } else {
        println!("Todo not found");
    }
}

fn delete_todo(tasks: &mut Vec<Todo>) {
    println!("Enter the ID of the todo to delete:");
    let id = get_user_choice();

    if let Some(index) = tasks.iter().position(|t| t.id == id) {
        tasks.remove(index);
    } else {
        println!("Todo not found");
    }
}

fn get_user_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}
