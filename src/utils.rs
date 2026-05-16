pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    
    if bytes == 0 {
        return "0 B".to_string();
    }
    
    let exp = (bytes as f64).log(1024.0).min(UNITS.len() as f64 - 1.0) as usize;
    let value = bytes as f64 / 1024f64.powi(exp as i32);
    
    format!("{:.2} {}", value, UNITS[exp])
}

pub fn format_duration(seconds: i64) -> String {
    if seconds < 60 {
        format!("{}s", seconds)
    } else if seconds < 3600 {
        format!("{}m {}s", seconds / 60, seconds % 60)
    } else if seconds < 86400 {
        format!("{}h {}m", seconds / 3600, (seconds % 3600) / 60)
    } else {
        format!("{}d {}h", seconds / 86400, (seconds % 86400) / 3600)
    }
}

pub fn validate_mac_address(mac: &str) -> bool {
    let parts: Vec<&str> = mac.split(':').collect();
    
    if parts.len() != 6 {
        return false;
    }
    
    parts.iter().all(|part| {
        part.len() == 2 && part.chars().all(|c| c.is_ascii_hexdigit())
    })
}

pub fn validate_ip_address(ip: &str) -> bool {
    let parts: Vec<&str> = ip.split('.').collect();
    
    if parts.len() != 4 {
        return false;
    }
    
    parts.iter().all(|part| {
        part.parse::<u8>().is_ok()
    })
}
