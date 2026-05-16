use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub interface: Option<String>,
    pub scan_interval: u64,
    pub device_timeout: u64,
    pub max_devices: usize,
    pub enable_fingerprinting: bool,
    pub export_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            interface: None,
            scan_interval: 30,
            device_timeout: 300,
            max_devices: 100,
            enable_fingerprinting: true,
            export_path: "./exports".to_string(),
        }
    }
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}
