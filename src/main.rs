use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct TodoStore {
    tasks: Vec<Task>,
}

impl TodoStore {
    fn load(file_path: &PathBuf) -> io::Result<Self> {
        if !file_path.exists() {
            return Ok(TodoStore::default());
        }

        let contents = fs::read_to_string(file_path)?;
        if contents.trim().is_empty() {
            return Ok(TodoStore::default());
        }

        match serde_json::from_str::<TodoStore>(&contents) {
            Ok(store) => Ok(store),
            Err(_) => Ok(TodoStore::default()),
        }
    }

    fn save(&self, file_path: &PathBuf) -> io::Result<()> {
        let json = serde_json::to_string_pretty(self).unwrap();
        fs::write(file_path, json)
    }

    fn add_task(&mut self, title: String) -> Task {
        let next_id = self
            .tasks
            .iter()
            .map(|task| task.id)
            .max()
            .unwrap_or(0)
            + 1;

        let task = Task {
            id: next_id,
            title,
            completed: false,
        };

        self.tasks.push(task.clone());
        task
    }

    fn complete_task(&mut self, task_id: u32) -> Result<String, String> {
        let task = self
            .tasks
            .iter_mut()
            .find(|task| task.id == task_id)
            .ok_or_else(|| format!("Task #{} not found.", task_id))?;

        if task.completed {
            return Ok(format!("Task #{} is already marked as complete.", task_id));
        }

        task.completed = true;
        Ok(format!("Marked task #{} as complete.", task_id))
    }

    fn delete_task(&mut self, task_id: u32) -> Result<String, String> {
        let initial_length = self.tasks.len();
        self.tasks.retain(|task| task.id != task_id);

        if self.tasks.len() == initial_length {
            return Err(format!("Task #{} not found.", task_id));
        }

        Ok(format!("Deleted task #{}.", task_id))
    }
}

fn print_help() {
    println!("Rust Todo CLI");
    println!("");
    println!("Usage:");
    println!("  cargo run -- add \"Buy milk\"");
    println!("  cargo run -- list");
    println!("  cargo run -- done 1");
    println!("  cargo run -- delete 1");
    println!("  cargo run -- clear");
    println!("  cargo run -- help");
    println!("");
    println!("Options:");
    println!("  --file <path>   Use a custom file for storing tasks (default: tasks.json)");
}

fn resolve_file_path(args: &[String]) -> Result<(PathBuf, Vec<String>), String> {
    let mut file_path = PathBuf::from("tasks.json");
    let mut positional_args = Vec::new();
    let mut index = 0;

    while index < args.len() {
        match args[index].as_str() {
            "--file" => {
                index += 1;
                if index >= args.len() {
                    return Err("Missing path after --file.".to_string());
                }
                file_path = PathBuf::from(&args[index]);
            }
            _ => positional_args.push(args[index].clone()),
        }
        index += 1;
    }

    Ok((file_path, positional_args))
}

fn print_tasks(tasks: &[Task]) {
    if tasks.is_empty() {
        println!("No tasks found.");
        return;
    }

    for task in tasks {
        let status = if task.completed { "x" } else { " " };
        println!("[{}] {}. {}", status, task.id, task.title);
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        print_help();
        return Ok(());
    }

    let (file_path, positional_args) = resolve_file_path(&args)?;

    if positional_args.is_empty() {
        print_help();
        return Ok(());
    }

    let command = positional_args[0].as_str();
    let mut store = TodoStore::load(&file_path)
        .map_err(|err| format!("Could not read tasks file '{}': {}", file_path.display(), err))?;

    match command {
        "add" => {
            let title = positional_args.get(1..).map(|parts| parts.join(" ")).unwrap_or_default();
            if title.trim().is_empty() {
                return Err("Please provide a task title.".to_string());
            }

            let task = store.add_task(title);
            store
                .save(&file_path)
                .map_err(|err| format!("Could not save tasks: {}", err))?;

            println!("Added task #{}: {}", task.id, task.title);
        }
        "list" => {
            print_tasks(&store.tasks);
        }
        "done" => {
            let task_id = positional_args
                .get(1)
                .ok_or_else(|| "Please provide a task ID.".to_string())?
                .parse::<u32>()
                .map_err(|_| "Task ID must be a number.".to_string())?;

            let message = store.complete_task(task_id)?;
            store
                .save(&file_path)
                .map_err(|err| format!("Could not save tasks: {}", err))?;
            println!("{}", message);
        }
        "delete" => {
            let task_id = positional_args
                .get(1)
                .ok_or_else(|| "Please provide a task ID.".to_string())?
                .parse::<u32>()
                .map_err(|_| "Task ID must be a number.".to_string())?;

            let message = store.delete_task(task_id)?;
            store
                .save(&file_path)
                .map_err(|err| format!("Could not save tasks: {}", err))?;
            println!("{}", message);
        }
        "clear" => {
            store.tasks.clear();
            store
                .save(&file_path)
                .map_err(|err| format!("Could not clear tasks: {}", err))?;
            println!("All tasks cleared.");
        }
        "help" | "--help" | "-h" => print_help(),
        _ => {
            println!("Unknown command: {}", command);
            print_help();
            process::exit(1);
        }
    }

    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {}", error);
        process::exit(1);
    }
}
