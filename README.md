# Developer Tools

A collection of handy tools for developers built with Rust and Slint.

[中文文档](README_zh.md)

## Features

### Timestamp Converter
- Convert Unix timestamp to human-readable date time
- Convert millisecond timestamp to human-readable date time
- Get current Unix timestamp
- Get current millisecond timestamp

### JSON Formatter
- Format and prettify JSON text
- Easy-to-read output with proper indentation

## Getting Started

### Prerequisites
- Rust 1.75.0 or later
- Cargo package manager

### Installation

1. Clone the repository
```bash
git clone https://github.com/yourusername/dev-tool.git
cd dev-tool
```

2. Build the project
```bash
cargo build --release
```

3. Run the application
```bash
cargo run --release
```

## Project Structure

```
dev-tool/
├── src/
│   ├── gui/
│   │   ├── components/
│   │   │   ├── styled_button.slint
│   │   │   ├── single_line_edit.slint
│   │   │   ├── timestamp_panel.slint
│   │   │   └── json_panel.slint
│   │   └── main.slint
│   ├── modules/
│   │   ├── timestamp.rs
│   │   ├── json_formatter.rs
│   │   └── mod.rs
│   ├── handlers/
│   │   ├── timestamp_handler.rs
│   │   ├── json_handler.rs
│   │   └── mod.rs
│   └── main.rs
└── Cargo.toml
```

## Contributing

1. Fork the project
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details. 