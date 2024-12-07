pub mod protocol_popup {
    use ratatui::{
        layout::{Alignment, Constraint, Direction, Layout, Rect},
        style::{Color, Style},
        text::{Line, Span},
        widgets::{Block, Borders, Clear, Paragraph, Tabs},
        Frame,
    };
    use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};

    use crate::{
        popup::{calculate_popup_area, InputBox},
        widget::TableWidget,
    };

    #[derive(Debug, Clone, Copy, PartialEq, FromRepr, Display, EnumIter)]
    #[repr(usize)]
    pub enum ConnectionType {
        TCP,
        P2P,
    }

    #[derive(Debug)]
    pub struct ConnectionPopup {
        pub selected: ConnectionType,
        pub visible: bool,
        pub input_popup: bool,
    }

    impl ConnectionType {
        pub fn next_val(self) -> Self {
            let index: usize = self as usize;
            let next_index = index.saturating_add(1);
            Self::from_repr(next_index).unwrap_or(self)
        }
        pub fn previous_val(self) -> Self {
            let index: usize = self as usize;
            let next_index = index.saturating_sub(1);
            Self::from_repr(next_index).unwrap_or(self)
        }
    }
    impl ConnectionPopup {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self {
                input_popup: false,
                selected: ConnectionType::TCP,
                visible: true,
            }
        }

        pub fn render(&self, f: &mut Frame) {
            if !self.visible {
                return;
            }

            let area = f.area();
            let popup_area = centered_rect(30, 20, area);

            f.render_widget(Clear, popup_area);

            let popup_block = Block::default()
                .title("Choose Connection Type")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan));

            f.render_widget(popup_block, popup_area);

            let inner_area = popup_area.inner(ratatui::layout::Margin {
                horizontal: (2),
                vertical: (2),
            });
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(2), // Question
                        Constraint::Length(1), // Spacing
                        Constraint::Length(3), // Options
                        Constraint::Length(2), // Instructions
                    ]
                    .as_ref(),
                )
                .split(inner_area);

            let question = Paragraph::new(Line::from(vec![Span::styled(
                "How would you like to connect?",
                Style::default().fg(Color::White),
            )]))
            .alignment(Alignment::Center);
            f.render_widget(question, chunks[0]);

            let titles = ConnectionType::iter()
                .map(|t| t.to_string())
                .collect::<Vec<_>>();
            let tabs = Tabs::new(titles)
                .select(self.selected as usize)
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().fg(Color::Black).bg(Color::Cyan))
                .divider("|");
            f.render_widget(tabs, chunks[2]);

            let instructions = Paragraph::new(Line::from(vec![Span::styled(
                "← → to select • Enter to confirm • Esc to cancel",
                Style::default().fg(Color::DarkGray),
            )]))
            .alignment(Alignment::Center);
            f.render_widget(instructions, chunks[3]);
        }

        pub fn next(&mut self) {
            self.selected = self.selected.next_val();
        }

        pub fn previous(&mut self) {
            self.selected = self.selected.previous_val();
        }
        pub fn return_selected(&self, table: &mut TableWidget) -> ConnectionType {
            table.connection = !table.connection;
            let selected: ConnectionType = self.selected;
            selected
        }

        pub fn draw_input(&mut self, f: &mut Frame) {
            let input_widget = InputBox::new();
            let area = calculate_popup_area(f.area(), 20, 30);
            f.render_widget(Clear, f.area());
            input_widget.draw_in_popup(f, area);
        }
    }

    fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ])
            .split(r);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(popup_layout[1])[1]
    }
}
