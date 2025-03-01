# 贪吃蛇游戏 - Rust 实现

这是一个使用 Rust 编写的基于命令行的贪吃蛇游戏, 玩家可以通过方向键控制蛇的移动。

## 功能特性

- **经典贪吃蛇玩法**：控制蛇的移动，吃掉食物并增长身体。
- **命令行界面**：基于终端的简洁界面，支持跨平台运行。
- **方向键控制**：使用上下左右方向键控制蛇的移动。
- **随机食物生成**：食物会随机出现在游戏区域内。
- **游戏结束检测**：当蛇撞到墙壁或自身时，游戏结束。

## 依赖

- [crossterm](https://crates.io/crates/crossterm)：用于处理终端输入输出和渲染。

## 安装与运行

### 下载并安装

到[此处](https://github.com/quyc07/examination/releases)下载对应平台的安装包,解压并运行.

### cargo 安装

```shell
cargo install snake_game_cli

snake_game_cli
```
