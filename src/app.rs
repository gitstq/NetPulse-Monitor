use anyhow::Result;
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

use crate::config::Config;
use crate::device::{Device, DeviceScanResult, DeviceType};
use crate::network::{NetworkInterface, NetworkStats};
use crate::scanner::NetworkScanner;

pub struct App {
    pub config: Config,
    pub devices: DashMap<String, Device>,
    pub interfaces: Vec<NetworkInterface>,
    pub selected_interface: Option<String>,
    pub network_stats: Arc<RwLock<NetworkStats>>,
    pub running: Arc<RwLock<bool>>,
    pub last_scan: Option<DateTime<Utc>>,
    pub scanner: NetworkScanner,
}

impl App {
    pub fn new(config: Config) -> Self {
        let interfaces = NetworkInterface::list_interfaces();
        let scanner = NetworkScanner::new();

        Self {
            config,
            devices: DashMap::new(),
            interfaces,
            selected_interface: None,
            network_stats: Arc::new(RwLock::new(NetworkStats::default())),
            running: Arc::new(RwLock::new(false)),
            last_scan: None,
            scanner,
        }
    }

    pub async fn start_monitoring(&mut self) -> Result<()> {
        info!("🟢 Starting network monitoring");
        *self.running.write().await = true;

        let interface = self
            .selected_interface
            .clone()
            .or_else(|| self.config.interface.clone())
            .or_else(|| self.interfaces.first().map(|i| i.name.clone()))
            .ok_or_else(|| anyhow::anyhow!("No network interface available"))?;

        info!("📡 Monitoring interface: {}", interface);

        let devices = self.devices.clone();
        let stats = self.network_stats.clone();
        let running = self.running.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));
            
            while *running.read().await {
                interval.tick().await;
                
                let mut stats_guard = stats.write().await;
                stats_guard.update().await;
                drop(stats_guard);

                let now = Utc::now();
                devices.retain(|_, device| {
                    let last_seen = device.last_seen;
                    let duration = now.signed_duration_since(last_seen);
                    duration.num_seconds() < 300
                });
            }
        });

        Ok(())
    }

    pub async fn stop_monitoring(&mut self) -> Result<()> {
        info!("🔴 Stopping network monitoring");
        *self.running.write().await = false;
        Ok(())
    }

    pub async fn scan_network(&mut self, range: &str) -> Result<()> {
        info!("🔍 Starting network scan: {}", range);
        
        let discovered_devices = self.scanner.scan_range(range).await?;
        
        for device_info in discovered_devices {
            let device = Device::from_scan_result(device_info);
            self.devices.insert(device.mac_address.clone(), device);
        }

        self.last_scan = Some(Utc::now());
        info!("✅ Network scan completed. Found {} devices", self.devices.len());
        
        Ok(())
    }

    pub async fn get_network_stats(&self) -> Result<NetworkStats> {
        let stats = self.network_stats.read().await.clone();
        Ok(stats)
    }

    pub fn get_devices(&self) -> Vec<Device> {
        self.devices
            .iter()
            .map(|entry| entry.value().clone())
            .collect()
    }

    pub fn get_device_count(&self) -> usize {
        self.devices.len()
    }

    pub fn get_active_device_count(&self) -> usize {
        let now = Utc::now();
        self.devices
            .iter()
            .filter(|entry| {
                let last_seen = entry.value().last_seen;
                let duration = now.signed_duration_since(last_seen);
                duration.num_seconds() < 60
            })
            .count()
    }

    pub async fn export_data(&self, format: &str) -> Result<()> {
        let export_dir = std::path::Path::new(&self.config.export_path);
        std::fs::create_dir_all(export_dir)?;

        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let filename = format!("netpulse_export_{}.{}", timestamp, format);
        let filepath = export_dir.join(&filename);

        let devices: Vec<Device> = self.get_devices();
        let stats = self.get_network_stats().await?;

        match format {
            "json" => {
                let export_data = serde_json::json!({
                    "timestamp": Utc::now().to_rfc3339(),
                    "devices": devices,
                    "stats": stats,
                });
                std::fs::write(&filepath, serde_json::to_string_pretty(&export_data)?)?;
            }
            "csv" => {
                let mut csv_content = String::from("MAC Address,IP Address,Hostname,Device Type,Vendor,First Seen,Last Seen\n");
                for device in devices {
                    csv_content.push_str(&format!(
                        "{},{},{},{},{},{},{}\n",
                        device.mac_address,
                        device.ip_address,
                        device.hostname.as_deref().unwrap_or("Unknown"),
                        device.device_type.as_str(),
                        device.vendor.as_deref().unwrap_or("Unknown"),
                        device.first_seen.to_rfc3339(),
                        device.last_seen.to_rfc3339()
                    ));
                }
                std::fs::write(&filepath, csv_content)?;
            }
            _ => {
                return Err(anyhow::anyhow!("Unsupported export format: {}", format));
            }
        }

        info!("💾 Data exported to: {:?}", filepath);
        Ok(())
    }

    pub fn select_interface(&mut self, interface_name: &str) -> Result<()> {
        if self.interfaces.iter().any(|i| i.name == interface_name) {
            self.selected_interface = Some(interface_name.to_string());
            info!("📡 Selected interface: {}", interface_name);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Interface not found: {}", interface_name))
        }
    }
}
