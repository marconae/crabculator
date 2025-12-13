use crate::editor::Buffer;
use crate::eval::EvalContext;
use crate::storage;

/// Application state for Crabculator.
pub struct App {
    /// Whether the application is still running.
    pub running: bool,
    /// The text buffer for expression input.
    pub buffer: Buffer,
    /// The evaluation context for variables.
    pub context: EvalContext,
    /// Scroll offset (first visible line index, 0-based).
    /// Used for vertical scrolling when content exceeds visible height.
    pub scroll_offset: usize,
}

impl App {
    /// Creates a new application instance with running state set to true.
    ///
    /// Attempts to load persisted state from disk. If state exists, the buffer
    /// and context are restored. Otherwise, defaults are used.
    #[must_use]
    pub fn new() -> Self {
        // Try to load persisted state
        let (buffer, context) = match storage::load() {
            Ok(Some(state)) => {
                let buffer = Buffer::from_lines(state.buffer_lines);
                let mut context = EvalContext::new();
                context.load_variables(&state.variables);
                (buffer, context)
            }
            Ok(None) | Err(_) => {
                // No state file or error loading - use defaults
                (Buffer::new(), EvalContext::new())
            }
        };

        Self {
            running: true,
            buffer,
            context,
            scroll_offset: 0,
        }
    }

    /// Signals the application to quit by setting running to false.
    pub const fn quit(&mut self) {
        self.running = false;
    }

    /// Saves the current state to disk.
    ///
    /// Persists the buffer lines and variables to the state file.
    /// Errors are silently ignored (state persistence is best-effort).
    pub fn save_state(&self) {
        let state = storage::PersistedState::new(
            self.buffer.lines().iter().map(String::clone).collect(),
            self.context.extract_variables(),
        );
        // Ignore errors - state persistence is best-effort
        let _ = storage::save(&state);
    }

    /// Clears all content from the editor.
    ///
    /// This method:
    /// - Clears all lines from the buffer
    /// - Resets the cursor to position (0, 0)
    /// - Clears all variables from the evaluation context
    /// - Resets the scroll offset to 0
    pub fn clear_all(&mut self) {
        self.buffer.clear();
        self.context.clear();
        self.scroll_offset = 0;
    }

