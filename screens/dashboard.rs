pub mod dashboard_view {
    use ratatui::{
        layout::{Constraint, Direction, Layout},
        style::{Color, Style},
        text::{Line, Span},
        widgets::{Block, Borders, List, ListItem, Paragraph},
        Frame,
    };

    #[derive(Debug)]
    pub struct Transfer {
        pub name: String,
        pub status: String,
        pub destination: String,
        pub time: String,
    }

    #[derive(Debug)]
    pub struct LogEntry {
        pub level: LogLevel,
        pub message: String,
    }

    #[derive(Debug)]
    pub enum LogLevel {
        Info,
        Status,
        Error,
    }

    #[derive(Debug)]
    pub struct App {
        pub log_entries: Vec<LogEntry>,
        pub transfers: Vec<Transfer>,
        pub input: String,
    }

    pub fn ui(f: &mut Frame, app: &App) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(10),   // Main content
                Constraint::Length(3), // Input
            ])
            .split(f.area());

        // Render header
        let header =
            Paragraph::new("File Sharing Dashboard - Press 'q' to quit, 'n' for new transfer")
                .style(Style::default().fg(Color::Cyan));
        f.render_widget(header, chunks[0]);

        // Split the main content horizontally
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[1]);

        // Render Activity Log
        let log_items: Vec<ListItem> = app
            .log_entries
            .iter()
            .map(|entry| {
                let color = match entry.level {
                    LogLevel::Info => Color::Green,
                    LogLevel::Status => Color::White,
                    LogLevel::Error => Color::Red,
                };
                ListItem::new(Line::from(vec![
                    Span::styled(
                        format!(r#"[{:?}]"#, entry.level),
                        Style::default().fg(color),
                    ),
                    Span::raw(format!(" {}", entry.message)),
                ]))
            })
            .collect();

        let log = List::new(log_items)
            .block(Block::default().title("Activity Log").borders(Borders::ALL));
        f.render_widget(log, main_chunks[0]);

        // Render Recent File Transfers
        let transfer_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Header
                Constraint::Min(1),    // Transfers list
            ])
            .split(main_chunks[1]);

        let transfers_header = Paragraph::new("Name           Status    Destination         Time")
            .style(Style::default().fg(Color::Cyan));
        f.render_widget(transfers_header, transfer_chunks[0]);

        let transfer_items: Vec<ListItem> = app
            .transfers
            .iter()
            .map(|transfer| {
                ListItem::new(Line::from(vec![
                    Span::raw(format!("{:<15}", transfer.name)),
                    Span::styled(
                        format!("{:<10}", transfer.status),
                        Style::default().fg(if transfer.status == "Sent" {
                            Color::Green
                        } else {
                            Color::Blue
                        }),
                    ),
                    Span::raw(format!("{:<20}", transfer.destination)),
                    Span::raw(transfer.time.clone()),
                ]))
            })
            .collect();

        let transfers = List::new(transfer_items).block(
            Block::default()
                .title("Recent File Transfers")
                .borders(Borders::ALL),
        );
        f.render_widget(transfers, transfer_chunks[1]);

        // Render input area
        let input = Paragraph::new(app.input.clone())
            .style(Style::default())
            .block(Block::default().borders(Borders::ALL).title("Input"));
        f.render_widget(input, chunks[2]);
    }
}
