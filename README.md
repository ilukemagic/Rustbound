# Rustbound

A command-line text adventure game for learning Rust fundamentals.

## Features

- **Core Gameplay**

  - Player management (movement, inventory, name customization)
  - World with grid-based rooms containing items/NPCs
  - Item interaction (take, drop, use)
  - NPC dialogue system
  - Command parsing with comprehensive help system
  - Robust error handling

- **Learning Objectives** 🦀

  - Practice Rust basics: ownership, borrowing, enums, pattern matching
  - Implement common data structures
  - Handle errors with `Result` and `Option`
  - Modular code organization

- **Technical Implementation**
  - Modular architecture:
    - `player.rs`: Player state and inventory management
    - `world.rs`: Room generation and item/NPC interactions
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

## Available Commands

```text
> go [direction]    - Move in a direction (north, south, east, west)
> take [item]       - Pick up an item from the current room
> drop [item]       - Drop an item from your inventory
> use [item]        - Use an item from your inventory
> talk to [npc]     - Talk to a character in the current room
> inventory         - Check your inventory
> name [new name]   - Change your character's name
> help              - Display help information
> quit              - Exit the game
```

## Project Structure

```text
src/
├── main.rs       # Game loop and entry point
├── player.rs     # Player state management
├── world.rs      # World generation and room data
└── command.rs    # Command parsing system
```

## Game Tips

- Explore different rooms by using 'go' in various directions
- Collect items with 'take' and use them with 'use'
- Talk to characters to get information and quests
- Some items have special effects when used in certain locations

## Roadmap

- [ ] Add combat system (`fight` command)
- [ ] Implement puzzle system for NPC interactions
- [ ] Add save/load functionality
- [ ] Create TUI interface using `tui-rs`

## Contributing

This project welcomes contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for development guidelines.
