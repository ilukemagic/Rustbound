# Rustbound

A command-line text adventure game built in Rust where you explore a mysterious world filled with treasures and challenges. Perfect for both gaming enthusiasts and those learning Rust programming fundamentals.

## Features

### Core Gameplay 🎮

- Explore diverse locations (mysterious forests, mountain peaks, treasure rooms)
- Player management system (movement, inventory, name customization)
- Interactive world with grid-based rooms containing items and NPCs
- Item interaction system (take, drop, use)
- NPC dialogue system
- Command parsing with comprehensive help
- Movement validation to prevent invalid actions

### Technical Highlights 🦀

- Robust error handling using Rust's Result and Option
- Modular architecture for easy expansion
- Clean code organization following Rust best practices
- Efficient state management for game elements

## Installation

### Windows Users

1. Download `Rustbound.exe` from the latest release
2. Double-click the executable to start your adventure

### Building from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/Rustbound.git

# Navigate to project directory
cd Rustbound

# Build the release version
cargo build --release

# Run the game
cargo run --release
```

## Available Commands

```text
Movement:
> north, south, east, west - Move in a direction
> look                     - Examine your surroundings

Inventory Management:
> take [item]             - Pick up an item
> drop [item]             - Drop an item
> use [item]              - Use an item
> inventory               - Check your inventory

Interaction:
> talk to [npc]           - Interact with characters
> name [new name]         - Change your character's name

System:
> help                    - Display help information
> quit                    - Exit the game
```

## Project Structure

```text
src/
├── main.rs       # Game loop and entry point
├── player.rs     # Player state management
├── world.rs      # World generation and room data
└── command.rs    # Command parsing system
```

## Game Tips 💡

- Explore thoroughly - each room may contain valuable items or important NPCs
- Talk to characters for valuable information and potential quests
- Some items have special effects in certain locations
- Use the 'look' command frequently to gather information about your surroundings

## Future Development 🚀

- Combat system implementation
- Puzzle-based NPC interactions
- Save/load game functionality
- Enhanced UI using tui-rs
- Additional locations and items

## Version

Current Version: 1.0.0

## Contributing

This project welcomes contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for development guidelines.

## Support

- Report bugs or suggest features through GitHub Issues
- Check the documentation for detailed information
- Join our community discussions

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

Built with 🦀 Rust and ❤️ for adventure gaming
