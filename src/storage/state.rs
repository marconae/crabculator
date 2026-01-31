//! State persistence for Crabculator.
//!
//! Provides functionality to save and load buffer lines to/from disk as plain text.

use std::fs;
use std::io::{self, ErrorKind};
use std::path::Path;

use super::paths;

/// Represents the persisted application state.
///
/// Contains the buffer lines that should be saved between sessions.
/// Variables are not persisted; they are computed from evaluating buffer lines.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistedState {
    /// The lines from the buffer.
    pub buffer_lines: Vec<String>,
}

impl PersistedState {
    /// Creates a new `PersistedState` with the given buffer lines.
    #[must_use]
    pub const fn new(buffer_lines: Vec<String>) -> Self {
        Self { buffer_lines }
    }

    /// Creates an empty `PersistedState` with no buffer lines.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            buffer_lines: Vec::new(),
        }
    }
}

impl Default for PersistedState {
    fn default() -> Self {
        Self::empty()
    }
}

/// Saves the given state to the state file as plain text.
///
/// Creates the state directory if it doesn't exist.
/// Each buffer line is written as one line in the file.
///
/// # Errors
///
/// Returns an error if:
/// - The state directory cannot be determined
/// - The directory cannot be created
/// - The file cannot be written
pub fn save(state: &PersistedState) -> io::Result<()> {
    let state_dir = paths::state_dir().ok_or_else(|| {
        io::Error::new(
            ErrorKind::NotFound,
            "Could not determine state directory path",
        )
    })?;

    let state_file = paths::state_file().ok_or_else(|| {
        io::Error::new(ErrorKind::NotFound, "Could not determine state file path")
    })?;

    fs::create_dir_all(&state_dir)?;

    let content = state.buffer_lines.join("\n");
    fs::write(&state_file, content)
}

/// Loads the state from the state file.
///
/// # Returns
///
/// - `Ok(Some(state))` if the file exists and contains valid content
/// - `Ok(None)` if the file doesn't exist
/// - `Ok(None)` if the file is corrupted (logs a warning)
///
/// # Errors
///
/// Returns an error if the file exists but cannot be read (e.g., permission denied).
pub fn load() -> io::Result<Option<PersistedState>> {
    let Some(state_file) = paths::state_file() else {
        return Ok(None);
    };

    load_from_path(&state_file)
}

/// Loads state from a specific path as plain text.
///
/// Each line in the file becomes one buffer line.
/// This is primarily used for testing with temporary files.
///
/// # Errors
///
/// Returns an error if the file exists but cannot be read (e.g., permission denied).
pub fn load_from_path(path: &Path) -> io::Result<Option<PersistedState>> {
    let contents = match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(e) if e.kind() == ErrorKind::NotFound => {
            return Ok(None);
        }
        Err(e) if e.kind() == ErrorKind::InvalidData => {
            eprintln!("Warning: State file contains invalid UTF-8, using empty state. Error: {e}");
            return Ok(None);
        }
        Err(e) => return Err(e),
    };

    let buffer_lines: Vec<String> = contents.lines().map(String::from).collect();
    Ok(Some(PersistedState::new(buffer_lines)))
}

