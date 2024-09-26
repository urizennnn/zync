pub mod custom_button_popup {
    use color_eyre::owo_colors::OwoColorize;
    use ratatui::{text::Line, widgets::Widget};

    #[derive(Debug, Clone)]
    pub struct Button<'b> {
        pub text: Line<'b>,
        pub theme: Theme,
        pub selected: bool,
        pub state: State,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Copy)]
    pub enum State {
        Normal,
        Selected,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Theme {
        text: ratatui::style::Color,
        background: ratatui::style::Color,
    }

    const YES: Theme = Theme {
        text: ratatui::style::Color::Yellow,
        background: ratatui::style::Color::Black,
    };
    const NO: Theme = Theme {
        text: ratatui::style::Color::Yellow,
        background: ratatui::style::Color::Black,
    };
    const ENTER: Theme = Theme {
        text: ratatui::style::Color::Green,
        background: ratatui::style::Color::Black,
    };

    impl<'b> Button<'b> {
        pub fn new<T: Into<Line<'b>>>(text: T) -> Self {
            Self {
                text: text.into(),
                theme: YES,
                selected: false,
                state: State::Normal,
            }
        }

        pub const fn theme(mut self, theme: Theme) -> Self {
            self.theme = theme;
            self
        }

        pub const fn state(mut self, state: State) -> Self {
            self.state = state;
            self
        }
    }

    impl<'b> Widget for Button<'b> {
        #[allow(clippy::cast_possible_truncation)]
        fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {}
    }
}
