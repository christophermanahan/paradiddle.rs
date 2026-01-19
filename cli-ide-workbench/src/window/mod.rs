//! Window abstractions for the IDE.
//!
//! A `Window` can render itself onto a [`ratatui::Frame`].  Concrete
//! implementations include an `EditorWindow` and a `TerminalWindow`.  In
//! future phases the window system will support layouts, split panes, and
//! tiling algorithms.

use ratatui::prelude::*;

mod editor_window;
mod terminal_window;
mod window_id;

#[cfg(test)]
mod snapshot_tests;

pub use editor_window::EditorWindow;
pub use terminal_window::TerminalWindow;
pub use window_id::WindowId;

/// A trait representing a drawable window.
pub trait Window {
    /// Render the window onto the given frame in the specified area.
    ///
    /// The `area` indicates the rectangular region of the terminal where the
    /// window should draw itself. Implementations should not render outside
    /// this area.
    fn render(&mut self, frame: &mut Frame, area: Rect);

    /// Render the window with focus state.
    ///
    /// The `focused` parameter indicates whether this window currently has
    /// focus. Implementations should provide visual feedback (e.g., different
    /// border style) when focused.
    ///
    /// Default implementation ignores focus state and calls `render()`.
    fn render_with_focus(&mut self, frame: &mut Frame, area: Rect, focused: bool) {
        let _ = focused; // Default implementation ignores focus
        self.render(frame, area);
    }
}
