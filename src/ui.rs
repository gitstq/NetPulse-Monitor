pub fn print_banner() {
    println!(r#"
    ╔═══════════════════════════════════════════════════════════╗
    ║                                                           ║
    ║   🚀  NetPulse - Intelligent Network Monitor              ║
    ║                                                           ║
    ║   Real-time network monitoring & device discovery         ║
    ║                                                           ║
    ╚═══════════════════════════════════════════════════════════╝
    "#);
}

pub fn print_help() {
    println!(r#"
Usage: netpulse [OPTIONS]

Options:
    -i, --interface <NAME>    Network interface to monitor
    -s, --scan <RANGE>        Scan network range (e.g., 192.168.1.0/24)
    -e, --export <FORMAT>     Export data (json, csv)
    -c, --config <PATH>       Configuration file path
    -v, --verbose             Enable verbose logging
        --headless            Run in headless mode (no TUI)
    -h, --help                Print help
    -V, --version             Print version

Examples:
    netpulse                          # Launch interactive TUI
    netpulse --interface eth0         # Monitor specific interface
    netpulse --scan 192.168.1.0/24    # Scan network range
    netpulse --export json            # Export data to JSON
    "#);
}
