# Auto Clicker

A Rust-based autoclicker application with a simple GUI for automating left mouse button clicks.

## Features

- **Simple UI**: Clean interface built with egui
- **Adjustable Click Rate**: Set clicks per second (1-100 CPS)
- **Toggle Control**: Start and stop clicking with a single button
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

4. Click "Start Clicking" to begin auto-clicking

5. Click "Stop Clicking" to pause

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

This project is provided as-is for educational and personal use.
