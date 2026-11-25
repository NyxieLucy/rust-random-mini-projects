# Rust Random Mini Projects

A collection of small Rust projects created as I learn the language, practice common programming concepts, and build practical CLI applications.

> This repository documents the journey of learning Rust one mini project at a time.

## Projects

| Project | Description | Main features |
| --- | --- | --- |
| [`dashboard`](./dashboard) | A terminal dashboard for tracking tasks and notes | Task management, note creation and storage, JSON-backed data |
| [`quote_board`](./quote_board) | A simple quote board application | Add, display, and manage quotes from the terminal |
| [`salary_management`](./salary_management) | A small salary and account management system | Account records, salary calculations, JSON-backed data |
| [`to_do_list`](./to_do_list) | A command-line to-do list | Create and manage tasks, persistent JSON storage |

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) with Cargo
- A terminal

## Running a Project

Each project is a separate Cargo package. Change into a project directory and run:

```bash
cargo run
```

For example:

```bash
cd dashboard
cargo run
```

Repeat the command from `quote_board`, `salary_management`, or `to_do_list` to run another project.

To check a project without running it:

```bash
cargo check
```

To format its Rust source:

```bash
cargo fmt
```

## Repository Structure

```text
.
├── dashboard/
├── quote_board/
├── salary_management/
└── to_do_list/
```

## Learning Goals

These projects are intended to practice:

- Rust ownership and borrowing
- Structs and enums
- Modules and project organization
- Error handling
- Command-line interaction
- Reading and writing JSON data
- Working with external Cargo crates

## Contributing

This is primarily a personal learning repository, but suggestions and improvements are welcome. Feel free to open an issue or submit a pull request.
