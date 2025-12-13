//! State serialization and deserialization for persistence.
//!
//! Provides functionality to save and load application state (buffer lines and variables)
//! to/from disk.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{self, ErrorKind};
use std::path::Path;

use super::paths;

/// Represents the persisted application state.
///
/// Contains the buffer lines and variables that should be saved between sessions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersistedState {
    /// The lines from the buffer.
    pub buffer_lines: Vec<String>,
    /// Variable name to value mapping.
    pub variables: HashMap<String, f64>,
}

impl PersistedState {
    /// Creates a new `PersistedState` with the given buffer lines and variables.
    #[must_use]
    #[allow(clippy::missing_const_for_fn)] // HashMap is not const-constructible
    pub fn new(buffer_lines: Vec<String>, variables: HashMap<String, f64>) -> Self {
        Self {
            buffer_lines,
            variables,
        }
    }

    /// Creates an empty `PersistedState` with no buffer lines and no variables.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            buffer_lines: Vec::new(),
            variables: HashMap::new(),
        }
    }
}

impl Default for PersistedState {
    fn default() -> Self {
        Self::empty()
    }
}

/// Saves the given state to the state file.
///
/// Creates the state directory if it doesn't exist.
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

    // Create the directory if it doesn't exist
    fs::create_dir_all(&state_dir)?;

    // Serialize state to JSON
    let json = serde_json::to_string_pretty(state).map_err(|e| {
        io::Error::new(
            ErrorKind::InvalidData,
            format!("Failed to serialize state: {e}"),
        )
    })?;

    // Write to file
    fs::write(&state_file, json)
}

/// Loads the state from the state file.
///
/// # Returns
///
/// - `Ok(Some(state))` if the file exists and is valid JSON
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

/// Loads state from a specific path.
///
/// This is primarily used for testing with temporary files.
///
/// # Errors
///
/// Returns an error if the file exists but cannot be read (e.g., permission denied).
pub fn load_from_path(path: &Path) -> io::Result<Option<PersistedState>> {
    // Try to read the file
    let contents = match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(e) if e.kind() == ErrorKind::NotFound => {
            // File doesn't exist - return None (not an error)
            return Ok(None);
        }
        Err(e) if e.kind() == ErrorKind::InvalidData => {
            // File contains invalid UTF-8 - treat as corrupted
            eprintln!("Warning: State file contains invalid UTF-8, using empty state. Error: {e}");
            return Ok(None);
        }
        Err(e) => return Err(e),
    };

    // Try to parse the JSON
    match serde_json::from_str::<PersistedState>(&contents) {
        Ok(state) => Ok(Some(state)),
        Err(e) => {
            // File is corrupted - log warning and return None
            eprintln!("Warning: State file is corrupted, using empty state. Error: {e}");
            Ok(None)
        }
    }
}

