//! Input abstraction layer for the CLI IDE.
//!
//! This module provides a terminal-agnostic input representation, decoupling
//! the application core from specific terminal libraries like crossterm.

/// Application-level key representation.
///
/// This enum abstracts over terminal-specific key codes, allowing the app core
/// to be tested without a real terminal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppKey {
    /// The 'q' key - typically used for quit
    Q,
    /// Escape key
    Esc,
    /// Tab key - used for focus switching
    Tab,
    /// A character key not specifically handled
    Char(char),
    /// Arrow keys
    Up,
    Down,
    Left,
    Right,
    /// Enter/Return key
    Enter,
    /// Backspace key
    Backspace,
    /// Any other key we don't specifically handle
    Other,
}

/// Application-level event representation.
///
/// Decoupled from crossterm events to enable testing without a TTY.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppEvent {
    /// A key was pressed
    Key(AppKey),
    /// Terminal was resized to (width, height)
    Resize(u16, u16),
    /// Tick event for periodic updates (optional, for animations/polling)
    Tick,
}

impl AppKey {
    /// Create an AppKey from a character.
    pub fn from_char(c: char) -> Self {
        match c {
            'q' | 'Q' => AppKey::Q,
            '\t' => AppKey::Tab,
            '\n' | '\r' => AppKey::Enter,
            c => AppKey::Char(c),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_key_from_char() {
        assert_eq!(AppKey::from_char('q'), AppKey::Q);
        assert_eq!(AppKey::from_char('Q'), AppKey::Q);
        assert_eq!(AppKey::from_char('\t'), AppKey::Tab);
        assert_eq!(AppKey::from_char('a'), AppKey::Char('a'));
    }

    #[test]
    fn test_app_event_equality() {
        assert_eq!(AppEvent::Key(AppKey::Q), AppEvent::Key(AppKey::Q));
        assert_eq!(AppEvent::Resize(80, 24), AppEvent::Resize(80, 24));
        assert_ne!(AppEvent::Key(AppKey::Q), AppEvent::Key(AppKey::Esc));
    }
}
