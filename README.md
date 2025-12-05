# Systemctl Manager (TUI)

A simple utility to interact with systemd services through a TUI, featuring handy shortcuts to make service management easier.

> [!NOTE]  
> This binary is more of a conceptual/learning project. While it works for my own use case, I developed it primarily for learning purposes.

> [!WARNING]  
> This project requires `systemd` to be installed on your system.

## Motivation

When I started using Linux, I initially used the `systemctl` command directly to interact with systemd services. Over time, I got tired of typing the same commands repeatedly. As a newbie, I created a small `bash` script with my most common services hardcoded and a menu to select different commands.

The goal is to re-implement this script in Rust to learn more about the language and its ecosystem while extending the original functionality.

## Usage

Use the `sm` binary to interact with systemd services. When you run the binary, a list of available services will be displayed with vim-like navigation.

**Key bindings:**
- Navigate the list using arrow keys or vim-like keys (j/k)
- Press `i` to enter search mode and filter services by name
- Press `Enter` to select a service
- Press `Esc` to exit search mode

**Alternative ways to exit:**
- Press `Ctrl+C` to quit the application

## Technologies

This is a simple TUI written in Rust using:
- `clap` for argument parsing
- `crossterm` for terminal interaction

## Known Limitations

This is a custom implementation designed to suit specific requirements, so there are many areas for improvement.

Feel free to fork the project, create issues, or suggest improvements!

## License

MIT License
