//! Text buffer management for the expression editor.
//!
//! Provides a multi-line text buffer with editing operations.

use super::Cursor;

/// A multi-line text buffer for editing expressions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Buffer {
    /// The lines of text in the buffer.
    lines: Vec<String>,
    /// The cursor position within the buffer.
    cursor: Cursor,
}

impl Buffer {
    /// Creates a new buffer with a single empty line.
    #[must_use]
    pub fn new() -> Self {
        Self {
            lines: vec![String::new()],
            cursor: Cursor::default(),
        }
    }

    /// Creates a buffer from existing lines.
    ///
    /// If the provided vector is empty, creates a buffer with a single empty line.
    /// The cursor is positioned at the origin (0, 0).
    #[must_use]
    pub fn from_lines(lines: Vec<String>) -> Self {
        let lines = if lines.is_empty() {
            vec![String::new()]
        } else {
            lines
        };
        Self {
            lines,
            cursor: Cursor::default(),
        }
    }

    /// Returns a reference to the lines in the buffer.
    #[must_use]
    pub fn lines(&self) -> &[String] {
        &self.lines
    }

    /// Returns the number of lines in the buffer.
    #[must_use]
    #[allow(clippy::missing_const_for_fn)] // Vec::len() is not const
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    /// Returns the cursor position.
    #[must_use]
    pub const fn cursor(&self) -> &Cursor {
        &self.cursor
    }

    /// Returns the current line the cursor is on.
    #[must_use]
    pub fn current_line(&self) -> &str {
        &self.lines[self.cursor.row()]
    }

    /// Returns the length of the current line.
    #[must_use]
    pub fn current_line_len(&self) -> usize {
        self.current_line().len()
    }

    /// Inserts a character at the current cursor position.
    pub fn insert_char(&mut self, ch: char) {
        let row = self.cursor.row();
        let col = self.cursor.col();
        self.lines[row].insert(col, ch);
        self.cursor.set_col(col + 1);
    }

    /// Creates a new line at the cursor position (Enter key behavior).
    ///
    /// Text after the cursor is moved to the new line.
    pub fn insert_newline(&mut self) {
        let row = self.cursor.row();
        let col = self.cursor.col();

        // Split the current line at the cursor position
        let remaining = self.lines[row].split_off(col);

        // Insert the remaining text as a new line
        self.lines.insert(row + 1, remaining);

        // Move cursor to start of new line
        self.cursor.set_row(row + 1);
        self.cursor.set_col(0);
    }

    /// Deletes the character before the cursor (Backspace key behavior).
    ///
    /// If at the start of a line (not the first line), merges with the previous line.
    /// Returns `true` if a deletion occurred, `false` if at the beginning of the buffer.
    pub fn delete_char_before(&mut self) -> bool {
        let row = self.cursor.row();
        let col = self.cursor.col();

        if col > 0 {
            // Delete character before cursor within the line
            self.lines[row].remove(col - 1);
            self.cursor.set_col(col - 1);
            true
        } else if row > 0 {
            // Merge current line with previous line
            let current_line = self.lines.remove(row);
            let prev_line_len = self.lines[row - 1].len();
            self.lines[row - 1].push_str(&current_line);
            self.cursor.set_row(row - 1);
            self.cursor.set_col(prev_line_len);
            true
        } else {
            false
        }
    }

    /// Deletes the character at the cursor position (Delete key behavior).
    ///
    /// If at the end of a line (not the last line), merges with the next line.
    /// Returns `true` if a deletion occurred, `false` if at the end of the buffer.
    pub fn delete_char_at(&mut self) -> bool {
        let row = self.cursor.row();
        let col = self.cursor.col();
        let line_len = self.lines[row].len();

        if col < line_len {
            // Delete character at cursor position
            self.lines[row].remove(col);
            true
        } else if row + 1 < self.lines.len() {
            // Merge next line with current line
            let next_line = self.lines.remove(row + 1);
            self.lines[row].push_str(&next_line);
            true
        } else {
            false
        }
    }

