# 开发工具集

简体中文 | [English](README.md)

一个使用 Rust 和 Slint 构建的开发工具集合。

## 功能特性

- 时间工具
  - 获取当前时间
  - 日期时间转时间戳（秒级和毫秒级）
  - 支持多种时间格式
- JSON工具
  - JSON格式化
  - JSON压缩
  - 实时预览
- 配置管理
  - 主题设置
  - 语言设置
  - 时间格式设置
- 历史记录
  - 按工具类型保存历史
  - 支持清除历史

## 最近更新

### 2024年更新

- **界面优化**：
  - 使用标准 ComboBox 组件替代自定义列表，提升时间格式选择体验
  - 简化了时间工具界面，移除了冗余功能
  - 优化错误提示信息，显示更详细的格式转换错误原因
  
- **功能改进**：
  - 移除时间戳转日期时间的功能，专注于日期时间转时间戳
  - 增强了时间格式验证逻辑，支持更多格式
  - 优化了日期时间转换逻辑，提供更友好的错误反馈

- **代码结构**：
  - 重构 TimeLogic 和 JsonLogic 类，优化内部代码结构
  - 优化事件处理逻辑，简化代码流程
  - 改进代码文档和注释

## 快速开始

### 环境要求

- Rust 1.70 或更高版本
- Cargo 包管理器

### 安装步骤

1. 克隆仓库
```bash
git clone https://github.com/yourusername/dev-tool.git
cd dev-tool
```

2. 构建项目
```bash
cargo build --release
```

3. 运行应用
```bash
cargo run --release
```

或直接运行可执行文件：Windows 系统在 `target/release/dev_tool.exe`，Linux/macOS 系统在 `target/release/dev_tool`。

## 开发指南

### 项目结构

```
src/
├── ui/                    # UI组件（.slint文件）
│   ├── main.slint
│   ├── components/       # 可复用组件
│   ├── layouts/         # 布局组件
│   └── themes/          # 主题定义
├── services/            # 系统服务
│   ├── config.rs       # 配置管理
│   └── storage.rs      # 数据存储
├── utils/              # 工具函数
│   ├── time.rs        # 时间转换工具
│   └── json.rs        # JSON处理工具
├── logic/              # 业务逻辑
│   ├── time.rs        # 时间业务逻辑
│   ├── json.rs        # JSON业务逻辑
│   └── event.rs       # 事件处理
└── main.rs            # 应用入口
```

### 模块说明

- **ui**: 包含所有UI相关的.slint文件，负责界面展示
- **services**: 提供系统级服务，如配置管理和数据存储
- **utils**: 提供基础工具函数，不包含业务逻辑
- **logic**: 包含所有业务逻辑实现，使用utils中的工具函数

### 从源码构建

1. 安装 Rust 和 Cargo
2. 克隆仓库
3. 运行 `cargo build --release`

## 许可证

本项目基于 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。
