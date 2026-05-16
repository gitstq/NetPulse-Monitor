# 🚀 NetPulse - Intelligent Network Monitoring & Device Management Tool

<div align="center">

[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20macOS%20%7C%20Windows-lightgrey)](https://github.com/gitstq/NetPulse-Monitor)
[![GitHub Stars](https://img.shields.io/github/stars/gitstq/NetPulse-Monitor?style=social)](https://github.com/gitstq/NetPulse-Monitor)

**🌐 [简体中文](README.zh-CN.md) | [繁體中文](README.zh-TW.md) | [English](README.md)**

</div>

---

## 🎉 Project Introduction

**NetPulse** is a high-performance network monitoring tool built with **Rust**, designed for network administrators, security researchers, and tech enthusiasts. It provides real-time network traffic monitoring, device discovery, port scanning, and network diagnostics capabilities.

### ✨ Why NetPulse?

- **🚀 High Performance**: Built with Rust, featuring extremely low resource consumption
- **🎨 Beautiful TUI**: Terminal-based interactive interface with intuitive data visualization
- **🔍 Smart Device Discovery**: Automatic device fingerprinting and classification
- **📊 Real-time Monitoring**: Real-time network traffic statistics and device status
- **🌍 Cross-platform**: Supports Linux, macOS, and Windows systems
- **🔒 Privacy First**: All data processed locally, no cloud uploads

---

## ✨ Core Features

| Feature | Description | Status |
|---------|-------------|--------|
| 📡 **Real-time Monitoring** | Monitor network traffic and device activity in real-time | ✅ Available |
| 🔍 **Device Discovery** | Automatically discover devices in the LAN and identify device types | ✅ Available |
| 🏷️ **Device Fingerprinting** | Identify device vendors and types based on MAC addresses | ✅ Available |
| 📊 **Traffic Statistics** | Real-time display of upload/download speeds and packet counts | ✅ Available |
| 🖥️ **TUI Interface** | Beautiful terminal user interface with keyboard navigation | ✅ Available |
| 💾 **Data Export** | Support exporting device lists to JSON and CSV formats | ✅ Available |
| 🔌 **Multi-interface** | Support monitoring multiple network interfaces | ✅ Available |
| ⚡ **Lightweight** | Minimal memory and CPU usage | ✅ Available |

---

## 🚀 Quick Start

### 📋 Requirements

- **Rust** 1.75 or higher
- **Linux/macOS/Windows** operating system
- Root/Administrator privileges (required for network scanning)

### 📦 Installation

#### Method 1: Install from Source

```bash
# Clone the repository
git clone https://github.com/gitstq/NetPulse-Monitor.git
cd NetPulse-Monitor

# Build release version
cargo build --release

# Run
sudo ./target/release/netpulse
```

#### Method 2: Using Cargo Install

```bash
cargo install --git https://github.com/gitstq/NetPulse-Monitor
```

### 🎮 Usage

```bash
# Launch interactive TUI
netpulse

# Monitor specified interface
netpulse --interface eth0

# Scan network range
netpulse --scan 192.168.1.0/24

# Export data to JSON
netpulse --export json

# Run in headless mode
netpulse --headless
```

### ⌨️ TUI Shortcuts

| Key | Function |
|-----|----------|
| `s` | Start network scan |
| `r` | Refresh data |
| `q` / `Esc` | Quit program |
| `↑` / `↓` | Navigate device list |

---

## 📖 Detailed Usage Guide

### 🔍 Device Discovery

NetPulse uses multiple technologies to discover devices in the LAN:

- **ARP Scanning**: Discover devices via ARP protocol
- **Ping Detection**: Detect active devices through ICMP
- **MAC Address Analysis**: Identify device vendors based on OUI

### 🏷️ Device Classification

NetPulse can automatically identify the following device types:

| Icon | Type | Description |
|------|------|-------------|
| 💻 | Computer | Desktop or laptop |
| 📱 | Mobile | Smartphone |
| 📲 | Tablet | Tablet device |
| 📡 | Router | Router or gateway |
| 🖨️ | Printer | Network printer |
| 📺 | Smart TV | Smart television |
| 🔌 | IoT Device | Internet of Things devices |
| 🖥️ | Server | Server device |
| 🎮 | Game Console | Gaming console |
| 📷 | Camera | Network camera |
| 🔊 | Speaker | Smart speaker |
| ⌚ | Watch | Smart watch |

### 💾 Data Export

Export device information to JSON format:

```bash
netpulse --scan 192.168.1.0/24 --export json
```

Export to CSV format:

```bash
netpulse --scan 192.168.1.0/24 --export csv
```

---

## 💡 Design Philosophy & Iteration Plan

### 🎯 Design Goals

1. **Performance First**: Rust's zero-cost abstraction ensures ultimate performance
2. **User Experience**: Intuitive TUI interface with no learning curve
3. **Lightweight**: Minimal resource usage, suitable for long-running
4. **Extensibility**: Modular design for easy feature extension

### 📅 Iteration Plan

- [ ] **v1.1.0** - Port scanning functionality
- [ ] **v1.2.0** - Network topology visualization
- [ ] **v1.3.0** - Traffic analysis and statistics
- [ ] **v1.4.0** - Alert notification system
- [ ] **v2.0.0** - Web management interface

---

## 📦 Build & Deployment Guide

### 🔨 Build from Source

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test
```

### 🐳 Docker Deployment

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

### 📋 System Requirements

| Item | Minimum | Recommended |
|------|---------|-------------|
| CPU | 1 core | 2 cores+ |
| Memory | 64MB | 256MB+ |
| Disk | 10MB | 100MB+ |
| Network | LAN | Gigabit LAN |

---

## 🤝 Contribution Guide

We welcome community contributions! Please follow these guidelines:

### 📝 Submitting Issues

- Use clear titles and descriptions
- Provide system environment information
- Attach error logs and screenshots

### 🔧 Submitting Code

1. Fork this repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'feat: add amazing feature'`)
4. Push to branch (`git push origin feature/amazing-feature`)
5. Create a Pull Request

### 📏 Code Standards

- Follow Rust official coding standards
- Add necessary comments and documentation
- Ensure code passes `cargo clippy` checks
- Maintain test coverage

---

## 📄 License

This project is licensed under the **MIT License** - see the [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

- [Ratatui](https://github.com/ratatui-org/ratatui) - Rust TUI framework
- [Tokio](https://tokio.rs/) - Rust asynchronous runtime
- [Clap](https://github.com/clap-rs/clap) - Rust command-line parser

---

<div align="center">

**⭐ If you like this project, please give us a star! ⭐**

[Report Issues](https://github.com/gitstq/NetPulse-Monitor/issues) · [Contribute Code](https://github.com/gitstq/NetPulse-Monitor/pulls) · [View Documentation](https://github.com/gitstq/NetPulse-Monitor/wiki)

</div>
