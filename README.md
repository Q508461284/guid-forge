# GUID Forge (GUID 生成器)

一个基于 **Rust** 与 **GPUI-Kit** 构建的轻量级、极简 macOS 风格的 Windows GUID 生成器。

## ✨ 特性

- 🎨 **原生 macOS 极简 UI**：浅灰背景、圆角卡片、系统蓝点缀，告别传统 Win32 界面。
- 🚀 **多格式支持**：一屏同时展示并支持独立生成与复制 7 种主流 GUID 格式：
  - 连字符格式 (Hyphenated)
  - 简单格式 (Simple)
  - 花括号格式 (Braced)
  - URN 格式
  - Windows CLSID (适用于注册表与 COM 开发)
  - 圆括号格式 (Parenthesized)
  - C 结构体格式 (可直接粘贴进 C/C++ 源码)
- 📋 **一键复制**：每行独立操作，点击“复制”自动写入系统剪贴板，并有 1.5 秒状态反馈。
- ⌨️ **输入法支持**：输入框支持 Windows 输入法（IME），可随时手动微调。
- 📦 **极致轻量**：开启 `lto` 和 `strip` 等编译优化，单文件仅约 13MB。

## 🛠️ 构建指南

确保你的系统已安装 [Rust](https://rustup.rs/)。

```bash
# 克隆仓库
git clone https://gitee.com/cong-ao-chen/guid-forge.git
cd guid-forge

# 构建 Release 版本
cargo build --release