/// Saves state to a specific path as plain text.
///
/// Each buffer line is written as one line in the file.
/// This is primarily used for testing with temporary files.
///
/// # Errors
///
/// Returns an error if:
/// - The parent directory cannot be created
/// - The file cannot be written
pub fn save_to_path(state: &PersistedState, path: &Path) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let content = state.buffer_lines.join("\n");
    fs::write(path, content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_persisted_state_new() {
        let lines = vec!["line1".to_string(), "line2".to_string()];
        let state = PersistedState::new(lines.clone());

        assert_eq!(state.buffer_lines, lines);
    }

    #[test]
    fn test_persisted_state_empty() {
        let state = PersistedState::empty();

        assert!(state.buffer_lines.is_empty());
    }

    #[test]
    fn test_persisted_state_default() {
        let state = PersistedState::default();

        assert!(state.buffer_lines.is_empty());
    }

    #[test]
    fn test_save_to_path_creates_file() {
        let dir = tempdir().expect("should create temp dir");
        let file_path = dir.path().join("state.txt");

        let state = PersistedState::empty();
        save_to_path(&state, &file_path).expect("save should succeed");

        assert!(file_path.exists());
    }

    #[test]
    fn test_save_to_path_creates_parent_directories() {
        let dir = tempdir().expect("should create temp dir");
        let file_path = dir.path().join("nested").join("dir").join("state.txt");

        let state = PersistedState::empty();
        save_to_path(&state, &file_path).expect("save should succeed");

        assert!(file_path.exists());
    }

    #[test]
    fn test_save_to_path_writes_plain_text() {
        let dir = tempdir().expect("should create temp dir");
        let file_path = dir.path().join("state.txt");

        let state = PersistedState::new(vec!["1 + 2".to_string(), "x = 5".to_string()]);
        save_to_path(&state, &file_path).expect("save should succeed");

        let contents = fs::read_to_string(&file_path).expect("should read file");
        assert_eq!(contents, "1 + 2\nx = 5");
    }

    #[test]
    fn test_save_to_path_overwrites_existing_file() {
        let dir = tempdir().expect("should create temp dir");
        let file_path = dir.path().join("state.txt");

        let state1 = PersistedState::new(vec!["first".to_string()]);
        save_to_path(&state1, &file_path).expect("first save should succeed");

        let state2 = PersistedState::new(vec!["second".to_string()]);
        save_to_path(&state2, &file_path).expect("second save should succeed");

        let loaded = load_from_path(&file_path)
            .expect("load should succeed")
            .expect("should have state");
        assert_eq!(loaded.buffer_lines, vec!["second"]);
    }

    #[test]
    fn test_load_from_path_returns_none_when_file_missing() {
        let dir = tempdir().expect("should create temp dir");
        let file_path = dir.path().join("nonexistent.txt");

        let result = load_from_path(&file_path).expect("load should not error");

        assert!(result.is_none());
    }

    #[test]
    fn test_load_from_path_returns_state_from_plain_text() {
        let dir = tempdir().expect("should create temp dir");
        let file_path = dir.path().join("state.txt");

        fs::write(&file_path, "hello\nworld").expect("should write file");

        let result = load_from_path(&file_path)
            .expect("load should succeed")
            .expect("should have state");

        assert_eq!(result.buffer_lines, vec!["hello", "world"]);
    }

    #[test]
    fn test_load_from_path_handles_empty_file() {
        let dir = tempdir().expect("should create temp dir");
        let file_path = dir.path().join("state.txt");

        fs::write(&file_path, "").expect("should write file");

        let result = load_from_path(&file_path)
            .expect("load should succeed")
            .expect("should have state");

        assert!(result.buffer_lines.is_empty());
    }

    #[test]
    fn test_load_from_path_handles_single_line() {
        let dir = tempdir().expect("should create temp dir");
        let file_path = dir.path().join("state.txt");

        fs::write(&file_path, "single line").expect("should write file");

        let result = load_from_path(&file_path)
            .expect("load should succeed")
            .expect("should have state");

        assert_eq!(result.buffer_lines, vec!["single line"]);
    }

    #[test]
    fn test_save_and_load_roundtrip() {
        let dir = tempdir().expect("should create temp dir");
        let file_path = dir.path().join("state.txt");

        let original = PersistedState::new(vec!["line1".to_string(), "line2".to_string()]);

        save_to_path(&original, &file_path).expect("save should succeed");
        let loaded = load_from_path(&file_path)
            .expect("load should succeed")
            .expect("should have state");

        assert_eq!(original, loaded);
    }

    #[test]
    fn test_save_and_load_empty_state() {
        let dir = tempdir().expect("should create temp dir");
        let file_path = dir.path().join("state.txt");

        let original = PersistedState::empty();

        save_to_path(&original, &file_path).expect("save should succeed");
        let loaded = load_from_path(&file_path)
            .expect("load should succeed")
            .expect("should have state");

        assert_eq!(original, loaded);
    }

    #[test]
    fn test_save_and_load_with_special_characters() {
        let dir = tempdir().expect("should create temp dir");
        let file_path = dir.path().join("state.txt");

        let original = PersistedState::new(vec![
            "1 + 2 = 3".to_string(),
            "x = \"hello\"".to_string(),
            "unicode: \u{1f980}".to_string(),
        ]);

        save_to_path(&original, &file_path).expect("save should succeed");
        let loaded = load_from_path(&file_path)
            .expect("load should succeed")
            .expect("should have state");

        assert_eq!(original, loaded);
    }

    #[test]
    fn test_graceful_handling_missing_file_returns_none() {
        let dir = tempdir().expect("should create temp dir");
        let file_path = dir.path().join("does_not_exist.txt");

        let result = load_from_path(&file_path);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_graceful_handling_binary_garbage_returns_none() {
        let dir = tempdir().expect("should create temp dir");
        let file_path = dir.path().join("garbage.txt");

        fs::write(&file_path, [0x00, 0xFF, 0xFE, 0x89]).expect("should write file");

        let result = load_from_path(&file_path);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }
}
