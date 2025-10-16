# Picture of the Day CLI Tool

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A simple command-line interface (CLI) tool written in Rust to fetch and download the Bing Picture of the Day.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Project Architecture](#project-architecture)
- [Contributing](#contributing)
- [License](#license)

## Features

*   Downloads the daily Bing image from a specified market.
*   Allows custom output directory and filename for the downloaded image.
*   Lightweight and fast, built with Rust and `tokio`.

## Installation

To build and run this project, you need to have [Rust](https://www.rust-lang.org/tools/install) and Cargo installed.

1.  **Build the project:**
    ```bash
    cargo build --release
    ```

The executable will be located at `./target/release/potd-rs`.

## Usage

Run the compiled executable with various options:

```bash
./target/release/potd-rs bing [OPTIONS]
```

### Commands

- `bing`: Downloads the Bing Picture of the Day.

### Options

*   `-m, --market <MARKET>`: Market code (e.g., `en-ID`, `en-US`).
    *   Default: `en-ID`
*   `-d, --save-dir <SAVE_DIR>`: Directory to save the image.
    *   Default: `./` (current directory)
*   `-n, --image-name <IMAGE_NAME>`: Name of the saved image file.
    *   Default: `bing_potd.jpg`
*   `-h, --help`: Print help information.
*   `-V, --version`: Print version information.

### Examples

1.  **Download the image for the `en-US` market to the current directory with the default name:**
    ```bash
    ./target/release/potd-rs bing --market en-US
    ```

2.  **Save the image to `~/Pictures` with a custom name `my_bing_wallpaper.jpg`:**
    ```bash
    ./target/release/potd-rs bing --save-dir ~/Pictures --image-name my_bing_wallpaper.jpg
    ```

3.  **Combine options:**
    ```bash
    ./target/release/potd-rs bing -m en-GB -d /tmp -n bing_image.jpeg
    ```

## Project Architecture

The project follows a hexagonal architecture (also known as ports and adapters). This architecture separates the core application logic from the outside world (e.g., CLI, GUI, etc.).

- **`src/core`**: Contains the core application logic, including entities, use cases, and ports.
- **`src/adapters`**: Contains the adapters that connect the core application to the outside world. Currently, there is only a CLI adapter.

## Contributing

Contributions are welcome! Please feel free to submit a pull request.

### Development

To get started with development, you'll need to have Rust and Cargo installed. Then, you can clone the repository and build the project in debug mode:

```bash
git clone https://github.com/khhini/potd-rs.git
cd potd-rs
cargo build
```

### Adding a new provider

To add a new provider, you need to:

1.  Create a new module in `src/adapters/cli` for the new provider.
2.  Implement the `Command` trait for the new provider.
3.  Add the new provider to the `dispatch_command` function in `src/adapters/cli/mod.rs`.

## License

This project is licensed under the MIT License.