    /// Moves the cursor left.
    ///
    /// At line start, moves to end of previous line.
    pub fn move_cursor_left(&mut self) {
        if !self.cursor.move_left() && self.cursor.row() > 0 {
            let prev_line_len = self.lines[self.cursor.row() - 1].len();
            self.cursor.move_to_prev_line_end(prev_line_len);
        }
    }

    /// Moves the cursor right.
    ///
    /// At line end, moves to start of next line.
    pub fn move_cursor_right(&mut self) {
        let line_len = self.current_line_len();
        if !self.cursor.move_right(line_len) && self.cursor.row() + 1 < self.lines.len() {
            self.cursor.move_to_next_line_start(self.lines.len());
        }
    }

    /// Moves the cursor up.
    ///
    /// Column is clamped to the length of the target line.
    pub fn move_cursor_up(&mut self) {
        if self.cursor.row() > 0 {
            let prev_line_len = self.lines[self.cursor.row() - 1].len();
            self.cursor.move_up(prev_line_len);
        }
    }

    /// Moves the cursor down.
    ///
    /// Column is clamped to the length of the target line.
    pub fn move_cursor_down(&mut self) {
        if self.cursor.row() + 1 < self.lines.len() {
            let next_line_len = self.lines[self.cursor.row() + 1].len();
            self.cursor.move_down(self.lines.len(), next_line_len);
        }
    }

    /// Moves the cursor to the start of the current line.
    pub const fn move_cursor_to_line_start(&mut self) {
        self.cursor.move_to_line_start();
    }

    /// Moves the cursor to the end of the current line.
    pub fn move_cursor_to_line_end(&mut self) {
        let line_len = self.current_line_len();
        self.cursor.move_to_line_end(line_len);
    }

    /// Returns the entire buffer content as a single string with newlines.
    #[must_use]
    pub fn content(&self) -> String {
        self.lines.join("\n")
    }

