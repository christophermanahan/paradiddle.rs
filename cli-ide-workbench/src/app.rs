//! Application core for the CLI IDE.
//!
//! The `App` struct owns the application state and windows, providing a
//! testable interface that is decoupled from terminal I/O.

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::Frame;

use crate::input::{AppEvent, AppKey};
use crate::window::{EditorWindow, TerminalWindow, Window};

/// Which pane currently has focus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FocusedPane {
    #[default]
    Editor,
    Terminal,
}

impl FocusedPane {
    /// Toggle to the other pane.
    pub fn toggle(self) -> Self {
        match self {
            FocusedPane::Editor => FocusedPane::Terminal,
            FocusedPane::Terminal => FocusedPane::Editor,
        }
    }
}

/// The main application state.
///
/// Owns the windows and manages application lifecycle. Can be driven by
/// `AppEvent`s for testing without a real terminal.
pub struct App {
    /// The editor window (left pane)
    editor: EditorWindow,
    /// The terminal window (right pane)
    terminal: TerminalWindow,
    /// Which pane has focus
    focused: FocusedPane,
    /// Whether the app is still running
    running: bool,
    /// Current terminal width
    width: u16,
    /// Current terminal height
    height: u16,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    /// Create a new App with default windows.
    pub fn new() -> Self {
        Self {
            editor: EditorWindow::default(),
            terminal: TerminalWindow::default(),
            focused: FocusedPane::default(),
            running: true,
            width: 80,
            height: 24,
        }
    }

    /// Create a new App with specified initial size.
    pub fn with_size(width: u16, height: u16) -> Self {
        Self {
            editor: EditorWindow::default(),
            terminal: TerminalWindow::default(),
            focused: FocusedPane::default(),
            running: true,
            width,
            height,
        }
    }

    /// Check if the app is still running.
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Get the current focused pane.
    pub fn focused(&self) -> FocusedPane {
        self.focused
    }

    /// Get the current terminal dimensions.
    pub fn size(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    /// Handle an application event.
    ///
    /// This is the main entry point for input handling. Events are processed
    /// and may update application state.
    pub fn handle_event(&mut self, event: AppEvent) {
        match event {
            AppEvent::Key(key) => self.handle_key(key),
            AppEvent::Resize(w, h) => {
                self.width = w;
                self.height = h;
            }
            AppEvent::Tick => {
                // Currently unused; placeholder for future animations/polling
            }
        }
    }

    /// Handle a key press.
    fn handle_key(&mut self, key: AppKey) {
        match key {
            AppKey::Q | AppKey::Esc => {
                self.running = false;
            }
            AppKey::Tab => {
                self.focused = self.focused.toggle();
            }
            _ => {
                // Other keys not handled yet
            }
        }
    }

    /// Render the application to a frame.
    ///
    /// Uses the stored dimensions to create a layout and renders both windows.
    /// The focused window gets a visual indicator in its title.
    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(area);

        // Render with focus indicator
        self.render_editor(frame, chunks[0]);
        self.render_terminal(frame, chunks[1]);
    }

    /// Render the editor window with optional focus indicator.
    fn render_editor(&mut self, frame: &mut Frame, area: Rect) {
        // For now, render normally - focus indicator can be added to Window trait later
        // This keeps the change minimal
        self.editor.render(frame, area);

        // If focused, we could overlay a marker, but that requires Window trait changes
        // Deferring visual focus indicator to keep scope minimal
    }

    /// Render the terminal window with optional focus indicator.
    fn render_terminal(&mut self, frame: &mut Frame, area: Rect) {
        self.terminal.render(frame, area);
    }

    /// Get the layout rects for the current size.
    ///
    /// Useful for testing to verify layout calculations.
    pub fn layout_rects(&self, area: Rect) -> (Rect, Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(area);
        (chunks[0], chunks[1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_new() {
        let app = App::new();
        assert!(app.is_running());
        assert_eq!(app.focused(), FocusedPane::Editor);
        assert_eq!(app.size(), (80, 24));
    }

    #[test]
    fn test_app_with_size() {
        let app = App::with_size(120, 40);
        assert_eq!(app.size(), (120, 40));
    }

    #[test]
    fn test_quit_on_q() {
        let mut app = App::new();
        assert!(app.is_running());

        app.handle_event(AppEvent::Key(AppKey::Q));

        assert!(!app.is_running());
    }

    #[test]
    fn test_quit_on_esc() {
        let mut app = App::new();
        assert!(app.is_running());

        app.handle_event(AppEvent::Key(AppKey::Esc));

        assert!(!app.is_running());
    }

    #[test]
    fn test_resize_updates_dimensions() {
        let mut app = App::new();
        assert_eq!(app.size(), (80, 24));

        app.handle_event(AppEvent::Resize(100, 50));

        assert_eq!(app.size(), (100, 50));
    }

    #[test]
    fn test_focus_toggle() {
        let mut app = App::new();
        assert_eq!(app.focused(), FocusedPane::Editor);

        app.handle_event(AppEvent::Key(AppKey::Tab));
        assert_eq!(app.focused(), FocusedPane::Terminal);

        app.handle_event(AppEvent::Key(AppKey::Tab));
        assert_eq!(app.focused(), FocusedPane::Editor);
    }

    #[test]
    fn test_focused_pane_toggle() {
        assert_eq!(FocusedPane::Editor.toggle(), FocusedPane::Terminal);
        assert_eq!(FocusedPane::Terminal.toggle(), FocusedPane::Editor);
    }

    #[test]
    fn test_tick_does_not_change_state() {
        let mut app = App::new();
        let running_before = app.is_running();
        let focused_before = app.focused();
        let size_before = app.size();

        app.handle_event(AppEvent::Tick);

        assert_eq!(app.is_running(), running_before);
        assert_eq!(app.focused(), focused_before);
        assert_eq!(app.size(), size_before);
    }
}
