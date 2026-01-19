//! Focus management for the CLI IDE.
//!
//! The `FocusManager` tracks which window currently has focus and emits
//! events when focus changes. This enables decoupled components to react
//! to focus changes without direct coupling.

use cli_ide_base::Event;

use crate::window::WindowId;

/// Event emitted when focus changes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FocusChanged {
    /// The previously focused window, if any.
    pub previous: Option<WindowId>,
    /// The newly focused window, if any.
    pub current: Option<WindowId>,
}

/// Manages window focus state.
///
/// The FocusManager tracks which window has focus and provides an event
/// stream for focus changes. Only one window can have focus at a time.
pub struct FocusManager {
    /// Currently focused window, if any.
    focused: Option<WindowId>,
    /// Event emitted when focus changes.
    on_focus_changed: Event<FocusChanged>,
}

impl Default for FocusManager {
    fn default() -> Self {
        Self::new()
    }
}

impl FocusManager {
    /// Create a new FocusManager with no initial focus.
    pub fn new() -> Self {
        Self {
            focused: None,
            on_focus_changed: Event::new(),
        }
    }

    /// Create a FocusManager with initial focus on the given window.
    pub fn with_focus(id: WindowId) -> Self {
        Self {
            focused: Some(id),
            on_focus_changed: Event::new(),
        }
    }

    /// Get the currently focused window, if any.
    pub fn focused(&self) -> Option<WindowId> {
        self.focused
    }

    /// Set focus to the given window.
    ///
    /// Emits a `FocusChanged` event if the focus actually changes.
    pub fn set_focus(&mut self, id: WindowId) {
        let previous = self.focused;
        if previous != Some(id) {
            self.focused = Some(id);
            self.on_focus_changed.emit(FocusChanged {
                previous,
                current: Some(id),
            });
        }
    }

    /// Clear focus (no window has focus).
    ///
    /// Emits a `FocusChanged` event if there was a previously focused window.
    pub fn clear_focus(&mut self) {
        let previous = self.focused;
        if previous.is_some() {
            self.focused = None;
            self.on_focus_changed.emit(FocusChanged {
                previous,
                current: None,
            });
        }
    }

    /// Check if the given window has focus.
    pub fn is_focused(&self, id: WindowId) -> bool {
        self.focused == Some(id)
    }

    /// Get a reference to the focus changed event for subscribing.
    pub fn on_focus_changed(&self) -> &Event<FocusChanged> {
        &self.on_focus_changed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_initial_focus_is_none() {
        let manager = FocusManager::new();
        assert!(manager.focused().is_none());
    }

    #[test]
    fn test_with_focus_sets_initial() {
        let id = WindowId::new();
        let manager = FocusManager::with_focus(id);
        assert_eq!(manager.focused(), Some(id));
    }

    #[test]
    fn test_set_focus_updates_focused() {
        let mut manager = FocusManager::new();
        let id = WindowId::new();

        manager.set_focus(id);

        assert_eq!(manager.focused(), Some(id));
    }

    #[test]
    fn test_set_focus_changes_focused() {
        let mut manager = FocusManager::new();
        let id1 = WindowId::new();
        let id2 = WindowId::new();

        manager.set_focus(id1);
        assert_eq!(manager.focused(), Some(id1));

        manager.set_focus(id2);
        assert_eq!(manager.focused(), Some(id2));
    }

    #[test]
    fn test_clear_focus_sets_none() {
        let mut manager = FocusManager::new();
        let id = WindowId::new();

        manager.set_focus(id);
        assert!(manager.focused().is_some());

        manager.clear_focus();
        assert!(manager.focused().is_none());
    }

    #[test]
    fn test_is_focused_returns_correct_state() {
        let mut manager = FocusManager::new();
        let id1 = WindowId::new();
        let id2 = WindowId::new();

        assert!(!manager.is_focused(id1));
        assert!(!manager.is_focused(id2));

        manager.set_focus(id1);
        assert!(manager.is_focused(id1));
        assert!(!manager.is_focused(id2));

        manager.set_focus(id2);
        assert!(!manager.is_focused(id1));
        assert!(manager.is_focused(id2));
    }

    #[test]
    fn test_focus_change_emits_event() {
        let mut manager = FocusManager::new();
        let receiver = manager.on_focus_changed().subscribe();
        let id = WindowId::new();

        manager.set_focus(id);

        let event = receiver.recv_timeout(Duration::from_millis(100)).unwrap();
        assert_eq!(event.previous, None);
        assert_eq!(event.current, Some(id));
    }

    #[test]
    fn test_focus_change_emits_event_with_previous() {
        let mut manager = FocusManager::new();
        let receiver = manager.on_focus_changed().subscribe();
        let id1 = WindowId::new();
        let id2 = WindowId::new();

        manager.set_focus(id1);
        let _ = receiver.recv_timeout(Duration::from_millis(100)).unwrap();

        manager.set_focus(id2);
        let event = receiver.recv_timeout(Duration::from_millis(100)).unwrap();
        assert_eq!(event.previous, Some(id1));
        assert_eq!(event.current, Some(id2));
    }

    #[test]
    fn test_clear_focus_emits_event() {
        let mut manager = FocusManager::new();
        let id = WindowId::new();
        manager.set_focus(id);

        let receiver = manager.on_focus_changed().subscribe();
        manager.clear_focus();

        let event = receiver.recv_timeout(Duration::from_millis(100)).unwrap();
        assert_eq!(event.previous, Some(id));
        assert_eq!(event.current, None);
    }

    #[test]
    fn test_set_same_focus_does_not_emit() {
        let mut manager = FocusManager::new();
        let id = WindowId::new();
        manager.set_focus(id);

        let receiver = manager.on_focus_changed().subscribe();

        // Setting the same focus should not emit
        manager.set_focus(id);

        assert!(receiver.recv_timeout(Duration::from_millis(50)).is_err());
    }

    #[test]
    fn test_clear_no_focus_does_not_emit() {
        let mut manager = FocusManager::new();
        let receiver = manager.on_focus_changed().subscribe();

        // Clearing when already no focus should not emit
        manager.clear_focus();

        assert!(receiver.recv_timeout(Duration::from_millis(50)).is_err());
    }
}
