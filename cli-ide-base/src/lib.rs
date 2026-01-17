//! Common primitives for the CLI IDE.
//!
//! This crate provides reusable types such as the event system used across the
//! IDE.  The event system is inspired by VS Code’s event abstractions and
//! includes basic transformations like `map`, `filter`, and `debounce`【6955392274892†L521-L533】.

pub mod event;
