//! Keybinding router for the CLI IDE.
//!
//! The `KeybindingRouter` manages key-to-action mappings and dispatches
//! key events to the appropriate handlers. It supports global bindings
//! (always active) and context-aware routing based on focus state.

use std::collections::HashMap;

use crate::input::AppKey;

/// Actions that can be triggered by keybindings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    /// Quit the application.
    Quit,
    /// Toggle focus between windows.
    ToggleFocus,
    /// Move focus to the next window.
    FocusNext,
    /// Move focus to the previous window.
    FocusPrev,
    /// No action (key was handled but no action taken).
    None,
}

/// Routes key events to actions based on registered bindings.
///
/// The router maintains a set of global bindings that are always active
/// regardless of which window has focus. Future versions will support
/// context-specific bindings based on the focused window.
pub struct KeybindingRouter {
    /// Global keybindings (always active).
    global_bindings: HashMap<AppKey, Action>,
}

impl Default for KeybindingRouter {
    fn default() -> Self {
        Self::new()
    }
}

impl KeybindingRouter {
    /// Create a new router with default global bindings.
    ///
    /// Default bindings:
    /// - `Q` / `Esc` → Quit
    /// - `Tab` → ToggleFocus
    pub fn new() -> Self {
        let mut router = Self {
            global_bindings: HashMap::new(),
        };

        // Register default bindings
        router.register_global(AppKey::Q, Action::Quit);
        router.register_global(AppKey::Esc, Action::Quit);
        router.register_global(AppKey::Tab, Action::ToggleFocus);

        router
    }

    /// Create an empty router with no bindings.
    pub fn empty() -> Self {
        Self {
            global_bindings: HashMap::new(),
        }
    }

    /// Register a global keybinding.
    ///
    /// Global bindings are always active regardless of focus state.
    /// If the key was already bound, the old binding is replaced.
    pub fn register_global(&mut self, key: AppKey, action: Action) {
        self.global_bindings.insert(key, action);
    }

    /// Unregister a global keybinding.
    ///
    /// Returns the previously bound action, if any.
    pub fn unregister_global(&mut self, key: AppKey) -> Option<Action> {
        self.global_bindings.remove(&key)
    }

    /// Dispatch a key event and return the action to take.
    ///
    /// Returns `Some(Action)` if the key matches a global binding,
    /// `None` if the key is not bound.
    pub fn dispatch(&self, key: AppKey) -> Option<Action> {
        self.global_bindings.get(&key).copied()
    }

    /// Check if a key has a global binding.
    pub fn is_globally_bound(&self, key: AppKey) -> bool {
        self.global_bindings.contains_key(&key)
    }

    /// Get all global bindings.
    pub fn global_bindings(&self) -> &HashMap<AppKey, Action> {
        &self.global_bindings
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_bindings() {
        let router = KeybindingRouter::new();

        assert_eq!(router.dispatch(AppKey::Q), Some(Action::Quit));
        assert_eq!(router.dispatch(AppKey::Esc), Some(Action::Quit));
        assert_eq!(router.dispatch(AppKey::Tab), Some(Action::ToggleFocus));
    }

    #[test]
    fn test_empty_router() {
        let router = KeybindingRouter::empty();

        assert_eq!(router.dispatch(AppKey::Q), None);
        assert_eq!(router.dispatch(AppKey::Esc), None);
        assert_eq!(router.dispatch(AppKey::Tab), None);
    }

    #[test]
    fn test_register_global() {
        let mut router = KeybindingRouter::empty();

        router.register_global(AppKey::Char('h'), Action::FocusPrev);

        assert_eq!(router.dispatch(AppKey::Char('h')), Some(Action::FocusPrev));
    }

    #[test]
    fn test_register_overwrites() {
        let mut router = KeybindingRouter::new();

        // Q is bound to Quit by default
        assert_eq!(router.dispatch(AppKey::Q), Some(Action::Quit));

        // Overwrite with ToggleFocus
        router.register_global(AppKey::Q, Action::ToggleFocus);

        assert_eq!(router.dispatch(AppKey::Q), Some(Action::ToggleFocus));
    }

    #[test]
    fn test_unregister_global() {
        let mut router = KeybindingRouter::new();

        let removed = router.unregister_global(AppKey::Q);
        assert_eq!(removed, Some(Action::Quit));
        assert_eq!(router.dispatch(AppKey::Q), None);
    }

    #[test]
    fn test_unregister_nonexistent() {
        let mut router = KeybindingRouter::new();

        let removed = router.unregister_global(AppKey::Char('x'));
        assert_eq!(removed, None);
    }

    #[test]
    fn test_dispatch_unbound_key() {
        let router = KeybindingRouter::new();

        assert_eq!(router.dispatch(AppKey::Char('a')), None);
        assert_eq!(router.dispatch(AppKey::Up), None);
        assert_eq!(router.dispatch(AppKey::Enter), None);
    }

    #[test]
    fn test_is_globally_bound() {
        let router = KeybindingRouter::new();

        assert!(router.is_globally_bound(AppKey::Q));
        assert!(router.is_globally_bound(AppKey::Esc));
        assert!(router.is_globally_bound(AppKey::Tab));
        assert!(!router.is_globally_bound(AppKey::Char('a')));
    }

    #[test]
    fn test_global_bindings_accessor() {
        let router = KeybindingRouter::new();
        let bindings = router.global_bindings();

        assert_eq!(bindings.len(), 3);
        assert_eq!(bindings.get(&AppKey::Q), Some(&Action::Quit));
    }

    #[test]
    fn test_action_equality() {
        assert_eq!(Action::Quit, Action::Quit);
        assert_ne!(Action::Quit, Action::ToggleFocus);
    }
}
