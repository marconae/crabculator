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
    /// Horizontal scroll offset (first visible column index, 0-based).
    /// Used for horizontal scrolling when content exceeds visible width.
    pub horizontal_scroll_offset: usize,
    /// Whether the help overlay is currently visible.
    pub help_visible: bool,
    /// Scroll offset for the help overlay content (0-based).
    pub help_scroll_offset: usize,
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
            horizontal_scroll_offset: 0,
            help_visible: false,
            help_scroll_offset: 0,
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
    /// - Resets the horizontal scroll offset to 0
    /// - Closes the help overlay and resets its scroll offset
    pub fn clear_all(&mut self) {
        self.buffer.clear();
        self.context.clear();
        self.scroll_offset = 0;
        self.horizontal_scroll_offset = 0;
        self.help_visible = false;
        self.help_scroll_offset = 0;
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

    /// Adjusts horizontal scroll offset to keep cursor within visible area.
    ///
    /// Called after cursor movement to ensure the cursor column is visible.
    /// If cursor is before visible area, scrolls left. If cursor is after
    /// visible area, scrolls right to make it visible.
    ///
    /// A margin is used to provide smoother scrolling experience by triggering
    /// scroll before the cursor reaches the absolute edge.
    ///
    /// # Arguments
    /// * `visible_width` - The number of visible columns in the viewport
    #[allow(clippy::missing_const_for_fn)] // cursor().col() is not const
    pub fn adjust_horizontal_scroll(&mut self, visible_width: usize) {
        if visible_width == 0 {
            return;
        }

        let cursor_col = self.buffer.cursor().col();
        // Use a margin for smoother scrolling (5 chars or less if width is small)
        let margin = visible_width.min(5).saturating_sub(1);

        // If cursor is before visible area (with margin), scroll left to show it
        if cursor_col < self.horizontal_scroll_offset + margin {
            self.horizontal_scroll_offset = cursor_col.saturating_sub(margin);
        }

        // If cursor is after visible area (with margin), scroll right to show it
        // Last visible column is horizontal_scroll_offset + visible_width - 1
        if cursor_col >= self.horizontal_scroll_offset + visible_width - margin {
            self.horizontal_scroll_offset = cursor_col.saturating_sub(visible_width - margin - 1);
        }
    }

    /// Toggles the help overlay visibility.
    ///
    /// When opening the help overlay, resets the scroll offset to 0.
    #[allow(clippy::missing_const_for_fn)] // Uses conditional logic
    pub fn toggle_help(&mut self) {
        self.help_visible = !self.help_visible;
        if self.help_visible {
            self.help_scroll_offset = 0;
        }
    }

    /// Closes the help overlay.
    pub const fn close_help(&mut self) {
        self.help_visible = false;
    }

    /// Scrolls the help overlay content down by one line.
    ///
    /// # Arguments
    /// * `content_height` - The total height of the help content in lines
    #[allow(clippy::missing_const_for_fn)] // May need more logic in future
    pub fn scroll_help_down(&mut self, content_height: usize) {
        if self.help_scroll_offset < content_height.saturating_sub(1) {
            self.help_scroll_offset += 1;
        }
    }

    /// Scrolls the help overlay content up by one line.
    pub const fn scroll_help_up(&mut self) {
        self.help_scroll_offset = self.help_scroll_offset.saturating_sub(1);
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

    // ============================================================
    // Help overlay state tests
    // ============================================================

    #[test]
    fn test_app_new_initializes_help_visible_to_false() {
        let app = App::new();
        assert!(!app.help_visible);
    }

    #[test]
    fn test_app_new_initializes_help_scroll_offset_to_zero() {
        let app = App::new();
        assert_eq!(app.help_scroll_offset, 0);
    }

    #[test]
    fn test_toggle_help_shows_overlay_when_hidden() {
        let mut app = App::new();
        assert!(!app.help_visible);

        app.toggle_help();

        assert!(app.help_visible);
    }

    #[test]
    fn test_toggle_help_hides_overlay_when_visible() {
        let mut app = App::new();
        app.help_visible = true;

        app.toggle_help();

        assert!(!app.help_visible);
    }

    #[test]
    fn test_toggle_help_resets_scroll_offset_when_opening() {
        let mut app = App::new();
        app.help_scroll_offset = 5;

        app.toggle_help(); // Open help

        assert_eq!(app.help_scroll_offset, 0);
    }

    #[test]
    fn test_close_help_hides_overlay() {
        let mut app = App::new();
        app.help_visible = true;

        app.close_help();

        assert!(!app.help_visible);
    }

    #[test]
    fn test_close_help_when_already_hidden_stays_hidden() {
        let mut app = App::new();
        assert!(!app.help_visible);

        app.close_help();

        assert!(!app.help_visible);
    }

    #[test]
    fn test_scroll_help_down_increases_offset() {
        let mut app = App::new();
        app.help_visible = true;
        app.help_scroll_offset = 0;

        app.scroll_help_down(10); // content_height

        assert_eq!(app.help_scroll_offset, 1);
    }

    #[test]
    fn test_scroll_help_up_decreases_offset() {
        let mut app = App::new();
        app.help_visible = true;
        app.help_scroll_offset = 5;

        app.scroll_help_up();

        assert_eq!(app.help_scroll_offset, 4);
    }

    #[test]
    fn test_scroll_help_up_does_not_go_below_zero() {
        let mut app = App::new();
        app.help_visible = true;
        app.help_scroll_offset = 0;

        app.scroll_help_up();

        assert_eq!(app.help_scroll_offset, 0);
    }

    #[test]
    fn test_clear_all_resets_help_state() {
        let mut app = App::new();
        app.help_visible = true;
        app.help_scroll_offset = 5;

        app.clear_all();

        assert!(!app.help_visible);
        assert_eq!(app.help_scroll_offset, 0);
    }

    // ============================================================
    // Horizontal scroll offset adjustment tests
    // ============================================================

    #[test]
    fn test_adjust_horizontal_scroll_cursor_before_visible_area_scrolls_left() {
        let mut app = App::new();
        // Create a line with some content
        for c in "0123456789abcdef".chars() {
            app.buffer.insert_char(c);
        }
        // Move cursor to start
        app.buffer.move_cursor_to_line_start();
        // Set horizontal scroll offset as if we scrolled right
        app.horizontal_scroll_offset = 10;
        // Cursor is at column 0, which is before visible area
        let visible_width = 10;

        app.adjust_horizontal_scroll(visible_width);

        // Scroll should adjust to show cursor (column 0)
        assert_eq!(app.horizontal_scroll_offset, 0);
    }

    #[test]
    fn test_adjust_horizontal_scroll_cursor_after_visible_area_scrolls_right() {
        let mut app = App::new();
        // Create a line with content extending beyond visible area
        for c in "0123456789abcdefghij".chars() {
            app.buffer.insert_char(c);
        }
        // Cursor should be at column 20 (end of line)
        assert_eq!(app.buffer.cursor().col(), 20);
        // Start with scroll at 0
        app.horizontal_scroll_offset = 0;
        let visible_width = 10;

        app.adjust_horizontal_scroll(visible_width);

        // Scroll should adjust so cursor is visible
        // With margin of 4 (min(10, 5) - 1), cursor at 20 needs offset >= 20 - (10 - 4 - 1) = 15
        assert!(app.horizontal_scroll_offset >= 15);
    }

    #[test]
    fn test_adjust_horizontal_scroll_cursor_within_visible_area_no_change() {
        let mut app = App::new();
        // Create a line with some content
        for c in "0123456789".chars() {
            app.buffer.insert_char(c);
        }
        // Move cursor to middle (column 5)
        app.buffer.move_cursor_to_line_start();
        for _ in 0..5 {
            app.buffer.move_cursor_right();
        }
        assert_eq!(app.buffer.cursor().col(), 5);
        // Set scroll offset to 0
        app.horizontal_scroll_offset = 0;
        let visible_width = 20;

        app.adjust_horizontal_scroll(visible_width);

        // No change needed, cursor is within visible area
        assert_eq!(app.horizontal_scroll_offset, 0);
    }

    #[test]
    fn test_adjust_horizontal_scroll_with_zero_visible_width() {
        let mut app = App::new();
        app.buffer.insert_char('a');
        app.horizontal_scroll_offset = 5;

        // visible_width of 0 should not crash and not change offset
        app.adjust_horizontal_scroll(0);

        // Scroll offset should remain unchanged
        assert_eq!(app.horizontal_scroll_offset, 5);
    }

    #[test]
    fn test_clear_all_resets_horizontal_scroll_offset() {
        let mut app = App::new();
        app.horizontal_scroll_offset = 10;

        app.clear_all();

        assert_eq!(app.horizontal_scroll_offset, 0);
    }

    #[test]
    fn test_app_new_initializes_horizontal_scroll_offset_to_zero() {
        let app = App::new();
        assert_eq!(app.horizontal_scroll_offset, 0);
    }
}
