use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write}; // Removed the unused import for std::io

const TASKS_FILE: &str = "tasks.txt";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "add" => add_task(args),
        "list" => list_tasks(),
        "complete" => complete_task(args),
        "help" => print_usage(),
        _ => println!("Invalid command. Use 'add', 'list', 'complete', or 'help'."),
    }
}

fn print_usage() {
    println!("To-Do List CLI App in Rust");
    println!("Usage:");
    println!("  Add task:    todo add 'Task description'");
    println!("  List tasks:  todo list");
    println!("  Complete task: todo complete <task_number>");
}

// fn add_task(args: Vec<String>) {
//     if args.len() < 3 {
//         println!("Usage: todo add 'Task description'");
//         return;
//     }

//     let task_description = &args[2..].join(" ");
//     if let Ok(mut file) = File::create(TASKS_FILE) {
//         if let Err(err) = writeln!(file, "{}", task_description) {
//             eprintln!("Error writing to file: {}", err);
//         } else {
//             println!("Task added: {}", task_description);
//         }
//     } else {
//         eprintln!("Error creating file '{}'", TASKS_FILE);
//     }
// }

fn add_task(args: Vec<String>) {
    if args.len() < 3 {
        println!("Usage: todo add 'Task description'");
        return;
    }

    let task_description = &args[2..].join(" ");

    if let Ok(mut file) = OpenOptions::new()
        .write(true)
        .append(true) // Append mode
        .open(TASKS_FILE)
    {
        if let Err(err) = writeln!(file, "{}", task_description) {
            eprintln!("Error writing to file: {}", err);
        } else {
            println!("Task added: {}", task_description);
        }
    } else {
        eprintln!("Error opening file '{}'", TASKS_FILE);
    }
}


fn list_tasks() {
    match File::open(TASKS_FILE) {
        Ok(file) => {
            let reader = BufReader::new(file);
            for (i, line) in reader.lines().enumerate() {
                if let Ok(task) = line {
                    println!("{}. {}", i + 1, task);
                }
            }
        }
        Err(_) => {
            println!("No tasks found.");
        }
    }
}
fn complete_task(args: Vec<String>) {
    if args.len() < 3 {
        println!("Usage: todo complete <task_number>");
        return;
    }

    let task_number: usize = match args[2].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid task number.");
            return;
        }
    };

    let lines: Vec<_> = match File::open(TASKS_FILE) {
        Ok(file) => BufReader::new(file)
            .lines()
            .collect::<Result<Vec<_>, _>>()
            .unwrap_or_else(|_| {
                println!("Error reading tasks.");
                vec![] // Return an empty Vec on error
            }),
        Err(_) => {
            println!("No tasks found.");
            return;
        }
    };

    if task_number > 0 && task_number <= lines.len() {
        let completed_task = lines
            .get(task_number - 1)
            .cloned()
            .unwrap_or_else(|| "".to_string());

        if !completed_task.is_empty() {
            println!("Completed task: {}", completed_task);

            let updated_tasks: Vec<_> = lines
                .iter()
                .enumerate()
                .filter_map(|(i, line)| if i != task_number - 1 { Some(line) } else { None })
                .collect();

            if let Ok(mut file) = File::create(TASKS_FILE) {
                for task in updated_tasks {
                    if let Err(err) = writeln!(file, "{}", task) {
                        eprintln!("Error writing to file: {}", err);
                        return;
                    }
                }
            } else {
                eprintln!("Error creating file '{}'", TASKS_FILE);
            }
        } else {
            println!("Task number out of range.");
        }
    } else {
        println!("Task number out of range.");
    }
}
