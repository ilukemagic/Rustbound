# Rustbound

A command-line text adventure game for learning Rust fundamentals.

## Features

- **Core Gameplay**

  - Player management (movement, inventory)
  - World with grid-based rooms containing items/NPCs
  - Command parsing: `go <direction>`, `take <item>`, `talk to <npc>`, `quit`
  - Robust error handling

- **Learning Objectives** 🦀

  - Practice Rust basics: ownership, borrowing, enums, pattern matching
  - Implement common data structures
  - Handle errors with `Result` and `Option`
  - Modular code organization

- **Technical Implementation**
  - Modular architecture:
    - `player.rs`: Player state and inventory
    - `world.rs`: Room generation and navigation
    - `command.rs`: Input parsing and validation
    - `main.rs`: Game loop and UI
  - Expandable system for adding new commands/features

## Getting Started

```bash
git clone https://github.com/yourusername/rustbound.git
cd rustbound
cargo build
cargo run
```

## Example Commands

```text
> go north
> take sword
> talk to old_man
> quit
```

## Project Structure

```text
src/
├── main.rs # Game loop and entry point
├── player.rs # Player state management
├── world.rs # World generation and room data
└── command.rs # Command parsing system
```

## Roadmap

- [ ] Add combat system (`fight` command)
- [ ] Implement puzzle system for NPC interactions
- [ ] Add save/load functionality
- [ ] Create TUI interface using `tui-rs`

## Contributing

This project welcomes contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for development guidelines.
