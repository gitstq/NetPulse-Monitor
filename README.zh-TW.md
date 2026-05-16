# 🚀 NetPulse - 智能網路監控與設備管理工具

<div align="center">

[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20macOS%20%7C%20Windows-lightgrey)](https://github.com/gitstq/NetPulse-Monitor)
[![GitHub Stars](https://img.shields.io/github/stars/gitstq/NetPulse-Monitor?style=social)](https://github.com/gitstq/NetPulse-Monitor)

**🌐 [English](README.md) | 繁體中文 | [简体中文](README.zh-CN.md)**

</div>

---

## 🎉 專案介紹

**NetPulse** 是一款基於 **Rust** 開發的高效能網路監控工具，專為網路管理員、安全研究人員和技術愛好者設計。它提供即時網路流量監控、設備發現、埠掃描和網路診斷等功能。

### ✨ 為什麼選擇 NetPulse？

- **🚀 極致效能**：採用 Rust 編寫，資源佔用極低
- **🎨 精美 TUI**：基於終端的互動式介面，資料視覺化直觀
- **🔍 智慧設備發現**：自動識別設備類型和廠商
- **📊 即時監控**：即時展示網路流量統計和設備狀態
- **🌍 跨平台**：支援 Linux、macOS 和 Windows 系統
- **🔒 隱私優先**：所有資料本地處理，不上傳雲端

---

## ✨ 核心特性

| 特性 | 描述 | 狀態 |
|------|------|------|
| 📡 **即時監控** | 即時監控網路流量和設備活動 | ✅ 已上線 |
| 🔍 **設備發現** | 自動發現區域網路內的設備並識別設備類型 | ✅ 已上線 |
| 🏷️ **設備指紋** | 基於 MAC 位址識別設備廠商和類型 | ✅ 已上線 |
| 📊 **流量統計** | 即時顯示上傳/下載速度和封包數量 | ✅ 已上線 |
| 🖥️ **TUI 介面** | 美觀的終端使用者介面，支援鍵盤操作 | ✅ 已上線 |
| 💾 **資料匯出** | 支援將設備列表匯出為 JSON 和 CSV 格式 | ✅ 已上線 |
| 🔌 **多網卡** | 支援監控多個網路介面 | ✅ 已上線 |
| ⚡ **輕量級** | 極低的記憶體和 CPU 佔用 | ✅ 已上線 |

---

## 🚀 快速開始

### 📋 環境要求

- **Rust** 1.75 或更高版本
- **Linux/macOS/Windows** 作業系統
- Root/管理員權限（網路掃描需要）

### 📦 安裝

#### 方式一：從原始碼編譯

```bash
# 複製倉庫
git clone https://github.com/gitstq/NetPulse-Monitor.git
cd NetPulse-Monitor

# 構建發布版本
cargo build --release

# 執行
sudo ./target/release/netpulse
```

#### 方式二：使用 Cargo 安裝

```bash
cargo install --git https://github.com/gitstq/NetPulse-Monitor
```

### 🎮 使用方法

```bash
# 啟動互動式 TUI
netpulse

# 監控指定網卡
netpulse --interface eth0

# 掃描網路範圍
netpulse --scan 192.168.1.0/24

# 匯出資料為 JSON
netpulse --export json

# 無介面模式執行
netpulse --headless
```

### ⌨️ TUI 快捷鍵

| 按鍵 | 功能 |
|------|------|
| `s` | 開始網路掃描 |
| `r` | 重新整理資料 |
| `q` / `Esc` | 退出程式 |
| `↑` / `↓` | 導航設備列表 |

---

## 📖 詳細使用指南

### 🔍 設備發現

NetPulse 使用多種技術來發現區域網路內的設備：

- **ARP 掃描**：通過 ARP 協定發現設備
- **Ping 探測**：通過 ICMP 檢測活躍設備
- **MAC 位址分析**：基於 OUI 識別設備廠商

### 🏷️ 設備分類

NetPulse 可以自動識別以下設備類型：

| 圖示 | 類型 | 說明 |
|------|------|------|
| 💻 | 電腦 | 桌上型或筆記型電腦 |
| 📱 | 手機 | 智慧型手機 |
| 📲 | 平板 | 平板設備 |
| 📡 | 路由器 | 路由器或閘道器 |
| 🖨️ | 印表機 | 網路印表機 |
| 📺 | 智慧電視 | 智慧電視設備 |
| 🔌 | IoT 設備 | 物聯網設備 |
| 🖥️ | 伺服器 | 伺服器設備 |
| 🎮 | 遊戲機 | 遊戲主機 |
| 📷 | 攝影機 | 網路攝影機 |
| 🔊 | 音箱 | 智慧音箱 |
| ⌚ | 手錶 | 智慧手錶 |

### 💾 資料匯出

匯出設備資訊為 JSON 格式：

```bash
netpulse --scan 192.168.1.0/24 --export json
```

匯出為 CSV 格式：

```bash
netpulse --scan 192.168.1.0/24 --export csv
```

---

## 💡 設計思路與迭代規劃

### 🎯 設計理念

1. **效能優先**：Rust 的零成本抽象確保極致效能
2. **使用者體驗**：直觀的 TUI 介面，無需學習成本
3. **輕量級**：極低的資源佔用，適合長期執行
4. **可擴展**：模組化設計，便於功能擴展

### 📅 迭代計劃

- [ ] **v1.1.0** - 埠掃描功能
- [ ] **v1.2.0** - 網路拓撲視覺化
- [ ] **v1.3.0** - 流量分析與統計
- [ ] **v1.4.0** - 告警通知系統
- [ ] **v2.0.0** - Web 管理介面

---

## 📦 打包與部署指南

### 🔨 從原始碼構建

```bash
# 偵錯構建
cargo build

# 發布構建（最佳化）
cargo build --release

# 執行測試
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

### 📋 系統要求

| 項目 | 最低配置 | 推薦配置 |
|------|---------|---------|
| CPU | 1 核 | 2 核+ |
| 記憶體 | 64MB | 256MB+ |
| 磁碟 | 10MB | 100MB+ |
| 網路 | 區域網路 | 千兆區域網路 |

---

## 🤝 貢獻指南

我們歡迎社群貢獻！請遵循以下準則：

### 📝 提交 Issue

- 使用清晰的標題和描述
- 提供系統環境資訊
- 附上錯誤日誌和截圖

### 🔧 提交程式碼

1. Fork 本倉庫
2. 建立功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'feat: add amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 建立 Pull Request

### 📏 程式碼規範

- 遵循 Rust 官方編碼規範
- 新增必要的註解和文件
- 確保程式碼通過 `cargo clippy` 檢查
- 保持測試覆蓋率

---

## 📄 開源協議

本專案採用 **MIT 協議** 開源 - 詳見 [LICENSE](LICENSE) 文件。

---

## 🙏 致謝

- [Ratatui](https://github.com/ratatui-org/ratatui) - Rust TUI 框架
- [Tokio](https://tokio.rs/) - Rust 非同步執行時
- [Clap](https://github.com/clap-rs/clap) - Rust 命令列解析器

---

<div align="center">

**⭐ 如果這個專案對你有幫助，請給我們一個星標！⭐**

[報告問題](https://github.com/gitstq/NetPulse-Monitor/issues) · [貢獻程式碼](https://github.com/gitstq/NetPulse-Monitor/pulls) · [查看文件](https://github.com/gitstq/NetPulse-Monitor/wiki)

</div>
