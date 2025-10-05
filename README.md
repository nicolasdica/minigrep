# Minigrep

A simple grep-like command-line tool written in Rust that searches for text patterns in files.

## Features

- 🔍 Search for patterns in text files
- 📝 Case-sensitive and case-insensitive search modes
- ⚡ Fast and memory-efficient
- 🎯 Simple command-line interface using `clap`

## Installation

Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed on your system.

```bash
# Clone the repository
git clone git@github.com:yourusername/minigrep.git
cd minigrep

# Build the project
cargo build --release

# The binary will be in target/release/minigrep
```

## Usage

```bash
# Basic search (case-sensitive)
cargo run -- --query "pattern" --file-path poem.txt

# Short form
cargo run -- -q "pattern" -f poem.txt

# Case-insensitive search
cargo run -- --query "pattern" --file-path poem.txt --ignore-case
cargo run -- -q "pattern" -f poem.txt -i

# Get help
cargo run -- --help
```

### Command-line Options

- `-q, --query <QUERY>` - The pattern to search for (required)
- `-f, --file-path <FILE_PATH>` - The file to search in (required)
- `-i, --ignore-case` - Ignore case when searching (optional, default: false)

## Examples

Given a file `poem.txt`:
```text
Rust:
safe, fast, productive.
Pick three.
Trust me.
```

### Case-sensitive search
```bash
$ cargo run -- -q "rust" -f poem.txt
# No results (no lowercase "rust")

$ cargo run -- -q "Rust" -f poem.txt
Rust:
```

### Case-insensitive search
```bash
$ cargo run -- -q "rust" -f poem.txt -i
Rust:
Trust me.
```

## Running Tests

```bash
cargo test
```

## Project Structure

```
minigrep/
├── src/
│   ├── main.rs    # Entry point and CLI handling
│   └── lib.rs     # Search logic and functions
├── Cargo.toml     # Project dependencies
├── Cargo.lock     # Dependency lock file
└── README.md      # This file
```

## Dependencies

- [clap](https://crates.io/crates/clap) v4.5.48 - Command line argument parser

## Learning Resources

This project is inspired by the [Rust Programming Language Book](https://doc.rust-lang.org/book/ch12-00-an-io-project.html), Chapter 12.
