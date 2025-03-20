# Developer Tools

A collection of handy tools for developers built with Rust and Slint.

[中文文档](README_zh.md)

## Features

### Timestamp Converter
- Convert Unix timestamp to human-readable date time
- Convert millisecond timestamp to human-readable date time
- Get current Unix timestamp
- Get current millisecond timestamp
- Support both input and output in a clear format

### JSON Formatter
- Format and prettify JSON text
- Easy-to-read output with proper indentation
- Support large JSON text processing with async handling
- Real-time formatting with progress indication
- Non-blocking UI during large JSON processing

## Getting Started

### Prerequisites
- Rust 1.75.0 or later
- Cargo package manager
- Windows 10/11 (for Windows builds)

### Installation

#### From Source
1. Clone the repository
```bash
git clone https://github.com/yourusername/dev-tool.git
cd dev-tool
```

2. Build the project
```bash
cargo build --release
```

3. Find the executable
- Windows: `target/release/dev-tool.exe`
- Linux/macOS: `target/release/dev-tool`

#### Direct Download
You can download the pre-built binaries from the [Releases](https://github.com/yourusername/dev-tool/releases) page.

## Usage

### Timestamp Converter
1. Enter a timestamp in the input field
2. Click "Unix时间戳转换" for Unix timestamp conversion
3. Click "毫秒时间戳转换" for millisecond timestamp conversion
4. Use "获取当前时间戳" to get the current timestamp
5. The converted result will appear in the output field

### JSON Formatter
1. Paste your JSON text into the input area
2. Click "格式化JSON" button
3. The formatted JSON will appear in the output area with proper indentation

## Project Structure

```
dev-tool/
├── src/
│   ├── gui/                # GUI related code
│   │   ├── components/     # UI components
│   │   │   ├── styled_button.slint    # Custom button component
│   │   │   ├── single_line_edit.slint # Single line input component
│   │   │   ├── timestamp_panel.slint  # Timestamp conversion panel
│   │   │   └── json_panel.slint       # JSON formatting panel
│   │   └── main.slint      # Main interface
│   ├── modules/            # Feature modules
│   │   ├── timestamp.rs    # Timestamp processing
│   │   ├── json_formatter.rs # JSON formatting
│   │   └── mod.rs         # Module exports
│   ├── handlers/          # Event handlers
│   │   ├── timestamp_handler.rs # Timestamp event handler
│   │   ├── json_handler.rs     # JSON event handler
│   │   └── mod.rs        # Handler exports
│   └── main.rs           # Program entry
├── resources/           # Application resources
│   └── app.manifest    # Windows manifest file
└── Cargo.toml         # Project configuration
```

## Development

### Building for Windows
The Windows build automatically configures:
- No console window when running
- Proper application manifest
- Windows-style GUI application

### Building for Linux/macOS
Standard Rust build process applies.

## Contributing

1. Fork the project
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE.txt) file for details. 