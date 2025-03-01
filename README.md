# CLI JIRA Application

A simple command-line task management tool inspired by JIRA, built with Rust.

## Features

- Create tickets with title, description, and optional assignee
- List all tickets
- View details of a specific ticket
- Update ticket properties (title, description, status, assignee)
- Persistent storage of tickets in JSON format

## Installation

1. Make sure you have Rust and Cargo installed. If not, install from [https://rustup.rs/](https://rustup.rs/)
2. Clone this repository
3. Build the application:

```bash
cargo build --release
```

## Usage

### Create a new ticket

```bash
cargo run -- create --title "Fix login bug" --description "Users can't log in with correct credentials" --assignee "John"
```

### List all tickets

```bash
cargo run -- list
```

### View a specific ticket

```bash
cargo run -- view <TICKET_ID>
```

### Update a ticket

```bash
cargo run -- update <TICKET_ID> --title "New title" --description "New description" --status "PROGRESS" --assignee "Jane"
```

## Ticket Status Options

Tickets can have one of the following status values:

- `TODO`: Work has not yet started
- `PROGRESS`: Work is currently in progress
- `DONE`: Work has been completed

## Data Storage

All tickets are stored in the `data/tickets.json` file in the project directory.

## Learning Rust

This project demonstrates several Rust concepts:

- Enums and pattern matching
- Structs and implementation blocks
- Error handling with Result
- Command-line argument parsing with clap
- File I/O and JSON serialization/deserialization
- Working with dates and times
- Option types for optional values