    /// Adjusts scroll offset to keep cursor within visible area.
    ///
    /// Called after cursor movement to ensure the cursor row is visible.
    /// If cursor is above visible area, scrolls up. If cursor is below
    /// visible area, scrolls down to make it visible.
    ///
    /// # Arguments
    /// * `visible_height` - The number of visible lines in the viewport
    #[allow(clippy::missing_const_for_fn)] // cursor().row() is not const
    pub fn adjust_scroll(&mut self, visible_height: usize) {
        if visible_height == 0 {
            return;
        }

        let cursor_row = self.buffer.cursor().row();

        // If cursor is above visible area, scroll up to show it
        if cursor_row < self.scroll_offset {
            self.scroll_offset = cursor_row;
        }

        // If cursor is below visible area, scroll down to show it
        // Last visible line is scroll_offset + visible_height - 1
        if cursor_row >= self.scroll_offset + visible_height {
            self.scroll_offset = cursor_row - visible_height + 1;
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use evalexpr::Value;

    #[test]
    fn test_app_new_initializes_running() {
        let app = App::new();
        assert!(app.running);
    }

    #[test]
    fn test_app_new_initializes_buffer() {
        let app = App::new();
        // Buffer may have content from persisted state or be empty
        assert!(app.buffer.line_count() >= 1);
    }

    #[test]
    fn test_app_new_initializes_context() {
        let app = App::new();
        // Context should exist (may have variables from persisted state)
        let _ = app.context.extract_variables();
    }

    #[test]
    fn test_app_quit_sets_running_false() {
        let mut app = App::new();
        app.quit();
        assert!(!app.running);
    }

    #[test]
    fn test_app_new_initializes_scroll_offset_to_zero() {
        let app = App::new();
        assert_eq!(app.scroll_offset, 0);
    }

    #[test]
    fn test_app_save_state_extracts_buffer_and_context() {
        // Create an app and modify its state
        let mut app = App::new();
        app.buffer.insert_char('a');
        app.buffer.insert_char('b');
        app.context.set_variable("x", Value::Int(42));

        // save_state should not panic
        app.save_state();
    }

    // ============================================================
    // Clear all tests
    // ============================================================

    #[test]
    fn test_clear_all_clears_buffer() {
        let mut app = App::new();
        app.buffer.insert_char('x');
        app.buffer.insert_newline();
        app.buffer.insert_char('y');

        app.clear_all();

        assert_eq!(app.buffer.line_count(), 1);
        assert_eq!(app.buffer.lines()[0], "");
    }

    #[test]
    fn test_clear_all_resets_cursor() {
        let mut app = App::new();
        app.buffer.insert_char('a');
        app.buffer.insert_newline();
        app.buffer.insert_char('b');

        app.clear_all();

        assert_eq!(app.buffer.cursor().row(), 0);
        assert_eq!(app.buffer.cursor().col(), 0);
    }

    #[test]
    fn test_clear_all_clears_context() {
        let mut app = App::new();
        app.context.set_variable("x", Value::Int(42));
        app.context.set_variable("y", Value::Float(3.15));

        app.clear_all();

        assert!(app.context.get_variable("x").is_none());
        assert!(app.context.get_variable("y").is_none());
    }

    #[test]
    fn test_clear_all_resets_scroll_offset() {
        let mut app = App::new();
        app.scroll_offset = 10;

        app.clear_all();

        assert_eq!(app.scroll_offset, 0);
    }

    // ============================================================
    // Scroll offset adjustment tests
    // ============================================================

    #[test]
    fn test_adjust_scroll_cursor_above_visible_area() {
        let mut app = App::new();
        // Create buffer with 10 lines
        for i in 0..10 {
            for c in format!("line {i}").chars() {
                app.buffer.insert_char(c);
            }
            app.buffer.insert_newline();
        }
        // Move cursor to line 0 (should be at end due to newlines)
        for _ in 0..10 {
            app.buffer.move_cursor_up();
        }
        // Set scroll offset to 5 (viewing lines 5-9)
        app.scroll_offset = 5;
        // Cursor is at line 0, which is above visible area
        let visible_height = 5;

        app.adjust_scroll(visible_height);

        // Scroll should adjust to show cursor (line 0)
        assert_eq!(app.scroll_offset, 0);
    }

    #[test]
    fn test_adjust_scroll_cursor_below_visible_area() {
        let mut app = App::new();
        // Create buffer with 10 lines
        for i in 0..10 {
            for c in format!("line {i}").chars() {
                app.buffer.insert_char(c);
            }
            if i < 9 {
                app.buffer.insert_newline();
            }
        }
        // Cursor should be at line 9
        assert_eq!(app.buffer.cursor().row(), 9);
        // Set scroll offset to 0 (viewing lines 0-4)
        app.scroll_offset = 0;
        let visible_height = 5;

        app.adjust_scroll(visible_height);

        // Scroll should adjust so cursor (line 9) is visible
        // visible area: scroll_offset to scroll_offset + visible_height - 1
        // line 9 should be visible: scroll_offset + 4 >= 9, so scroll_offset >= 5
        assert_eq!(app.scroll_offset, 5);
    }

    #[test]
    fn test_adjust_scroll_cursor_within_visible_area_no_change() {
        let mut app = App::new();
        // Create buffer with 5 lines
        for i in 0..5 {
            for c in format!("line {i}").chars() {
                app.buffer.insert_char(c);
            }
            if i < 4 {
                app.buffer.insert_newline();
            }
        }
        // Move cursor to line 2
        app.buffer.move_cursor_up();
        app.buffer.move_cursor_up();
        assert_eq!(app.buffer.cursor().row(), 2);
        // Set scroll offset to 0 (viewing lines 0-4)
        app.scroll_offset = 0;
        let visible_height = 5;

        app.adjust_scroll(visible_height);

        // No change needed, cursor is within visible area
        assert_eq!(app.scroll_offset, 0);
    }

    #[test]
    fn test_adjust_scroll_with_zero_visible_height() {
        let mut app = App::new();
        app.buffer.insert_char('a');
        app.scroll_offset = 0;

        // visible_height of 0 should not crash
        app.adjust_scroll(0);

        // Scroll offset should remain unchanged
        assert_eq!(app.scroll_offset, 0);
    }

    #[test]
    fn test_adjust_scroll_single_line_buffer() {
        let mut app = App::new();
        app.buffer.insert_char('a');
        app.scroll_offset = 0;
        let visible_height = 10;

        app.adjust_scroll(visible_height);

        // Single line, scroll should stay at 0
        assert_eq!(app.scroll_offset, 0);
    }
}
