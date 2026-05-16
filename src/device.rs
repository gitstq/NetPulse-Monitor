use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub mac_address: String,
    pub ip_address: String,
    pub hostname: Option<String>,
    pub vendor: Option<String>,
    pub device_type: DeviceType,
    pub open_ports: Vec<u16>,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub packet_count: u64,
    pub bytes_sent: u64,
    pub bytes_received: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceType {
    Unknown,
    Computer,
    Mobile,
    Tablet,
    Router,
    Printer,
    SmartTV,
    IoT,
    Server,
    GameConsole,
    Camera,
    Speaker,
    Watch,
    Other(String),
}

impl DeviceType {
    pub fn as_str(&self) -> &str {
        match self {
            DeviceType::Unknown => "Unknown",
            DeviceType::Computer => "Computer",
            DeviceType::Mobile => "Mobile",
            DeviceType::Tablet => "Tablet",
            DeviceType::Router => "Router",
            DeviceType::Printer => "Printer",
            DeviceType::SmartTV => "Smart TV",
            DeviceType::IoT => "IoT Device",
            DeviceType::Server => "Server",
            DeviceType::GameConsole => "Game Console",
            DeviceType::Camera => "Camera",
            DeviceType::Speaker => "Speaker",
            DeviceType::Watch => "Watch",
            DeviceType::Other(s) => s.as_str(),
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            DeviceType::Unknown => "❓",
            DeviceType::Computer => "💻",
            DeviceType::Mobile => "📱",
            DeviceType::Tablet => "📲",
            DeviceType::Router => "📡",
            DeviceType::Printer => "🖨️",
            DeviceType::SmartTV => "📺",
            DeviceType::IoT => "🔌",
            DeviceType::Server => "🖥️",
            DeviceType::GameConsole => "🎮",
            DeviceType::Camera => "📷",
            DeviceType::Speaker => "🔊",
            DeviceType::Watch => "⌚",
            DeviceType::Other(_) => "📦",
        }
    }
}

#[derive(Debug, Clone)]
pub struct DeviceScanResult {
    pub mac_address: String,
    pub ip_address: String,
    pub hostname: Option<String>,
    pub open_ports: Vec<u16>,
    pub ttl: Option<u8>,
}

impl Device {
    pub fn from_scan_result(result: DeviceScanResult) -> Self {
        let now = Utc::now();
        let vendor = lookup_vendor(&result.mac_address);
        let device_type = detect_device_type(&result, &vendor);

        Self {
            mac_address: result.mac_address,
            ip_address: result.ip_address,
            hostname: result.hostname,
            vendor,
            device_type,
            open_ports: result.open_ports,
            first_seen: now,
            last_seen: now,
            packet_count: 0,
            bytes_sent: 0,
            bytes_received: 0,
        }
    }

    pub fn update_activity(&mut self, bytes_sent: u64, bytes_received: u64) {
        self.last_seen = Utc::now();
        self.packet_count += 1;
        self.bytes_sent += bytes_sent;
        self.bytes_received += bytes_received;
    }

    pub fn is_active(&self) -> bool {
        let now = Utc::now();
        let duration = now.signed_duration_since(self.last_seen);
        duration.num_seconds() < 60
    }

    pub fn get_total_bytes(&self) -> u64 {
        self.bytes_sent + self.bytes_received
    }
}

fn lookup_vendor(mac_address: &str) -> Option<String> {
    let oui = mac_address.split(':').take(3).collect::<Vec<_>>().join(":");
    
    let vendors: std::collections::HashMap<&str, &str> = [
        ("b8:27:eb", "Raspberry Pi"),
        ("dc:a6:32", "Raspberry Pi"),
        ("00:17:88", "Philips Hue"),
        ("18:b4:30", "Nest"),
        ("64:69:4e", "Amazon"),
        ("a4:cf:12", "Xiaomi"),
        ("ac:de:48", "Apple"),
        ("f0:18:98", "Apple"),
        ("28:cf:e9", "Apple"),
        ("3c:5a:b4", "Google"),
        ("94:eb:2c", "Google"),
        ("a4:5e:60", "Samsung"),
        ("00:1e:c0", "Samsung"),
        ("00:12:47", "Samsung"),
        ("00:26:bb", "Samsung"),
        ("00:1f:3a", "Samsung"),
        ("00:24:54", "Samsung"),
        ("00:e0:4c", "Realtek"),
        ("00:1b:21", "Intel"),
        ("00:1c:c4", "Intel"),
        ("00:21:5c", "Intel"),
        ("00:22:fa", "Intel"),
        ("00:23:14", "Intel"),
        ("00:24:d6", "Intel"),
        ("00:26:c6", "Intel"),
        ("00:50:56", "VMware"),
        ("08:00:27", "VirtualBox"),
        ("52:54:00", "QEMU"),
    ].iter().cloned().collect();
    
    vendors.get(oui.as_str()).map(|&v| v.to_string())
}

fn detect_device_type(result: &DeviceScanResult, vendor: &Option<String>) -> DeviceType {
    if let Some(v) = vendor {
        let v_lower = v.to_lowercase();
        if v_lower.contains("apple") {
            return DeviceType::Mobile;
        } else if v_lower.contains("samsung") {
            return DeviceType::Mobile;
        } else if v_lower.contains("xiaomi") {
            return DeviceType::Mobile;
        } else if v_lower.contains("raspberry") {
            return DeviceType::Computer;
        } else if v_lower.contains("philips") || v_lower.contains("hue") {
            return DeviceType::IoT;
        } else if v_lower.contains("nest") {
            return DeviceType::IoT;
        } else if v_lower.contains("amazon") {
            return DeviceType::Speaker;
        }
    }
    
    DeviceType::Unknown
}
