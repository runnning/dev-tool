# Dev Tool

[简体中文](README_zh.md) | English

A collection of development tools built with Rust and Slint.

## Features

- Time Tool
  - Get current time
  - Convert datetime to timestamp (seconds and milliseconds)
  - Support multiple time formats
- JSON Tool
  - Format JSON
  - Minify JSON
  - Real-time preview
- Configuration Management
  - Theme settings
  - Language settings
  - Time format settings
- History Records
  - Save history by tool type
  - Support history clearing

## Recent Updates

### 2024 Update

- **UI Improvements**:
  - Replaced custom list with standard ComboBox for better time format selection
  - Simplified the Time Tool interface by removing redundant features
  - Enhanced error messages with more detailed format conversion error reasons
  
- **Feature Enhancements**:
  - Removed timestamp to datetime conversion to focus on datetime to timestamp
  - Improved time format validation logic with support for more formats
  - Enhanced datetime conversion logic with friendlier error feedback

- **Code Structure**:
  - Refactored TimeLogic and JsonLogic classes for better internal structure
  - Optimized event handling logic for simplified code flow
  - Improved code documentation and comments

## Getting Started

### Prerequisites

- Rust 1.70 or higher
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

Or directly run the executable file in `target/release/dev_tool.exe` (Windows) or `target/release/dev_tool` (Linux/macOS).

## Development

### Project Structure

```
src/
├── ui/                    # UI components (.slint files)
│   ├── main.slint
│   ├── components/       # Reusable components
│   ├── layouts/         # Layout components
│   └── themes/          # Theme definitions
├── services/            # System services
│   ├── config.rs       # Configuration management
│   └── storage.rs      # Data storage
├── utils/              # Utility functions
│   ├── time.rs        # Time conversion utilities
│   └── json.rs        # JSON processing utilities
├── logic/              # Business logic
│   ├── time.rs        # Time business logic
│   ├── json.rs        # JSON business logic
│   └── event.rs       # Event handling
└── main.rs            # Application entry
```

### Module Description

- **ui**: Contains all UI-related .slint files, responsible for interface presentation
- **services**: Provides system-level services like configuration management and data storage
- **utils**: Provides basic utility functions without business logic
- **logic**: Contains all business logic implementations, utilizing utility functions from utils

### Build from Source

1. Install Rust and Cargo
2. Clone the repository
3. Run `cargo build --release`

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
