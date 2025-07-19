# todo list

[English](./README-en.md) | [中文](./README.md)

A minimalist and high-performance to-do list manager tailored for programmers, offering both CLI and GUI modes to streamline task management for developers.

![example](./docs/screen_shot.jpg)

## Key Features

### 1. Fast Interaction

- Supports both command line (CLI) and menu bar icon (Tray) modes for quick wake-up, allowing you to record tasks anytime and anywhere.

### 2. Protecting Data Privacy

- All the data are stored in the local database.

### 3. Concentration

- Keep the main interface simple and hide all the configurations in the configuration bar.

## Quick Start

```bash
cargo build

cd app
npm install
npm run tauri build
# for dev
npm run tauri dev
```

## 技术栈

- **Rust** + **Tauri**: Lightweight cross-platform GUI container
- **React** + **Ant Design**: Modern responsive interfaces and out-of-the-box components
- **sqlite3**: Local data storage
