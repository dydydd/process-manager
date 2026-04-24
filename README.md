# Process Manager

A cross-platform CLI process manager written in Rust (edition 2024).

## Features

- **ps** — List all running processes with filtering, sorting, and limit
- **kill** — Terminate a process by PID
- **top** — Real-time process monitor with auto-refresh
- **tree** — Visual process tree view
- **info** — Detailed information about a specific process

## Installation

```bash
cargo install --path .
```

## Usage

```bash
# List all processes
pm ps

# Filter by name and sort by memory
pm ps -f chrome -s memory

# Show top 10 processes by CPU
pm ps -s cpu -l 10

# Kill a process
pm kill <PID>

# Force kill
pm kill <PID> --force

# Real-time monitor (refresh every 2s)
pm top

# Real-time monitor with 1s interval, sorted by memory
pm top -i 1 -s memory

# Show process tree
pm tree

# Filter process tree
pm tree -f chrome

# Show process details
pm info <PID>
```

## Options

| Command | Flag | Description |
|---------|------|-------------|
| `ps` | `-f, --filter` | Filter by process name (case-insensitive) |
| `ps` | `-s, --sort` | Sort by: pid, name, cpu, memory, status |
| `ps` | `-l, --limit` | Show only top N processes |
| `kill` | `--force` | Force kill |
| `top` | `-i, --interval` | Refresh interval in seconds |
| `top` | `-s, --sort` | Sort by: pid, name, cpu, memory |
| `tree` | `-f, --filter` | Filter by process name |

## Build

```bash
cargo build --release
```

Binary: `target/release/process-manager`
