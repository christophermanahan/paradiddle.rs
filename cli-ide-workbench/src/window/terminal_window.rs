//! Implementation of a terminal window.

use super::Window;
use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};

/// A stub terminal window. In later phases this will spawn a PTY and render
/// shell output; for now it displays placeholder text.
pub struct TerminalWindow {
    /// Placeholder output.
    buffer: String,
}

impl Default for TerminalWindow {
    fn default() -> Self {
        Self {
            buffer: String::from("Terminal output will appear here."),
        }
    }
}

impl Window for TerminalWindow {
    fn render(&mut self, frame: &mut Frame, area: Rect) {
        self.render_with_focus(frame, area, false);
    }

    fn render_with_focus(&mut self, frame: &mut Frame, area: Rect, focused: bool) {
        let border_type = if focused {
            BorderType::Thick
        } else {
            BorderType::Plain
        };

        let title = if focused { "Terminal [*]" } else { "Terminal" };

        let paragraph = Paragraph::new(self.buffer.clone()).block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_type(border_type),
        );
        frame.render_widget(paragraph, area);
    }
}
