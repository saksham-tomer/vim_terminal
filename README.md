# Vim-Style Terminal File Explorer

A fast and efficient terminal-based file explorer with vim-like keybindings, built in Rust.

## Features

- Vim-style navigation and commands
- Fast file and directory browsing
- Customizable keybindings
- Cross-platform support (Windows, macOS, Linux)

## Installation

Ensure you have Rust and Cargo installed on your system. Then run:

```
cargo install vim_terminal
```

## Usage

To start the file explorer, run:

```
vim_terminal [path]
```

If no path is provided, it will start in the current directory.

### Key Bindings

- `j` / `k`: Move cursor down / up
- `h` / `l`: Go to parent directory / Enter selected directory
- `gg` / `G`: Move to top / bottom of the file list
- `/`: Search for files
- `dd`: Delete selected file/directory
- `yy`: Copy selected file/directory
- `p`: Paste copied file/directory
- `i`: Show file/directory information
- `q`: Quit the application

## Dependencies

This project relies on the following crates:

- [tui](https://crates.io/crates/tui): A library for creating rich terminal user interfaces and dashboards.
- [crossterm](https://crates.io/crates/crossterm): A crossplatform terminal manipulation library in Rust.

## Building from Source

1. Clone the repository:

   ```
   git clone https://github.com/saksham-tomer/vim_terminal.git
   cd vim_terminal
   ```
2. Build the project:

   ```
   cargo build --release
   ```
3. Run the application:

   ```
   cargo run
   ```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

Saksham Tomer

[Saksham Tomar (@SakshamDevDose) / X](https://x.com/SakshamDevDose)

## Acknowledgments

- The Rust community for their excellent documentation and support.
- The creators and maintainers of the tui and crossterm crates.
