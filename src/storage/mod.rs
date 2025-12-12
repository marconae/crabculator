//! Storage module for state persistence.
//!
//! Provides functionality for saving and loading application state.

pub mod paths;
pub mod state;

pub use paths::{state_dir, state_file};
pub use state::{PersistedState, load, load_from_path, save, save_to_path};
