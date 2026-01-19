//! Common primitives for the CLI IDE.
//!
//! This crate provides reusable types such as the event system used across the
//! IDE. The event system is inspired by VS Code's event abstractions and
//! includes basic transformations like `map`, `filter`, and `debounce`.

pub mod event;

// Re-export Event for convenience
pub use event::Event;
