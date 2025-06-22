# Picture of the Day CLI Tool

A simple command-line interface (CLI) tool written in Rust to fetch and download the Bing Picture of the Day.

## Features

*   Downloads the daily Bing image from a specified market.
*   Allows custom output directory and filename for the downloaded image.
*   Lightweight and fast, built with Rust and `tokio`.

## Installation

To build and run this project, you need to have [Rust](https://www.rust-lang.org/tools/install) and Cargo installed.

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/khhini/potd-rs.git # Replace with actual repo URL if available
    cd potd-rs
    ```
2.  **Build the project:**
    ```bash
    cargo build --release
    ```

The executable will be located at `./target/release/potd_cli` (or `potd_cli.exe` on Windows).

## Usage

Run the compiled executable with various options:

```bash
./target/release/potd_cli [OPTIONS]
```

### Options:

*   `-m, --market <MARKET>`: Market code (e.g., `en-ID`, `en-US`).
    *   Default: `en-ID`
*   `-s, --save-dir <SAVE_DIR>`: Directory to save the image.
    *   Default: `./` (current directory)
*   `-i, --image-name <IMAGE_NAME>`: Name of the saved image file.
    *   Default: `bing_potd.jpg`
*   `-h, --help`: Print help information.
*   `-V, --version`: Print version information.

### Examples:

1.  **Download the image for the `en-US` market to the current directory with the default name:**
    ```bash
    ./target/release/potd_cli --market en-US
    ```

2.  **Save the image to `~/Pictures` with a custom name `my_bing_wallpaper.jpg`:**
    ```bash
    ./target/release/potd_cli --save-dir ~/Pictures --image-name my_bing_wallpaper.jpg
    ```

3.  **Combine options:**
    ```bash
    ./target/release/potd_cli -m en-GB -s /tmp -i bing_image.jpeg
    ```


