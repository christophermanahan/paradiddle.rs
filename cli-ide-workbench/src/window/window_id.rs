//! Unique window identifier type.
//!
//! WindowId provides a lightweight, unique identifier for each window instance.
//! IDs are generated using an atomic counter to ensure thread-safe uniqueness.

use std::sync::atomic::{AtomicU64, Ordering};

/// Global counter for generating unique window IDs.
static NEXT_ID: AtomicU64 = AtomicU64::new(1);

/// A unique identifier for a window.
///
/// WindowId is a lightweight, copyable identifier that can be used as a key
/// in collections. Each ID is guaranteed to be unique within a process.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WindowId(u64);

impl WindowId {
    /// Create a new unique WindowId.
    ///
    /// Each call returns a distinct ID. IDs are never reused within a process.
    pub fn new() -> Self {
        Self(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }

    /// Get the raw u64 value of this ID.
    ///
    /// Useful for debugging and logging.
    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

impl Default for WindowId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for WindowId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WindowId({})", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_window_id_unique() {
        let id1 = WindowId::new();
        let id2 = WindowId::new();
        let id3 = WindowId::new();

        assert_ne!(id1, id2);
        assert_ne!(id2, id3);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_window_id_equality() {
        let id = WindowId::new();
        let id_copy = id;

        assert_eq!(id, id_copy);
    }

    #[test]
    fn test_window_id_hash() {
        let id1 = WindowId::new();
        let id2 = WindowId::new();

        let mut set = HashSet::new();
        set.insert(id1);
        set.insert(id2);

        assert!(set.contains(&id1));
        assert!(set.contains(&id2));
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_window_id_display() {
        let id = WindowId::new();
        let display = format!("{}", id);

        assert!(display.starts_with("WindowId("));
        assert!(display.ends_with(")"));
    }

    #[test]
    fn test_window_id_as_u64() {
        let id = WindowId::new();
        let raw = id.as_u64();

        assert!(raw > 0);
    }

    #[test]
    fn test_window_id_default() {
        let id1 = WindowId::default();
        let id2 = WindowId::default();

        assert_ne!(id1, id2); // Default should also generate unique IDs
    }
}
