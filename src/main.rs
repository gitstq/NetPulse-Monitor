use anyhow::Result;
use clap::Parser;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

mod app;
mod config;
mod device;
mod network;
mod scanner;
mod tui;
mod ui;
mod utils;

use app::App;
use config::Config;

#[derive(Parser)]
#[command(
    name = "netpulse",
    version = "1.0.0",
    about = "🚀 NetPulse - Intelligent Network Monitoring & Device Management Tool"
)]
struct Cli {
    #[arg(short, long)]
    interface: Option<String>,
    #[arg(short, long)]
    scan: Option<String>,
    #[arg(short, long)]
    export: Option<String>,
    #[arg(short, long)]
    config: Option<String>,
    #[arg(short, long)]
    verbose: bool,
    #[arg(long)]
    headless: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let subscriber = tracing_subscriber::fmt()
        .with_max_level(if cli.verbose {
            tracing::Level::DEBUG
        } else {
            tracing::Level::INFO
        })
        .finish();
    
    tracing::subscriber::set_global_default(subscriber)?;

    info!("🚀 Starting NetPulse v1.0.0");

    let config = if let Some(config_path) = cli.config {
        Config::from_file(&config_path)?
    } else {
        Config::default()
    };

    let app = Arc::new(RwLock::new(App::new(config)));

    if let Some(scan_range) = cli.scan {
        info!("🔍 Scanning network range: {}", scan_range);
        let mut app_guard = app.write().await;
        app_guard.scan_network(&scan_range).await?;
        
        if let Some(export_format) = cli.export {
            app_guard.export_data(&export_format).await?;
        }
        
        return Ok(());
    }

    if cli.headless {
        info!("📊 Running in headless mode");
        run_headless(app).await?;
    } else {
        info!("🖥️  Starting TUI interface");
        tui::run(app).await?;
    }

    Ok(())
}

async fn run_headless(app: Arc<RwLock<App>>) -> Result<()> {
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(5));
    
    loop {
        interval.tick().await;
        
        let app_guard = app.read().await;
        let stats = app_guard.get_network_stats().await?;
        
        println!("📊 Network Stats:");
        println!("  Download: {:.2} MB/s", stats.download_speed);
        println!("  Upload: {:.2} MB/s", stats.upload_speed);
        println!("  Active Devices: {}", stats.active_devices);
        println!("  Total Packets: {}", stats.total_packets);
        println!("---");
    }
}