/// Saves state to a specific path.
///
/// This is primarily used for testing with temporary files.
///
/// # Errors
///
/// Returns an error if:
/// - The parent directory cannot be created
/// - The file cannot be written
pub fn save_to_path(state: &PersistedState, path: &Path) -> io::Result<()> {
    // Create parent directory if it doesn't exist
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Serialize state to JSON
    let json = serde_json::to_string_pretty(state).map_err(|e| {
        io::Error::new(
            ErrorKind::InvalidData,
            format!("Failed to serialize state: {e}"),
        )
    })?;

    // Write to file
    fs::write(path, json)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    // === PersistedState struct tests ===

    #[test]
    fn test_persisted_state_new() {
        let lines = vec!["line1".to_string(), "line2".to_string()];
        let mut vars = HashMap::new();
        vars.insert("x".to_string(), 42.0);

        let state = PersistedState::new(lines.clone(), vars.clone());

        assert_eq!(state.buffer_lines, lines);
        assert_eq!(state.variables, vars);
    }

    #[test]
    fn test_persisted_state_empty() {
        let state = PersistedState::empty();

        assert!(state.buffer_lines.is_empty());
        assert!(state.variables.is_empty());
    }

    #[test]
    fn test_persisted_state_default() {
        let state = PersistedState::default();

        assert!(state.buffer_lines.is_empty());
        assert!(state.variables.is_empty());
    }

    #[test]
    fn test_persisted_state_serializes_to_json() {
        let mut vars = HashMap::new();
        vars.insert("x".to_string(), 42.0);
        vars.insert("y".to_string(), 123.456);

        let state = PersistedState::new(vec!["1 + 2".to_string(), "x = 5".to_string()], vars);

        let json = serde_json::to_string(&state).expect("serialization should succeed");

        // Verify it's valid JSON by parsing it back
        let parsed: serde_json::Value = serde_json::from_str(&json).expect("JSON should be valid");
        assert!(parsed.get("buffer_lines").is_some());
        assert!(parsed.get("variables").is_some());
    }

    #[test]
    fn test_persisted_state_deserializes_from_json() {
        let json = r#"{"buffer_lines":["1 + 2","x = 5"],"variables":{"x":42.0,"y":123.456}}"#;

        let state: PersistedState =
            serde_json::from_str(json).expect("deserialization should succeed");

        assert_eq!(state.buffer_lines, vec!["1 + 2", "x = 5"]);
        assert_eq!(state.variables.get("x"), Some(&42.0));
        assert_eq!(state.variables.get("y"), Some(&123.456));
    }

    #[test]
    fn test_persisted_state_roundtrip() {
        let mut vars = HashMap::new();
        vars.insert("answer".to_string(), 42.0);

        let original = PersistedState::new(vec!["hello".to_string()], vars);

        let json = serde_json::to_string(&original).expect("serialization should succeed");
        let restored: PersistedState =
            serde_json::from_str(&json).expect("deserialization should succeed");

        assert_eq!(original, restored);
    }

    // === Save function tests ===

    #[test]
    fn test_save_to_path_creates_file() {
        let dir = tempdir().expect("should create temp dir");
        let file_path = dir.path().join("state.json");

        let state = PersistedState::empty();
        save_to_path(&state, &file_path).expect("save should succeed");

        assert!(file_path.exists());
    }

    #[test]
    fn test_save_to_path_creates_parent_directories() {
        let dir = tempdir().expect("should create temp dir");
        let file_path = dir.path().join("nested").join("dir").join("state.json");

        let state = PersistedState::empty();
        save_to_path(&state, &file_path).expect("save should succeed");

        assert!(file_path.exists());
    }

    #[test]
    fn test_save_to_path_writes_valid_json() {
        let dir = tempdir().expect("should create temp dir");
        let file_path = dir.path().join("state.json");

        let mut vars = HashMap::new();
        vars.insert("x".to_string(), 42.0);
        let state = PersistedState::new(vec!["1 + 2".to_string()], vars);

        save_to_path(&state, &file_path).expect("save should succeed");

        let contents = fs::read_to_string(&file_path).expect("should read file");
        let parsed: serde_json::Value =
            serde_json::from_str(&contents).expect("should be valid JSON");

        assert!(parsed.get("buffer_lines").is_some());
        assert!(parsed.get("variables").is_some());
    }

    #[test]
    fn test_save_to_path_overwrites_existing_file() {
        let dir = tempdir().expect("should create temp dir");
        let file_path = dir.path().join("state.json");

        let state1 = PersistedState::new(vec!["first".to_string()], HashMap::new());
        save_to_path(&state1, &file_path).expect("first save should succeed");

        let state2 = PersistedState::new(vec!["second".to_string()], HashMap::new());
        save_to_path(&state2, &file_path).expect("second save should succeed");

        let loaded = load_from_path(&file_path)
            .expect("load should succeed")
            .expect("should have state");
        assert_eq!(loaded.buffer_lines, vec!["second"]);
    }

    // === Load function tests ===

    #[test]
    fn test_load_from_path_returns_none_when_file_missing() {
        let dir = tempdir().expect("should create temp dir");
        let file_path = dir.path().join("nonexistent.json");

        let result = load_from_path(&file_path).expect("load should not error");

        assert!(result.is_none());
    }

    #[test]
    fn test_load_from_path_returns_state_when_valid_json() {
        let dir = tempdir().expect("should create temp dir");
        let file_path = dir.path().join("state.json");

        let json = r#"{"buffer_lines":["hello"],"variables":{"x":42.0}}"#;
        fs::write(&file_path, json).expect("should write file");

        let result = load_from_path(&file_path)
            .expect("load should succeed")
            .expect("should have state");

        assert_eq!(result.buffer_lines, vec!["hello"]);
        assert_eq!(result.variables.get("x"), Some(&42.0));
    }

    #[test]
    fn test_load_from_path_returns_none_when_corrupted() {
        let dir = tempdir().expect("should create temp dir");
        let file_path = dir.path().join("state.json");

        // Write invalid JSON
        fs::write(&file_path, "not valid json {{{").expect("should write file");

        let result = load_from_path(&file_path).expect("load should not error");

        // Should return None (not error) for corrupted file
        assert!(result.is_none());
    }

    #[test]
    fn test_load_from_path_returns_none_when_partial_json() {
        let dir = tempdir().expect("should create temp dir");
        let file_path = dir.path().join("state.json");

        // Write partial JSON (valid JSON but missing required fields)
        fs::write(&file_path, r#"{"buffer_lines":["hello"]}"#).expect("should write file");

        let result = load_from_path(&file_path).expect("load should not error");

        // Should return None because variables field is missing
        assert!(result.is_none());
    }

    #[test]
    fn test_load_from_path_returns_none_when_empty_file() {
        let dir = tempdir().expect("should create temp dir");
        let file_path = dir.path().join("state.json");

        // Write empty file
        fs::write(&file_path, "").expect("should write file");

        let result = load_from_path(&file_path).expect("load should not error");

        // Should return None for empty file
        assert!(result.is_none());
    }

    // === Save and Load roundtrip tests ===

    #[test]
    fn test_save_and_load_roundtrip() {
        let dir = tempdir().expect("should create temp dir");
        let file_path = dir.path().join("state.json");

        let mut vars = HashMap::new();
        vars.insert("x".to_string(), 42.0);
        vars.insert("y".to_string(), 123.456);

        let original = PersistedState::new(vec!["line1".to_string(), "line2".to_string()], vars);

        save_to_path(&original, &file_path).expect("save should succeed");
        let loaded = load_from_path(&file_path)
            .expect("load should succeed")
            .expect("should have state");

        assert_eq!(original, loaded);
    }

    #[test]
    fn test_save_and_load_empty_state() {
        let dir = tempdir().expect("should create temp dir");
        let file_path = dir.path().join("state.json");

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
        let file_path = dir.path().join("state.json");

        let original = PersistedState::new(
            vec![
                "1 + 2 = 3".to_string(),
                "x = \"hello\"".to_string(),
                "unicode: ".to_string(),
            ],
            HashMap::new(),
        );

        save_to_path(&original, &file_path).expect("save should succeed");
        let loaded = load_from_path(&file_path)
            .expect("load should succeed")
            .expect("should have state");

        assert_eq!(original, loaded);
    }

    // === Graceful handling tests ===

    #[test]
    fn test_graceful_handling_missing_file_returns_none() {
        let dir = tempdir().expect("should create temp dir");
        let file_path = dir.path().join("does_not_exist.json");

        // Should return Ok(None), not an error
        let result = load_from_path(&file_path);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_graceful_handling_corrupted_json_returns_none() {
        let dir = tempdir().expect("should create temp dir");
        let file_path = dir.path().join("corrupted.json");

        // Write corrupted JSON
        fs::write(&file_path, "{ broken json").expect("should write file");

        // Should return Ok(None) and log warning, not crash
        let result = load_from_path(&file_path);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_graceful_handling_wrong_json_structure_returns_none() {
        let dir = tempdir().expect("should create temp dir");
        let file_path = dir.path().join("wrong_structure.json");

        // Write valid JSON but wrong structure
        fs::write(&file_path, r#"{"some_other_field": 123}"#).expect("should write file");

        // Should return Ok(None) because it can't deserialize to PersistedState
        let result = load_from_path(&file_path);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_graceful_handling_binary_garbage_returns_none() {
        let dir = tempdir().expect("should create temp dir");
        let file_path = dir.path().join("garbage.json");

        // Write binary garbage
        fs::write(&file_path, [0x00, 0xFF, 0xFE, 0x89]).expect("should write file");

        // Should return Ok(None), not crash
        let result = load_from_path(&file_path);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }
}
