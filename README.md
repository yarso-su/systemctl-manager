# Systemctl Manager - TUI 🔧

[![Crates.io](https://img.shields.io/crates/v/systemctl-manager.svg)](https://crates.io/crates/systemctl-manager)
[![License](https://img.shields.io/crates/l/systemctl-manager.svg)](LICENSE)
[![Downloads](https://img.shields.io/crates/d/systemctl-manager.svg)](https://crates.io/crates/systemctl-manager)

A simple utility to interact with systemd services through a TUI, featuring handy shortcuts to make service management easier (Vim-Keys).

![Preview](assets/preview.gif)

> [!NOTE]
> This binary is primarily a learning project. While it works for my use case, its main purpose is educational.

> [!WARNING]
> This project requires `systemd` to be installed on your system.

## Motivation

When I started using Linux, I got tired of typing the same commands repeatedly. As a newcomer, I created a small `bash` script with a hardcoded list of common services and a menu to run different commands.

This project is a re-implementation of that script in Rust, meant to help me learn more about the language and its ecosystem while also extending the original functionality.

## Installation

### Cargo

```bash
cargo install systemctl-manager
```

### AUR

```bash
yay -S systemctl-manager
```

## Usage

Use the `sm` binary to interact with systemd services. When you run the binary, a list of available services will be displayed with vim-like navigation.

> [!NOTE]
> Operations that modify services (start, stop, restart, enable, disable) require `sudo` privileges. The application will prompt for your password when needed.

**Key bindings:**
**Key bindings:**
- Navigate using arrow keys or vim-like keys (`j`/`k`)
- Press `i`/`a`/`I`/`A` to filter services by name
- Press `/` to search for text matches in the service list
- Press `w` to start the selected service
- Press `e` to stop the selected service
- Press `r` to reload the selected service
- Press `t` to restart the selected service
- Press `y` to enable the selected service
- Press `u` to disable the selected service
- Press `Ctrl+q` to quit the application

**Exiting alternative modes:**
- Press `Ctrl+c` or `Esc` to dismiss the current mode
- Press `Enter` to confirm the current mode

## Known Limitations

This is a custom implementation tailored to my own workflow, so there is plenty of room for improvement.

Feel free to fork the project, open issues, or contribute suggestions.

### Missing Features

- Support for multiple selections
- Should operation keybindings be moved to a secondary menu to make the default message bar clearer?

## Contributing

This is primarily a learning project, but contributions are welcome! Feel free to:

- Open issues for bugs or feature requests
- Submit pull requests
- Share feedback on the code

## Acknowledgments

Built with:
- [crossterm](https://github.com/crossterm-rs/crossterm) - Terminal manipulation
- [Rust](https://www.rust-lang.org/) - Because learning is fun

## License

MIT License
