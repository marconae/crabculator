//! Expression editor module for multi-line text editing.
//!
//! Provides a text buffer and cursor management for entering mathematical expressions.

mod buffer;
mod cursor;

pub use buffer::Buffer;
pub use cursor::Cursor;
