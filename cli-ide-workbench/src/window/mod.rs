//! Window abstractions for the IDE.
//!
//! A `Window` can render itself onto a [`ratatui::Frame`].  Concrete
//! implementations include an `EditorWindow` and a `TerminalWindow`.  In
//! future phases the window system will support layouts, split panes, and
//! tiling algorithms【6955392274892†L521-L533】.

use ratatui::prelude::*;

/// A trait representing a drawable window.
pub trait Window {
    /// Render the window onto the given frame in the specified area.  The
    /// `area` indicates the rectangular region of the terminal where the
    /// window should draw itself.  Implementations should not render
    /// outside this area.
    fn render(&mut self, frame: &mut Frame, area: Rect);
}

mod editor_window;
mod terminal_window;

#[cfg(test)]
mod snapshot_tests;

pub use editor_window::EditorWindow;
pub use terminal_window::TerminalWindow;
