# Auto Clicker

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.0+-orange.svg)
![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg)
![GitHub release](https://img.shields.io/github/v/release/rynmon/autoclicker?include_prereleases)

A Rust-based autoclicker application with a simple GUI for automating left mouse button clicks.

## Features

- **Simple UI**: Clean interface built with egui
- **Adjustable Click Rate**: Set clicks per second (1-100 CPS)
- **Toggle Control**: Start and stop clicking with a single button
- **Global Hotkey**: Press F6 anywhere to stop clicking (works even when window isn't focused)
- **Custom Icon**: Professional app icon embedded in executable
- **Window Icon**: Icon appears in title bar and taskbar
- **Status Indicator**: Visual feedback showing active/inactive state
- **Cross-Platform**: Works on Windows, macOS, and Linux

## Requirements

- Rust (latest stable version)
- Cargo (comes with Rust)

## Installation

1. Make sure you have Rust installed. If not, install it from [rustup.rs](https://rustup.rs/)

2. Clone or download this repository

3. Build the project:
```bash
cargo build --release
```

## Usage

1. Run the application:
```bash
cargo run --release
```

2. The application window will open with:
   - A status indicator (ACTIVE/INACTIVE)
   - A slider to adjust clicks per second (1-100)
   - A toggle button to start/stop clicking

3. Position your mouse cursor where you want to click

4. Click "Start Clicking" to begin auto-clicking (1 second delay before clicking starts)

5. Click "Stop Clicking" to pause, or press **F6** to stop clicking from anywhere

## Building

### Debug Build
```bash
cargo build
```

### Release Build (Recommended)
```bash
cargo build --release
```

The executable will be located at `target/release/autoclicker.exe` (Windows) or `target/release/autoclicker` (Linux/macOS).

## Notes

- The clicker will click at the current mouse position when activated
- Adjust the CPS slider to control how fast the clicks occur
- The application runs in a separate thread, so the UI remains responsive
- On macOS, you may need to grant Accessibility permissions for the app to simulate mouse clicks

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
