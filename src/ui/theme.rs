//! Theme detection for Crabculator.

use terminal_colorsaurus::{QueryOptions, ThemeMode, theme_mode};

/// Detected terminal theme.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppTheme {
    Light,
    Dark,
}

impl AppTheme {
    /// Detects the terminal theme using terminal-colorsaurus.
    ///
    /// Falls back to Dark theme if detection fails.
    #[must_use]
    pub fn detect() -> Self {
        theme_mode(QueryOptions::default())
            .map(|mode| match mode {
                ThemeMode::Light => Self::Light,
                ThemeMode::Dark => Self::Dark,
            })
            .unwrap_or(Self::Dark)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_theme_detect_returns_valid_theme() {
        let theme = AppTheme::detect();
        // Theme detection may succeed or fail, but should always return a valid variant
        assert!(matches!(theme, AppTheme::Light | AppTheme::Dark));
    }

    #[test]
    fn app_theme_debug_implementation() {
        assert_eq!(format!("{:?}", AppTheme::Light), "Light");
        assert_eq!(format!("{:?}", AppTheme::Dark), "Dark");
    }

    #[test]
    fn app_theme_equality() {
        assert_eq!(AppTheme::Light, AppTheme::Light);
        assert_eq!(AppTheme::Dark, AppTheme::Dark);
        assert_ne!(AppTheme::Light, AppTheme::Dark);
    }

    #[test]
    fn app_theme_clone() {
        let theme = AppTheme::Dark;
        let cloned = theme;
        assert_eq!(theme, cloned);
    }
}
