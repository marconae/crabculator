//! Terminal management for the TUI application.
//!
//! This module provides functions to set up and restore the terminal state,
//! including handling panics to ensure the terminal is always restored.

use std::io::{self, Stdout};

use crossterm::{
    cursor::{Hide, Show},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

/// Sets up the terminal for TUI rendering.
///
/// This function:
/// - Enables raw mode for direct keyboard input
/// - Enters the alternate screen buffer
/// - Hides the cursor
///
/// # Errors
///
/// Returns an error if any terminal setup operation fails.
pub fn setup_terminal() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?;
    Terminal::new(CrosstermBackend::new(stdout))
}

/// Restores the terminal to its original state.
///
/// This function:
/// - Disables raw mode
/// - Leaves the alternate screen buffer
/// - Shows the cursor
///
/// # Errors
///
/// Returns an error if any terminal restoration operation fails.
pub fn restore_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen, Show)?;
    Ok(())
}

/// Installs a panic hook that restores the terminal before panicking.
///
/// This ensures that even if the application panics, the terminal is left
/// in a usable state. Without this, a panic would leave the terminal in
/// raw mode with the cursor hidden.
pub fn install_panic_hook() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        // Attempt to restore terminal; ignore errors since we're already panicking
        let _ = restore_terminal();
        original_hook(panic_info);
    }));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_install_panic_hook_does_not_panic() {
        // This test verifies that installing the panic hook doesn't cause issues.
        // We can't easily test the actual panic behavior without triggering a panic,
        // but we can verify the hook installation succeeds.
        install_panic_hook();
    }
}
