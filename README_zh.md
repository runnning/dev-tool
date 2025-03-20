# 开发者工具

基于 Rust 和 Slint 构建的实用开发者工具集。

[English Documentation](README.md)

## 功能特性

### 时间戳转换
- Unix 时间戳转换为可读日期时间
- 毫秒时间戳转换为可读日期时间
- 获取当前 Unix 时间戳
- 获取当前毫秒时间戳

### JSON 美化
- 格式化和美化 JSON 文本
- 带有适当缩进的易读输出

## 快速开始

### 环境要求
- Rust 1.75.0 或更高版本
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

## 项目结构

```
dev-tool/
├── src/
│   ├── gui/                # GUI 相关代码
│   │   ├── components/     # UI 组件
│   │   │   ├── styled_button.slint    # 自定义按钮组件
│   │   │   ├── single_line_edit.slint # 单行输入组件
│   │   │   ├── timestamp_panel.slint  # 时间戳转换面板
│   │   │   └── json_panel.slint       # JSON 美化面板
│   │   └── main.slint      # 主界面
│   ├── modules/            # 功能模块
│   │   ├── timestamp.rs    # 时间戳处理
│   │   ├── json_formatter.rs # JSON 格式化
│   │   └── mod.rs         # 模块导出
│   ├── handlers/          # 事件处理
│   │   ├── timestamp_handler.rs # 时间戳事件处理
│   │   ├── json_handler.rs     # JSON 事件处理
│   │   └── mod.rs        # 处理器导出
│   └── main.rs           # 程序入口
└── Cargo.toml           # 项目配置文件
```

## 参与贡献

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m '添加某个特性'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 提交 Pull Request

## 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件 