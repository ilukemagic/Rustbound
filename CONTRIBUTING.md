# Contributing Guide 🛠️

Welcome to Rustbound! We appreciate your interest in contributing. Here's how to get started:

## Development Setup

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/rustbound.git
   cd rustbound
   ```

2. Install Rust using [rustup](https://rustup.rs/)
3. Build the project:

   ```bash
   cargo build
   ```

## Code Style

- Follow Rustfmt standards:

  ```bash
  cargo fmt
  ```

- Run Clippy checks:

  ```bash
  cargo clippy
  ```

- Use explicit error handling (avoid `unwrap()`)
- Document public APIs with /// comments

## Architecture Overview

```text
src/
├── main.rs        # Game loop and input handling
├── player.rs      # Player state and inventory management
├── world.rs       # World generation and room connections
└── command.rs     # Command parsing and validation
```

## Implementing New Features

1. Create a feature branch:

   ```bash
   git checkout -b feat/new-command
   ```

2. Add tests for new functionality
3. Update documentation (README/help texts)
4. Submit a Pull Request with:
   - Description of changes
   - Testing evidence
   - Relevant documentation updates

## Common Contribution Areas

### Adding New Commands

1. Extend `Command` enum in `command.rs`
2. Implement parsing logic in `parse_command()`
3. Add handler in `main.rs` game loop
4. Update help documentation

### Creating New Room Types

1. Modify `world.rs` generation logic
2. Add descriptive text and interactions
3. Update navigation connections

### Implementing NPC Interactions

1. Extend `Room` struct in `world.rs`
2. Add dialogue system
3. Create interaction handlers

## Testing Guidelines

- Unit tests in module files (`#[cfg(test)]`)
- Integration tests in `/tests` directory
- Test all error cases and edge conditions
- Run tests with:

  ```bash
  cargo test
  ```

## Issue Reporting

When filing bugs:

1. Describe expected vs actual behavior
2. Include reproduction steps
3. Specify Rust version (`rustc --version`)
4. Add relevant game state screenshots

## Code of Conduct

Be respectful and constructive. We follow the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).
