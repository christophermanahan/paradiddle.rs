//! Application core for the CLI IDE.
//!
//! The `App` struct owns the application state and windows, providing a
//! testable interface that is decoupled from terminal I/O.

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::Frame;

use crate::focus::FocusManager;
use crate::input::{AppEvent, AppKey};
use crate::keybinding::{Action, KeybindingRouter};
use crate::window::{EditorWindow, TerminalWindow, Window, WindowId};

/// Which pane currently has focus.
///
/// This enum is kept for backward compatibility with existing tests.
/// Internally, the App now uses FocusManager with WindowIds.
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
    /// Editor window ID
    editor_id: WindowId,
    /// Terminal window ID
    terminal_id: WindowId,
    /// Focus manager
    focus_manager: FocusManager,
    /// Keybinding router
    keybinding_router: KeybindingRouter,
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
        let editor_id = WindowId::new();
        let terminal_id = WindowId::new();

        // Start with editor focused
        let focus_manager = FocusManager::with_focus(editor_id);

        Self {
            editor: EditorWindow::default(),
            terminal: TerminalWindow::default(),
            editor_id,
            terminal_id,
            focus_manager,
            keybinding_router: KeybindingRouter::new(),
            running: true,
            width: 80,
            height: 24,
        }
    }

    /// Create a new App with specified initial size.
    pub fn with_size(width: u16, height: u16) -> Self {
        let mut app = Self::new();
        app.width = width;
        app.height = height;
        app
    }

    /// Check if the app is still running.
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Get the current focused pane.
    ///
    /// Returns the FocusedPane enum for backward compatibility.
    pub fn focused(&self) -> FocusedPane {
        match self.focus_manager.focused() {
            Some(id) if id == self.editor_id => FocusedPane::Editor,
            Some(id) if id == self.terminal_id => FocusedPane::Terminal,
            _ => FocusedPane::Editor, // Default to editor if unknown
        }
    }

    /// Get the focused window ID.
    pub fn focused_id(&self) -> Option<WindowId> {
        self.focus_manager.focused()
    }

    /// Get the editor window ID.
    pub fn editor_id(&self) -> WindowId {
        self.editor_id
    }

    /// Get the terminal window ID.
    pub fn terminal_id(&self) -> WindowId {
        self.terminal_id
    }

    /// Get a reference to the focus manager.
    pub fn focus_manager(&self) -> &FocusManager {
        &self.focus_manager
    }

    /// Get a reference to the keybinding router.
    pub fn keybinding_router(&self) -> &KeybindingRouter {
        &self.keybinding_router
    }

    /// Get a mutable reference to the keybinding router.
    pub fn keybinding_router_mut(&mut self) -> &mut KeybindingRouter {
        &mut self.keybinding_router
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

    /// Handle a key press using the keybinding router.
    fn handle_key(&mut self, key: AppKey) {
        if let Some(action) = self.keybinding_router.dispatch(key) {
            self.execute_action(action);
        }
        // Keys not bound to actions are ignored (could be forwarded to focused window)
    }

    /// Execute an action.
    fn execute_action(&mut self, action: Action) {
        match action {
            Action::Quit => {
                self.running = false;
            }
            Action::ToggleFocus => {
                self.toggle_focus();
            }
            Action::FocusNext => {
                self.toggle_focus(); // With only 2 windows, next == toggle
            }
            Action::FocusPrev => {
                self.toggle_focus(); // With only 2 windows, prev == toggle
            }
            Action::None => {
                // Do nothing
            }
        }
    }

    /// Toggle focus between editor and terminal.
    fn toggle_focus(&mut self) {
        let current = self.focus_manager.focused();
        let next = match current {
            Some(id) if id == self.editor_id => self.terminal_id,
            _ => self.editor_id,
        };
        self.focus_manager.set_focus(next);
    }

    /// Render the application to a frame.
    ///
    /// Uses the stored dimensions to create a layout and renders both windows.
    /// The focused window gets a visual indicator.
    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(area);

        // Render with focus indicators
        let editor_focused = self.focus_manager.is_focused(self.editor_id);
        let terminal_focused = self.focus_manager.is_focused(self.terminal_id);

        self.editor
            .render_with_focus(frame, chunks[0], editor_focused);
        self.terminal
            .render_with_focus(frame, chunks[1], terminal_focused);
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

    #[test]
    fn test_window_ids_are_unique() {
        let app = App::new();
        assert_ne!(app.editor_id(), app.terminal_id());
    }

    #[test]
    fn test_focused_id_tracks_editor() {
        let app = App::new();
        assert_eq!(app.focused_id(), Some(app.editor_id()));
    }

    #[test]
    fn test_focused_id_tracks_terminal() {
        let mut app = App::new();
        app.handle_event(AppEvent::Key(AppKey::Tab));
        assert_eq!(app.focused_id(), Some(app.terminal_id()));
    }

    #[test]
    fn test_focus_manager_accessible() {
        let app = App::new();
        assert!(app.focus_manager().is_focused(app.editor_id()));
    }

    #[test]
    fn test_keybinding_router_accessible() {
        let app = App::new();
        assert!(app.keybinding_router().is_globally_bound(AppKey::Q));
    }

    #[test]
    fn test_keybinding_router_mutable() {
        let mut app = App::new();
        app.keybinding_router_mut()
            .register_global(AppKey::Char('x'), Action::Quit);
        assert!(app.keybinding_router().is_globally_bound(AppKey::Char('x')));
    }
}
