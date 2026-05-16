# 🚀 NetPulse - 智能网络监控与设备管理工具

<div align="center">

[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20macOS%20%7C%20Windows-lightgrey)](https://github.com/gitstq/NetPulse-Monitor)
[![GitHub Stars](https://img.shields.io/github/stars/gitstq/NetPulse-Monitor?style=social)](https://github.com/gitstq/NetPulse-Monitor)

**🌐 [English](README.md) | [繁體中文](README.zh-TW.md) | 简体中文**

</div>

---

## 🎉 项目介绍

**NetPulse** 是一款基于 **Rust** 开发的高性能网络监控工具，专为网络管理员、安全研究人员和技术爱好者设计。它提供实时网络流量监控、设备发现、端口扫描和网络诊断等功能。

### ✨ 为什么选择 NetPulse？

- **🚀 极致性能**：采用 Rust 编写，资源占用极低
- **🎨 精美 TUI**：基于终端的交互式界面，数据可视化直观
- **🔍 智能设备发现**：自动识别设备类型和厂商
- **📊 实时监控**：实时展示网络流量统计和设备状态
- **🌍 跨平台**：支持 Linux、macOS 和 Windows 系统
- **🔒 隐私优先**：所有数据本地处理，不上传云端

---

## ✨ 核心特性

| 特性 | 描述 | 状态 |
|------|------|------|
| 📡 **实时监控** | 实时监控网络流量和设备活动 | ✅ 已上线 |
| 🔍 **设备发现** | 自动发现局域网内的设备并识别设备类型 | ✅ 已上线 |
| 🏷️ **设备指纹** | 基于 MAC 地址识别设备厂商和类型 | ✅ 已上线 |
| 📊 **流量统计** | 实时显示上传/下载速度和数据包数量 | ✅ 已上线 |
| 🖥️ **TUI 界面** | 美观的终端用户界面，支持键盘操作 | ✅ 已上线 |
| 💾 **数据导出** | 支持将设备列表导出为 JSON 和 CSV 格式 | ✅ 已上线 |
| 🔌 **多网卡** | 支持监控多个网络接口 | ✅ 已上线 |
| ⚡ **轻量级** | 极低的内存和 CPU 占用 | ✅ 已上线 |

---

## 🚀 快速开始

### 📋 环境要求

- **Rust** 1.75 或更高版本
- **Linux/macOS/Windows** 操作系统
- Root/管理员权限（网络扫描需要）

### 📦 安装

#### 方式一：从源码编译

```bash
# 克隆仓库
git clone https://github.com/gitstq/NetPulse-Monitor.git
cd NetPulse-Monitor

# 构建发布版本
cargo build --release

# 运行
sudo ./target/release/netpulse
```

#### 方式二：使用 Cargo 安装

```bash
cargo install --git https://github.com/gitstq/NetPulse-Monitor
```

### 🎮 使用方法

```bash
# 启动交互式 TUI
netpulse

# 监控指定网卡
netpulse --interface eth0

# 扫描网络范围
netpulse --scan 192.168.1.0/24

# 导出数据为 JSON
netpulse --export json

# 无界面模式运行
netpulse --headless
```

### ⌨️ TUI 快捷键

| 按键 | 功能 |
|------|------|
| `s` | 开始网络扫描 |
| `r` | 刷新数据 |
| `q` / `Esc` | 退出程序 |
| `↑` / `↓` | 导航设备列表 |

---

## 📖 详细使用指南

### 🔍 设备发现

NetPulse 使用多种技术来发现局域网内的设备：

- **ARP 扫描**：通过 ARP 协议发现设备
- **Ping 探测**：通过 ICMP 检测活跃设备
- **MAC 地址分析**：基于 OUI 识别设备厂商

### 🏷️ 设备分类

NetPulse 可以自动识别以下设备类型：

| 图标 | 类型 | 说明 |
|------|------|------|
| 💻 | 计算机 | 台式机或笔记本 |
| 📱 | 手机 | 智能手机 |
| 📲 | 平板 | 平板设备 |
| 📡 | 路由器 | 路由器或网关 |
| 🖨️ | 打印机 | 网络打印机 |
| 📺 | 智能电视 | 智能电视设备 |
| 🔌 | IoT 设备 | 物联网设备 |
| 🖥️ | 服务器 | 服务器设备 |
| 🎮 | 游戏机 | 游戏主机 |
| 📷 | 摄像头 | 网络摄像头 |
| 🔊 | 音箱 | 智能音箱 |
| ⌚ | 手表 | 智能手表 |

### 💾 数据导出

导出设备信息为 JSON 格式：

```bash
netpulse --scan 192.168.1.0/24 --export json
```

导出为 CSV 格式：

```bash
netpulse --scan 192.168.1.0/24 --export csv
```

---

## 💡 设计思路与迭代规划

### 🎯 设计理念

1. **性能优先**：Rust 的零成本抽象确保极致性能
2. **用户体验**：直观的 TUI 界面，无需学习成本
3. **轻量级**：极低的资源占用，适合长期运行
4. **可扩展**：模块化设计，便于功能扩展

### 📅 迭代计划

- [ ] **v1.1.0** - 端口扫描功能
- [ ] **v1.2.0** - 网络拓扑可视化
- [ ] **v1.3.0** - 流量分析与统计
- [ ] **v1.4.0** - 告警通知系统
- [ ] **v2.0.0** - Web 管理界面

---

## 📦 打包与部署指南

### 🔨 从源码构建

```bash
# 调试构建
cargo build

# 发布构建（优化）
cargo build --release

# 运行测试
cargo test
```

### 🐳 Docker 部署

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libpcap-dev
COPY --from=builder /app/target/release/netpulse /usr/local/bin/
ENTRYPOINT ["netpulse"]
```

### 📋 系统要求

| 项目 | 最低配置 | 推荐配置 |
|------|---------|---------|
| CPU | 1 核 | 2 核+ |
| 内存 | 64MB | 256MB+ |
| 磁盘 | 10MB | 100MB+ |
| 网络 | 局域网 | 千兆局域网 |

---

## 🤝 贡献指南

我们欢迎社区贡献！请遵循以下准则：

### 📝 提交 Issue

- 使用清晰的标题和描述
- 提供系统环境信息
- 附上错误日志和截图

### 🔧 提交代码

1. Fork 本仓库
2. 创建功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'feat: add amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 创建 Pull Request

### 📏 代码规范

- 遵循 Rust 官方编码规范
- 添加必要的注释和文档
- 确保代码通过 `cargo clippy` 检查
- 保持测试覆盖率

---

## 📄 开源协议

本项目采用 **MIT 协议** 开源 - 详见 [LICENSE](LICENSE) 文件。

---

## 🙏 致谢

- [Ratatui](https://github.com/ratatui-org/ratatui) - Rust TUI 框架
- [Tokio](https://tokio.rs/) - Rust 异步运行时
- [Clap](https://github.com/clap-rs/clap) - Rust 命令行解析器

---

<div align="center">

**⭐ 如果这个项目对你有帮助，请给我们一个星标！⭐**

[报告问题](https://github.com/gitstq/NetPulse-Monitor/issues) · [贡献代码](https://github.com/gitstq/NetPulse-Monitor/pulls) · [查看文档](https://github.com/gitstq/NetPulse-Monitor/wiki)

</div>
