//! Implementation of an editor window.

use super::Window;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

/// A simple editor window stub.
pub struct EditorWindow {
    /// Contents of the editor.  In Phase 1 this is static; later it will be
    /// backed by a rope data structure【6955392274892†L521-L533】.
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
        let paragraph = Paragraph::new(self.buffer.clone())
            .block(Block::default().title("Editor").borders(Borders::ALL));
        frame.render_widget(paragraph, area);
    }
}
