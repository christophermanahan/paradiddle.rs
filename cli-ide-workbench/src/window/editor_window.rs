//! Implementation of an editor window.

use super::Window;
use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};

/// A simple editor window stub.
pub struct EditorWindow {
    /// Contents of the editor. In Phase 1 this is static; later it will be
    /// backed by a rope data structure.
    buffer: String,
}

impl Default for EditorWindow {
    fn default() -> Self {
        Self {
            buffer: String::from("Welcome to Paradiddle.rs!"),
        }
    }
}

impl Window for EditorWindow {
    fn render(&mut self, frame: &mut Frame, area: Rect) {
        self.render_with_focus(frame, area, false);
    }

    fn render_with_focus(&mut self, frame: &mut Frame, area: Rect, focused: bool) {
        let border_type = if focused {
            BorderType::Thick
        } else {
            BorderType::Plain
        };

        let title = if focused { "Editor [*]" } else { "Editor" };

        let paragraph = Paragraph::new(self.buffer.clone()).block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_type(border_type),
        );
        frame.render_widget(paragraph, area);
    }
}
