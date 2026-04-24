# Process Manager

An interactive htop-style terminal process manager written in Rust (edition 2024).

## Preview

```
┌─ Process Manager ─────────────────────────────────────────────────────┐
│ Tasks: 287 | CPU: 12.3% | Mem: 64.2% | Sort: CPU% │                    │
├─ Processes ───────────────────────────────────────────────────────────┤
│ PID   User        Status   CPU%    MEM        Name                    │
│ 1234  LiDream     R       15.2    389.9M     chrome.exe          >>  │
│ 5678  LiDream     S        8.1    126.1M     code.exe                │
│ 9012  SYSTEM      S        3.4     63.6M     svchost.exe             │
│ ...                                                                       │
├─ Process Info ────────────────────────────────────────────────────────┤
│ PID: 1234 | PPID: 25252 | User: LiDream | CPU: 15.2% | MEM: 389.9M  │
└───────────────────────────────────────────────────────────────────────┘
```

## Features

- Real-time process list with auto-refresh (1s interval)
- Sort by CPU%, Memory, Name, PID, or User
- Search/filter processes
- Kill selected process
- Color-coded rows with selection highlight
- Process detail footer

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `↑` / `k` | Move selection up |
| `↓` / `j` | Move selection down |
| `PageUp` / `PageDown` | Scroll by page |
| `Home` / `End` | Jump to top / bottom |
| `F5` | Refresh process list |
| `F6` | Cycle sort field (CPU → MEM → NAME → PID → USER) |
| `F3` / `/` | Toggle search |
| `F9` / `K` | Kill selected process |
| `q` | Quit |

## Build

```bash
cargo build --release
```

Binary: `target/release/process-manager`

## Run

```bash
cargo run --release
```

## Dependencies

- [ratatui](https://github.com/ratatui/ratatui) — Terminal UI framework
- [crossterm](https://github.com/crossterm-rs/crossterm) — Terminal manipulation
- [sysinfo](https://github.com/GuillaumeGomez/sysinfo) — System process information
