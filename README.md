# Rust Todo CLI

A practical Rust project built to showcase core skills in:
- Rust syntax and ownership
- Structs, enums, and pattern matching
- File handling and JSON persistence
- Command-line interface design
- Basic error handling

This project is a simple but professional CLI todo app that lets you create, view, complete, and delete tasks from a JSON file.

## Why this project is useful

This is a strong portfolio project because it demonstrates that you can build a complete, real-world command-line tool in Rust without relying on a framework. It can be used as a foundation for more advanced apps later.

## Features

- Add tasks
- List all tasks
- Mark tasks as complete
- Delete tasks
- Clear all tasks
- Save data in a JSON file

## Project structure

```text
rust-todo-cli/
├── Cargo.toml
├── README.md
├── src/
│   └── main.rs
└── tasks.json
```

## Prerequisites

Install Rust from the official site:

- https://rustup.rs/

Then verify installation:

```bash
rustc --version
cargo --version
```

## Run the project

From the project root:

```bash
cargo run -- add "Buy groceries"
cargo run -- list
cargo run -- done 1
cargo run -- delete 1
cargo run -- clear
```

## Using a custom storage file

```bash
cargo run -- --file my-tasks.json add "Write project update"
cargo run -- --file my-tasks.json list
```

## Example output

```text
$ cargo run -- add "Plan portfolio update"
Added task #1: Plan portfolio update

$ cargo run -- list
[ ] 1. Plan portfolio update
```

## Future improvements

- Add task categories or priorities
- Add due dates
- Add a persistent config directory
- Add unit tests
- Build a GUI or web version later

## GitHub

Repository: https://github.com/celinasohal/rust-todo-cli
