use anyhow::Result;
use std::process::Command;
use std::time::Duration;
use tokio::time::timeout;

use crate::device::DeviceScanResult;

pub struct NetworkScanner;

impl NetworkScanner {
    pub fn new() -> Self {
        Self
    }

    pub async fn scan_range(&self, range: &str) -> Result<Vec<DeviceScanResult>> {
        let mut results = Vec::new();
        
        let (base_ip, mask) = if let Some((ip, m)) = range.split_once('/') {
            (ip.to_string(), m.parse::<u8>().unwrap_or(24))
        } else {
            (range.to_string(), 24)
        };
        
        let base_octets: Vec<u8> = base_ip
            .split('.')
            .filter_map(|s| s.parse().ok())
            .collect();
        
        if base_octets.len() != 4 {
            return Err(anyhow::anyhow!("Invalid IP address format"));
        }
        
        let host_bits = 32 - mask;
        let num_hosts = if host_bits >= 32 {
            1
        } else {
            1 << host_bits
        };
        
        let scan_count = num_hosts.min(256);
        
        for i in 1..scan_count {
            let ip = format!("{}.{}.{}.{}", 
                base_octets[0], 
                base_octets[1], 
                base_octets[2], 
                base_octets[3].wrapping_add(i as u8)
            );
            
            if let Ok(Some(result)) = timeout(
                Duration::from_millis(100),
                self.scan_host(&ip)
            ).await {
                results.push(result);
            }
        }
        
        Ok(results)
    }

    async fn scan_host(&self, ip: &str) -> Result<Option<DeviceScanResult>> {
        let ping_result = if cfg!(target_os = "windows") {
            Command::new("ping")
                .args(&["-n", "1", "-w", "500", ip])
                .output()
        } else {
            Command::new("ping")
                .args(&["-c", "1", "-W", "1", ip])
                .output()
        };
        
        if let Ok(output) = ping_result {
            if output.status.success() {
                let mac = self.get_mac_address(ip).await.ok();
                let hostname = self.resolve_hostname(ip).await.ok();
                
                return Ok(Some(DeviceScanResult {
                    mac_address: mac.unwrap_or_else(|| "00:00:00:00:00:00".to_string()),
                    ip_address: ip.to_string(),
                    hostname,
                    open_ports: Vec::new(),
                    ttl: None,
                }));
            }
        }
        
        Ok(None)
    }

    async fn get_mac_address(&self, ip: &str) -> Result<String> {
        let output = if cfg!(target_os = "windows") {
            Command::new("arp").args(&["-a", ip]).output()?
        } else {
            Command::new("ip").args(&["neigh", "show", ip]).output()?
        };
        
        let text = String::from_utf8_lossy(&output.stdout);
        
        for line in text.lines() {
            if line.contains(ip) {
                for word in line.split_whitespace() {
                    if word.len() == 17 && word.contains(':') {
                        return Ok(word.to_string());
                    }
                }
            }
        }
        
        Err(anyhow::anyhow!("MAC address not found"))
    }

    async fn resolve_hostname(&self, ip: &str) -> Result<String> {
        let output = Command::new("host")
            .args(&[ip])
            .output()?;
        
        let text = String::from_utf8_lossy(&output.stdout);
        
        if let Some(line) = text.lines().next() {
            if let Some(domain) = line.split("domain name pointer ").nth(1) {
                return Ok(domain.trim().trim_end_matches('.').to_string());
            }
        }
        
        Err(anyhow::anyhow!("Hostname not found"))
    }
}