    /// Clears all content from the buffer.
    ///
    /// Resets the buffer to a single empty line and positions
    /// the cursor at the origin (row 0, column 0).
    pub fn clear(&mut self) {
        self.lines.clear();
        self.lines.push(String::new());
        self.cursor.set_row(0);
        self.cursor.set_col(0);
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // === Initial State Tests ===

    #[test]
    fn test_buffer_new_has_one_empty_line() {
        let buffer = Buffer::new();
        assert_eq!(buffer.line_count(), 1);
        assert_eq!(buffer.lines()[0], "");
    }

    #[test]
    fn test_buffer_new_cursor_at_origin() {
        let buffer = Buffer::new();
        assert_eq!(buffer.cursor().row(), 0);
        assert_eq!(buffer.cursor().col(), 0);
    }

    #[test]
    fn test_buffer_default_same_as_new() {
        let buffer1 = Buffer::new();
        let buffer2 = Buffer::default();
        assert_eq!(buffer1, buffer2);
    }

    // === Character Insertion Tests ===

    #[test]
    fn test_insert_char_at_start() {
        let mut buffer = Buffer::new();
        buffer.insert_char('a');
        assert_eq!(buffer.lines()[0], "a");
        assert_eq!(buffer.cursor().col(), 1);
    }

    #[test]
    fn test_insert_multiple_chars() {
        let mut buffer = Buffer::new();
        buffer.insert_char('h');
        buffer.insert_char('i');
        assert_eq!(buffer.lines()[0], "hi");
        assert_eq!(buffer.cursor().col(), 2);
    }

    #[test]
    fn test_insert_char_in_middle() {
        let mut buffer = Buffer::new();
        buffer.insert_char('a');
        buffer.insert_char('c');
        buffer.move_cursor_left();
        buffer.insert_char('b');
        assert_eq!(buffer.lines()[0], "abc");
        assert_eq!(buffer.cursor().col(), 2);
    }

    // === New Line Creation Tests ===

    #[test]
    fn test_insert_newline_at_end() {
        let mut buffer = Buffer::new();
        buffer.insert_char('a');
        buffer.insert_newline();
        assert_eq!(buffer.line_count(), 2);
        assert_eq!(buffer.lines()[0], "a");
        assert_eq!(buffer.lines()[1], "");
        assert_eq!(buffer.cursor().row(), 1);
        assert_eq!(buffer.cursor().col(), 0);
    }

    #[test]
    fn test_insert_newline_in_middle_of_line() {
        let mut buffer = Buffer::new();
        buffer.insert_char('a');
        buffer.insert_char('b');
        buffer.insert_char('c');
        buffer.move_cursor_left();
        buffer.move_cursor_left();
        buffer.insert_newline();
        assert_eq!(buffer.line_count(), 2);
        assert_eq!(buffer.lines()[0], "a");
        assert_eq!(buffer.lines()[1], "bc");
        assert_eq!(buffer.cursor().row(), 1);
        assert_eq!(buffer.cursor().col(), 0);
    }

    #[test]
    fn test_insert_newline_at_start_of_line() {
        let mut buffer = Buffer::new();
        buffer.insert_char('a');
        buffer.insert_char('b');
        buffer.move_cursor_to_line_start();
        buffer.insert_newline();
        assert_eq!(buffer.line_count(), 2);
        assert_eq!(buffer.lines()[0], "");
        assert_eq!(buffer.lines()[1], "ab");
    }

    // === Backspace Deletion Tests ===

    #[test]
    fn test_delete_char_before_in_middle() {
        let mut buffer = Buffer::new();
        buffer.insert_char('a');
        buffer.insert_char('b');
        buffer.insert_char('c');
        assert!(buffer.delete_char_before());
        assert_eq!(buffer.lines()[0], "ab");
        assert_eq!(buffer.cursor().col(), 2);
    }

    #[test]
    fn test_delete_char_before_at_line_start_merges_lines() {
        let mut buffer = Buffer::new();
        buffer.insert_char('a');
        buffer.insert_newline();
        buffer.insert_char('b');
        buffer.move_cursor_to_line_start();
        assert!(buffer.delete_char_before());
        assert_eq!(buffer.line_count(), 1);
        assert_eq!(buffer.lines()[0], "ab");
        assert_eq!(buffer.cursor().row(), 0);
        assert_eq!(buffer.cursor().col(), 1);
    }

    #[test]
    fn test_delete_char_before_at_buffer_start_returns_false() {
        let mut buffer = Buffer::new();
        buffer.insert_char('a');
        buffer.move_cursor_to_line_start();
        assert!(!buffer.delete_char_before());
        assert_eq!(buffer.lines()[0], "a");
    }

    // === Delete Key Tests ===

    #[test]
    fn test_delete_char_at_cursor() {
        let mut buffer = Buffer::new();
        buffer.insert_char('a');
        buffer.insert_char('b');
        buffer.move_cursor_to_line_start();
        assert!(buffer.delete_char_at());
        assert_eq!(buffer.lines()[0], "b");
        assert_eq!(buffer.cursor().col(), 0);
    }

    #[test]
    fn test_delete_char_at_line_end_merges_lines() {
        let mut buffer = Buffer::new();
        buffer.insert_char('a');
        buffer.insert_newline();
        buffer.insert_char('b');
        buffer.move_cursor_up();
        buffer.move_cursor_to_line_end();
        assert!(buffer.delete_char_at());
        assert_eq!(buffer.line_count(), 1);
        assert_eq!(buffer.lines()[0], "ab");
    }

    #[test]
    fn test_delete_char_at_buffer_end_returns_false() {
        let mut buffer = Buffer::new();
        buffer.insert_char('a');
        assert!(!buffer.delete_char_at());
        assert_eq!(buffer.lines()[0], "a");
    }

    // === Cursor Left Movement Tests ===

    #[test]
    fn test_move_cursor_left_within_line() {
        let mut buffer = Buffer::new();
        buffer.insert_char('a');
        buffer.insert_char('b');
        buffer.move_cursor_left();
        assert_eq!(buffer.cursor().col(), 1);
    }

    #[test]
    fn test_move_cursor_left_at_line_start_goes_to_prev_line_end() {
        let mut buffer = Buffer::new();
        buffer.insert_char('a');
        buffer.insert_char('b');
        buffer.insert_newline();
        buffer.insert_char('c');
        buffer.move_cursor_to_line_start();
        buffer.move_cursor_left();
        assert_eq!(buffer.cursor().row(), 0);
        assert_eq!(buffer.cursor().col(), 2);
    }

    #[test]
    fn test_move_cursor_left_at_buffer_start_stays() {
        let mut buffer = Buffer::new();
        buffer.insert_char('a');
        buffer.move_cursor_to_line_start();
        buffer.move_cursor_left();
        assert_eq!(buffer.cursor().row(), 0);
        assert_eq!(buffer.cursor().col(), 0);
    }

    // === Cursor Right Movement Tests ===

    #[test]
    fn test_move_cursor_right_within_line() {
        let mut buffer = Buffer::new();
        buffer.insert_char('a');
        buffer.insert_char('b');
        buffer.move_cursor_to_line_start();
        buffer.move_cursor_right();
        assert_eq!(buffer.cursor().col(), 1);
    }

    #[test]
    fn test_move_cursor_right_at_line_end_goes_to_next_line_start() {
        let mut buffer = Buffer::new();
        buffer.insert_char('a');
        buffer.insert_newline();
        buffer.insert_char('b');
        buffer.move_cursor_up();
        buffer.move_cursor_to_line_end();
        buffer.move_cursor_right();
        assert_eq!(buffer.cursor().row(), 1);
        assert_eq!(buffer.cursor().col(), 0);
    }

    #[test]
    fn test_move_cursor_right_at_buffer_end_stays() {
        let mut buffer = Buffer::new();
        buffer.insert_char('a');
        buffer.move_cursor_right();
        assert_eq!(buffer.cursor().col(), 1);
    }

    // === Cursor Up Movement Tests ===

    #[test]
    fn test_move_cursor_up() {
        let mut buffer = Buffer::new();
        buffer.insert_char('a');
        buffer.insert_newline();
        buffer.insert_char('b');
        buffer.move_cursor_up();
        assert_eq!(buffer.cursor().row(), 0);
    }

    #[test]
    fn test_move_cursor_up_clamps_column() {
        let mut buffer = Buffer::new();
        buffer.insert_char('a');
        buffer.insert_newline();
        buffer.insert_char('b');
        buffer.insert_char('c');
        buffer.insert_char('d');
        buffer.move_cursor_up();
        assert_eq!(buffer.cursor().row(), 0);
        assert_eq!(buffer.cursor().col(), 1); // Clamped to line length
    }

    #[test]
    fn test_move_cursor_up_at_first_line_stays() {
        let mut buffer = Buffer::new();
        buffer.insert_char('a');
        buffer.move_cursor_up();
        assert_eq!(buffer.cursor().row(), 0);
    }

    // === Cursor Down Movement Tests ===

    #[test]
    fn test_move_cursor_down() {
        let mut buffer = Buffer::new();
        buffer.insert_char('a');
        buffer.insert_newline();
        buffer.insert_char('b');
        buffer.move_cursor_up();
        buffer.move_cursor_down();
        assert_eq!(buffer.cursor().row(), 1);
    }

    #[test]
    fn test_move_cursor_down_clamps_column() {
        let mut buffer = Buffer::new();
        buffer.insert_char('a');
        buffer.insert_char('b');
        buffer.insert_char('c');
        buffer.insert_newline();
        buffer.insert_char('d');
        buffer.move_cursor_up();
        buffer.move_cursor_to_line_end();
        buffer.move_cursor_down();
        assert_eq!(buffer.cursor().row(), 1);
        assert_eq!(buffer.cursor().col(), 1); // Clamped to line length
    }

    #[test]
    fn test_move_cursor_down_at_last_line_stays() {
        let mut buffer = Buffer::new();
        buffer.insert_char('a');
        buffer.move_cursor_down();
        assert_eq!(buffer.cursor().row(), 0);
    }

    // === Home/End Movement Tests ===

    #[test]
    fn test_move_cursor_to_line_start() {
        let mut buffer = Buffer::new();
        buffer.insert_char('a');
        buffer.insert_char('b');
        buffer.insert_char('c');
        buffer.move_cursor_to_line_start();
        assert_eq!(buffer.cursor().col(), 0);
    }

    #[test]
    fn test_move_cursor_to_line_end() {
        let mut buffer = Buffer::new();
        buffer.insert_char('a');
        buffer.insert_char('b');
        buffer.insert_char('c');
        buffer.move_cursor_to_line_start();
        buffer.move_cursor_to_line_end();
        assert_eq!(buffer.cursor().col(), 3);
    }

    // === Content Retrieval Tests ===

    #[test]
    fn test_content_single_line() {
        let mut buffer = Buffer::new();
        buffer.insert_char('h');
        buffer.insert_char('i');
        assert_eq!(buffer.content(), "hi");
    }

    #[test]
    fn test_content_multiple_lines() {
        let mut buffer = Buffer::new();
        buffer.insert_char('a');
        buffer.insert_newline();
        buffer.insert_char('b');
        assert_eq!(buffer.content(), "a\nb");
    }

    #[test]
    fn test_current_line() {
        let mut buffer = Buffer::new();
        buffer.insert_char('a');
        buffer.insert_newline();
        buffer.insert_char('b');
        assert_eq!(buffer.current_line(), "b");
        buffer.move_cursor_up();
        assert_eq!(buffer.current_line(), "a");
    }

    // === from_lines Tests ===

    #[test]
    fn test_from_lines_with_content() {
        let lines = vec!["hello".to_string(), "world".to_string()];
        let buffer = Buffer::from_lines(lines);
        assert_eq!(buffer.line_count(), 2);
        assert_eq!(buffer.lines()[0], "hello");
        assert_eq!(buffer.lines()[1], "world");
    }

    #[test]
    fn test_from_lines_empty_vec_creates_single_empty_line() {
        let buffer = Buffer::from_lines(vec![]);
        assert_eq!(buffer.line_count(), 1);
        assert_eq!(buffer.lines()[0], "");
    }

    #[test]
    fn test_from_lines_cursor_at_origin() {
        let lines = vec!["hello".to_string(), "world".to_string()];
        let buffer = Buffer::from_lines(lines);
        assert_eq!(buffer.cursor().row(), 0);
        assert_eq!(buffer.cursor().col(), 0);
    }

    #[test]
    fn test_from_lines_single_line() {
        let buffer = Buffer::from_lines(vec!["single".to_string()]);
        assert_eq!(buffer.line_count(), 1);
        assert_eq!(buffer.lines()[0], "single");
    }

    // === Clear Tests ===

    #[test]
    fn test_clear_resets_to_single_empty_line() {
        let mut buffer = Buffer::new();
        buffer.insert_char('a');
        buffer.insert_newline();
        buffer.insert_char('b');
        buffer.insert_newline();
        buffer.insert_char('c');

        buffer.clear();

        assert_eq!(buffer.line_count(), 1);
        assert_eq!(buffer.lines()[0], "");
    }

    #[test]
    fn test_clear_resets_cursor_to_origin() {
        let mut buffer = Buffer::new();
        buffer.insert_char('a');
        buffer.insert_char('b');
        buffer.insert_newline();
        buffer.insert_char('c');

        buffer.clear();

        assert_eq!(buffer.cursor().row(), 0);
        assert_eq!(buffer.cursor().col(), 0);
    }

    #[test]
    fn test_clear_on_empty_buffer() {
        let mut buffer = Buffer::new();
        buffer.clear();

        assert_eq!(buffer.line_count(), 1);
        assert_eq!(buffer.lines()[0], "");
        assert_eq!(buffer.cursor().row(), 0);
        assert_eq!(buffer.cursor().col(), 0);
    }

    #[test]
    fn test_clear_allows_new_input() {
        let mut buffer = Buffer::new();
        buffer.insert_char('x');
        buffer.clear();
        buffer.insert_char('y');

        assert_eq!(buffer.line_count(), 1);
        assert_eq!(buffer.lines()[0], "y");
        assert_eq!(buffer.cursor().col(), 1);
    }
}
