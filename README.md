# Rust Todo CLI

<p align="center">
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust" />
  <img src="https://img.shields.io/badge/CLI-Tool-4EAA25?style=for-the-badge" alt="CLI Tool" />
  <img src="https://img.shields.io/badge/JSON-Storage-323330?style=for-the-badge" alt="JSON Storage" />
</p>

A practical Rust project designed as a portfolio piece to demonstrate core software engineering and Rust programming skills.

## Project overview

This project is a command-line todo manager built in Rust. It helps users manage tasks from the terminal, store them in a file, and keep a simple but useful workflow for daily task tracking.

The goal of this project is to showcase:
- Rust fundamentals and syntax
- Structs, enums, and pattern matching
- File I/O and JSON persistence
- CLI argument handling
- Error handling and user-friendly output

## Why this project is valuable

This is a strong portfolio project because it demonstrates that I can build a real, useful application in Rust rather than just learning syntax in isolation. It shows practical problem solving, clean code structure, and the ability to build a tool that could be extended into a larger application later.

## Features

- Add a new task
- View all tasks
- Mark a task as complete
- Delete a task
- Clear all tasks
- Save tasks to a local JSON file
- Use a custom storage path with `--file`

## Example usage

```bash
cargo run -- add "Write project update"
cargo run -- list
cargo run -- done 1
cargo run -- delete 1
cargo run -- clear
```

## Custom storage file

```bash
cargo run -- --file my-tasks.json add "Review networking notes"
cargo run -- --file my-tasks.json list
```

## Example output

```text
$ cargo run -- add "Plan portfolio update"
Added task #1: Plan portfolio update

$ cargo run -- list
[ ] 1. Plan portfolio update
```

## Project structure

```text
rust-todo-cli/
├── Cargo.toml
├── README.md
├── src/
│   └── main.rs
├── tasks.json
└── .gitignore
```

## Learning outcomes demonstrated

This project helps demonstrate understanding of:
- Rust ownership and borrowing
- Working with data structures
- Creating reusable logic for app features
- Reading and writing files
- Handling user input and commands
- Building a CLI from scratch

## Getting started

### Prerequisites

Install Rust with rustup:

- https://rustup.rs/

Then verify installation:

```bash
rustc --version
cargo --version
```

### Run locally

```bash
git clone https://github.com/celinasohal/rust-todo-cli.git
cd rust-todo-cli
cargo run -- list
```

## Future improvements

Possible extensions for this project include:
- Add task priorities
- Add categories or tags
- Add due dates
- Add tests for the CLI logic
- Save tasks in a dedicated app folder
- Build a more advanced task manager with filtering

## Repository

- GitHub: https://github.com/celinasohal/rust-todo-cli

## License

This project is licensed under the MIT License.
