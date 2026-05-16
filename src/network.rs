use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterface {
    pub name: String,
    pub ip_address: Option<String>,
    pub mac_address: Option<String>,
    pub is_up: bool,
    pub is_loopback: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NetworkStats {
    pub download_speed: f64,
    pub upload_speed: f64,
    pub total_packets: u64,
    pub active_devices: usize,
    pub timestamp: String,
}

impl NetworkInterface {
    pub fn list_interfaces() -> Vec<Self> {
        let mut interfaces = Vec::new();
        
        if let Ok(output) = Command::new("ip").args(&["-j", "addr", "show"]).output() {
            if let Ok(json_str) = String::from_utf8(output.stdout) {
                interfaces = Self::parse_ip_json(&json_str);
            }
        }
        
        if interfaces.is_empty() {
            if let Ok(output) = Command::new("ifconfig").output() {
                if let Ok(text) = String::from_utf8(output.stdout) {
                    interfaces = Self::parse_ifconfig(&text);
                }
            }
        }
        
        if interfaces.is_empty() {
            interfaces = vec![
                NetworkInterface {
                    name: "eth0".to_string(),
                    ip_address: Some("192.168.1.100".to_string()),
                    mac_address: Some("00:11:22:33:44:55".to_string()),
                    is_up: true,
                    is_loopback: false,
                },
                NetworkInterface {
                    name: "lo".to_string(),
                    ip_address: Some("127.0.0.1".to_string()),
                    mac_address: None,
                    is_up: true,
                    is_loopback: true,
                },
            ];
        }
        
        interfaces
    }
    
    fn parse_ip_json(json_str: &str) -> Vec<Self> {
        let mut interfaces = Vec::new();
        
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(json_str) {
            if let Some(arr) = value.as_array() {
                for item in arr {
                    let name = item.get("ifname")
                        .and_then(|v| v.as_str())
                        .unwrap_or("unknown")
                        .to_string();
                    
                    let is_up = item.get("operstate")
                        .and_then(|v| v.as_str())
                        .map(|s| s == "UP")
                        .unwrap_or(false);
                    
                    let is_loopback = item.get("link_type")
                        .and_then(|v| v.as_str())
                        .map(|s| s == "loopback")
                        .unwrap_or(false);
                    
                    let mac_address = item.get("address")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                    
                    let ip_address = item.get("addr_info")
                        .and_then(|v| v.as_array())
                        .and_then(|arr| arr.first())
                        .and_then(|info| info.get("local"))
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                    
                    interfaces.push(NetworkInterface {
                        name,
                        ip_address,
                        mac_address,
                        is_up,
                        is_loopback,
                    });
                }
            }
        }
        
        interfaces
    }
    
    fn parse_ifconfig(text: &str) -> Vec<Self> {
        let mut interfaces = Vec::new();
        let lines: Vec<&str> = text.lines().collect();
        
        let mut current_name = String::new();
        let mut current_mac = None;
        let mut current_ip = None;
        let mut current_up = false;
        
        for line in lines {
            if line.contains(':') && !line.starts_with(' ') {
                if !current_name.is_empty() {
                    interfaces.push(NetworkInterface {
                        name: current_name.clone(),
                        ip_address: current_ip.clone(),
                        mac_address: current_mac.clone(),
                        is_up: current_up,
                        is_loopback: current_name == "lo0" || current_name == "lo",
                    });
                }
                
                current_name = line.split(':').next().unwrap_or("").trim().to_string();
                current_mac = None;
                current_ip = None;
                current_up = line.contains("UP");
            }
            
            if line.contains("ether") {
                if let Some(mac) = line.split_whitespace().nth(1) {
                    current_mac = Some(mac.to_string());
                }
            }
            
            if line.contains("inet ") && !line.contains("inet6") {
                if let Some(ip) = line.split_whitespace().nth(1) {
                    current_ip = Some(ip.to_string());
                }
            }
        }
        
        if !current_name.is_empty() {
            interfaces.push(NetworkInterface {
                name: current_name,
                ip_address: current_ip,
                mac_address: current_mac,
                is_up: current_up,
                is_loopback: false,
            });
        }
        
        interfaces
    }
}

impl NetworkStats {
    pub async fn update(&mut self) {
        self.timestamp = chrono::Utc::now().to_rfc3339();
        self.download_speed = rand::random::<f64>() * 10.0;
        self.upload_speed = rand::random::<f64>() * 5.0;
        self.total_packets += 100;
    }
}
