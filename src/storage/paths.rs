//! Path utilities for state storage.
//!
//! Provides functions to determine where state files should be stored.
//! State is stored in `~/.crabculator/` directory across all platforms.

use std::path::PathBuf;

/// Returns the directory where state files are stored.
///
/// Returns `~/.crabculator/` on all platforms:
/// - Unix (Linux/macOS): Uses `$HOME/.crabculator/`
/// - Windows: Uses `%USERPROFILE%\.crabculator\`
///
/// # Returns
///
/// `Some(PathBuf)` containing the state directory path, or `None` if the
/// home directory cannot be determined.
#[must_use]
pub fn state_dir() -> Option<PathBuf> {
    dirs::home_dir().map(|home| home.join(".crabculator"))
}

/// Returns the path to the state file.
///
/// Returns `~/.crabculator/state.json` on all platforms.
///
/// # Returns
///
/// `Some(PathBuf)` containing the state file path, or `None` if the
/// home directory cannot be determined.
#[must_use]
pub fn state_file() -> Option<PathBuf> {
    state_dir().map(|dir| dir.join("state.json"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_dir_returns_some() {
        let dir = state_dir();
        assert!(
            dir.is_some(),
            "state_dir() should return Some on systems with a home directory"
        );
    }

    #[test]
    fn state_dir_ends_with_dot_crabculator() {
        let dir = state_dir().expect("state_dir should return Some");
        assert!(
            dir.ends_with(".crabculator"),
            "state_dir should end with .crabculator, got: {dir:?}",
        );
    }

    #[test]
    fn state_dir_is_under_home_directory() {
        let dir = state_dir().expect("state_dir should return Some");
        let home = dirs::home_dir().expect("home_dir should be available");

        assert!(
            dir.starts_with(&home),
            "state_dir {dir:?} should be under home {home:?}",
        );
    }

    #[test]
    fn state_dir_is_absolute_path() {
        let dir = state_dir().expect("state_dir should return Some");
        assert!(
            dir.is_absolute(),
            "state_dir should return absolute path, got: {dir:?}",
        );
    }

    #[test]
    fn state_dir_parent_is_home() {
        let dir = state_dir().expect("state_dir should return Some");
        let home = dirs::home_dir().expect("home_dir should be available");

        let parent = dir.parent().expect("state_dir should have a parent");
        assert_eq!(
            parent, home,
            "state_dir parent {parent:?} should equal home {home:?}",
        );
    }

    #[test]
    fn state_file_returns_some() {
        let file = state_file();
        assert!(
            file.is_some(),
            "state_file() should return Some on systems with a home directory"
        );
    }

    #[test]
    fn state_file_ends_with_state_json() {
        let file = state_file().expect("state_file should return Some");
        assert!(
            file.ends_with("state.json"),
            "state_file should end with state.json, got: {file:?}",
        );
    }

    #[test]
    fn state_file_is_inside_state_dir() {
        let dir = state_dir().expect("state_dir should return Some");
        let file = state_file().expect("state_file should return Some");

        assert!(
            file.starts_with(&dir),
            "state_file {file:?} should be inside state_dir {dir:?}",
        );
    }

    #[test]
    fn state_file_parent_is_state_dir() {
        let dir = state_dir().expect("state_dir should return Some");
        let file = state_file().expect("state_file should return Some");

        let parent = file.parent().expect("state_file should have a parent");
        assert_eq!(
            parent, dir,
            "state_file parent {parent:?} should equal state_dir {dir:?}",
        );
    }

    #[test]
    fn state_file_is_absolute_path() {
        let file = state_file().expect("state_file should return Some");
        assert!(
            file.is_absolute(),
            "state_file should return absolute path, got: {file:?}",
        );
    }

    #[test]
    fn state_dir_matches_expected_format() {
        let dir = state_dir().expect("state_dir should return Some");
        let home = dirs::home_dir().expect("home_dir should be available");

        // Verify the exact path is home/.crabculator
        let expected = home.join(".crabculator");
        assert_eq!(dir, expected, "state_dir {dir:?} should equal {expected:?}",);
    }

    #[test]
    fn state_file_matches_expected_format() {
        let file = state_file().expect("state_file should return Some");
        let home = dirs::home_dir().expect("home_dir should be available");

        // Verify the exact path is home/.crabculator/state.json
        let expected = home.join(".crabculator").join("state.json");
        assert_eq!(
            file, expected,
            "state_file {file:?} should equal {expected:?}",
        );
    }
}
