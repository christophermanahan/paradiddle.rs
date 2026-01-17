//! Implementation of a terminal window.

use super::Window;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

/// A stub terminal window.  In later phases this will spawn a PTY and render
/// shell output; for now it displays placeholder text【6955392274892†L521-L533】.
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
        let paragraph = Paragraph::new(self.buffer.clone())
            .block(Block::default().title("Terminal").borders(Borders::ALL));
        frame.render_widget(paragraph, area);
    }
}
