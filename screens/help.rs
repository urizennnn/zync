pub mod help_popup {
    use ratatui::{
        Frame,
        layout::{Alignment, Rect},
        style::{Color, Style},
        text::{Line, Span},
        widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
    };

    use crate::utils::calculate::calculate_popup_area;

    #[derive(Debug, Clone)]
    pub struct HelpPopup {
        pub title: String,
        pub message: String,
    }

    impl HelpPopup {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self {
                title: "Help".to_string(),
                message: "".to_string(),
            }
        }
        pub fn draw_dashboard_help(&mut self, frame: &mut Frame) {
            let area = calculate_popup_area(frame.area(), 50, 40);
            let topic = Line::from(vec![Span::styled(
                "-- Dashboard Help --",
                Style::default().fg(Color::Green),
            )])
            .centered();

            let keybindings = vec![
                Line::from(vec![
                    Span::styled("q", Style::default().fg(Color::LightBlue)),
                    Span::styled(
                        ": This keybind is used to quit the application",
                        Style::default().fg(Color::Gray),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("n", Style::default().fg(Color::LightBlue)),
                    Span::styled(
                        ": This keybind is used to start a new file sharing session or connect to over a protocol",
                        Style::default().fg(Color::Gray),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("s", Style::default().fg(Color::LightBlue)),
                    Span::styled(
                        ": This keybind is used to stop the current file sharing session",
                        Style::default().fg(Color::Gray),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("Esc", Style::default().fg(Color::LightBlue)),
                    Span::styled(
                        ": This keybind is used to close any open popup menu",
                        Style::default().fg(Color::Gray),
                    ),
                ]),
            ];

            let content = vec![Line::from(""), topic, Line::from("")]
                .into_iter()
                .chain(keybindings)
                .collect::<Vec<Line>>();

            let help_popup = Paragraph::new(content)
                .block(
                    Block::default()
                        .title(
                            Line::from(vec![Span::styled(
                                "Help",
                                Style::default().fg(Color::Gray),
                            )])
                            .centered(),
                        )
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .border_style(Style::default().fg(Color::Gray)),
                )
                .alignment(ratatui::layout::Alignment::Left)
                .wrap(Wrap { trim: true });

            frame.render_widget(Clear, area);
            frame.render_widget(help_popup, area);

            if let Some(footer_area) = HelpPopup::get_footer_area(frame.area(), area) {
                let footer_text = "Press '?' key to close this popup";
                let footer = Paragraph::new(Line::from(vec![Span::styled(
                    footer_text,
                    Style::default().fg(Color::DarkGray),
                )]))
                .alignment(Alignment::Center);
                frame.render_widget(footer, footer_area);
            }
        }

        fn get_footer_area(main_area: Rect, popup_area: Rect) -> Option<Rect> {
            if popup_area.bottom() + 1 >= main_area.bottom() {
                return None;
            }

            Some(Rect {
                x: popup_area.x,
                y: popup_area.bottom(),
                width: popup_area.width,
                height: 1,
            })
        }
    }
}
