use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame, Terminal,
};
use std::io;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

use crate::app::App;

pub async fn run(app: Arc<RwLock<App>>) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(250);

    loop {
        terminal.draw(|f| {
            let app_guard = app.try_read();
            if let Ok(app) = app_guard {
                draw_ui(f, &app);
            }
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Char('s') => {
                            let mut app_guard = app.write().await;
                            if let Err(e) = app_guard.scan_network("192.168.1.0/24").await {
                                eprintln!("Scan error: {}", e);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn draw_ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(10),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(f.size());

    draw_title(f, chunks[0]);
    draw_stats(f, app, chunks[1]);
    draw_device_list(f, app, chunks[2]);
    draw_footer(f, chunks[3]);
}

fn draw_title(f: &mut Frame, area: Rect) {
    let title = Paragraph::new("🚀 NetPulse - Network Monitor")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, area);
}

fn draw_stats(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(area);

    let device_count = app.get_device_count();
    let device_widget = Paragraph::new(format!("📱 Devices\n\n{}", device_count))
        .style(Style::default().fg(Color::Green))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Devices"));
    f.render_widget(device_widget, chunks[0]);

    let active_count = app.get_active_device_count();
    let active_widget = Paragraph::new(format!("🟢 Active\n\n{}", active_count))
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Active"));
    f.render_widget(active_widget, chunks[1]);

    let interface_count = app.interfaces.len();
    let interface_widget = Paragraph::new(format!("🌐 Interfaces\n\n{}", interface_count))
        .style(Style::default().fg(Color::Blue))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Interfaces"));
    f.render_widget(interface_widget, chunks[2]);

    let last_scan = app
        .last_scan
        .map(|t| t.format("%H:%M:%S").to_string())
        .unwrap_or_else(|| "Never".to_string());
    let scan_widget = Paragraph::new(format!("🔍 Last Scan\n\n{}", last_scan))
        .style(Style::default().fg(Color::Magenta))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Scan"));
    f.render_widget(scan_widget, chunks[3]);
}

fn draw_device_list(f: &mut Frame, app: &App, area: Rect) {
    let devices = app.get_devices();

    let header = Row::new(vec!["Icon", "MAC Address", "IP Address", "Hostname", "Vendor", "Type"])
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .height(1);

    let rows: Vec<Row> = devices
        .iter()
        .map(|device| {
            Row::new(vec![
                Cell::from(device.device_type.icon()),
                Cell::from(device.mac_address.clone()),
                Cell::from(device.ip_address.clone()),
                Cell::from(device.hostname.clone().unwrap_or_else(|| "Unknown".to_string())),
                Cell::from(device.vendor.clone().unwrap_or_else(|| "Unknown".to_string())),
                Cell::from(device.device_type.as_str().to_string()),
            ])
            .style(Style::default().fg(Color::White))
            .height(1)
        })
        .collect();

    let table = Table::new(rows, vec![
        Constraint::Length(5),
        Constraint::Length(18),
        Constraint::Length(16),
        Constraint::Length(20),
        Constraint::Length(15),
        Constraint::Length(15),
    ])
    .header(header)
    .block(Block::default().borders(Borders::ALL).title("Discovered Devices"))
    .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED));

    f.render_widget(table, area);
}

fn draw_footer(f: &mut Frame, area: Rect) {
    let footer = Paragraph::new("Press 's' to scan | 'q' to quit")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, area);
}
