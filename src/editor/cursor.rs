//! Cursor management for the expression editor.
//!
//! Handles cursor position and navigation within the text buffer.

/// Represents a cursor position in a text buffer (0-indexed row and column).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Cursor {
    row: usize,
    col: usize,
}

impl Cursor {
    /// Creates a new cursor at the specified position.
    #[must_use]
    pub const fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    /// Returns the current row position.
    #[must_use]
    pub const fn row(&self) -> usize {
        self.row
    }

    /// Returns the current column position.
    #[must_use]
    pub const fn col(&self) -> usize {
        self.col
    }

    /// Sets the row position.
    pub const fn set_row(&mut self, row: usize) {
        self.row = row;
    }

    /// Sets the column position.
    pub const fn set_col(&mut self, col: usize) {
        self.col = col;
    }

    /// Moves the cursor left within a line.
    ///
    /// Returns `true` if the cursor moved, `false` if already at column 0.
    pub const fn move_left(&mut self) -> bool {
        if self.col > 0 {
            self.col -= 1;
            true
        } else {
            false
        }
    }

    /// Moves the cursor right within a line.
    ///
    /// # Arguments
    /// * `line_len` - The length of the current line
    ///
    /// Returns `true` if the cursor moved, `false` if already at end of line.
    pub const fn move_right(&mut self, line_len: usize) -> bool {
        if self.col < line_len {
            self.col += 1;
            true
        } else {
            false
        }
    }

    /// Moves the cursor up to the previous line.
    ///
    /// # Arguments
    /// * `prev_line_len` - The length of the previous line (for column clamping)
    ///
    /// Returns `true` if the cursor moved, `false` if already on first line.
    pub fn move_up(&mut self, prev_line_len: usize) -> bool {
        if self.row > 0 {
            self.row -= 1;
            self.col = self.col.min(prev_line_len);
            true
        } else {
            false
        }
    }

    /// Moves the cursor down to the next line.
    ///
    /// # Arguments
    /// * `total_lines` - Total number of lines in the buffer
    /// * `next_line_len` - The length of the next line (for column clamping)
    ///
    /// Returns `true` if the cursor moved, `false` if already on last line.
    pub fn move_down(&mut self, total_lines: usize, next_line_len: usize) -> bool {
        if self.row + 1 < total_lines {
            self.row += 1;
            self.col = self.col.min(next_line_len);
            true
        } else {
            false
        }
    }

    /// Moves the cursor to the start of the current line (column 0).
    pub const fn move_to_line_start(&mut self) {
        self.col = 0;
    }

    /// Moves the cursor to the end of the current line.
    ///
    /// # Arguments
    /// * `line_len` - The length of the current line
    pub const fn move_to_line_end(&mut self, line_len: usize) {
        self.col = line_len;
    }

    /// Moves the cursor to the end of the previous line.
    ///
    /// Used when pressing left at the start of a line.
    ///
    /// # Arguments
    /// * `prev_line_len` - The length of the previous line
    ///
    /// Returns `true` if the cursor moved, `false` if already on first line.
    pub const fn move_to_prev_line_end(&mut self, prev_line_len: usize) -> bool {
        if self.row > 0 {
            self.row -= 1;
            self.col = prev_line_len;
            true
        } else {
            false
        }
    }

    /// Moves the cursor to the start of the next line.
    ///
    /// Used when pressing right at the end of a line.
    ///
    /// # Arguments
    /// * `total_lines` - Total number of lines in the buffer
    ///
    /// Returns `true` if the cursor moved, `false` if already on last line.
    pub const fn move_to_next_line_start(&mut self, total_lines: usize) -> bool {
        if self.row + 1 < total_lines {
            self.row += 1;
            self.col = 0;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor_new_at_origin() {
        let cursor = Cursor::new(0, 0);
        assert_eq!(cursor.row(), 0);
        assert_eq!(cursor.col(), 0);
    }

    #[test]
    fn test_cursor_new_at_position() {
        let cursor = Cursor::new(5, 10);
        assert_eq!(cursor.row(), 5);
        assert_eq!(cursor.col(), 10);
    }

    #[test]
    fn test_cursor_default_is_origin() {
        let cursor = Cursor::default();
        assert_eq!(cursor.row(), 0);
        assert_eq!(cursor.col(), 0);
    }

    #[test]
    fn test_cursor_set_row() {
        let mut cursor = Cursor::default();
        cursor.set_row(3);
        assert_eq!(cursor.row(), 3);
    }

    #[test]
    fn test_cursor_set_col() {
        let mut cursor = Cursor::default();
        cursor.set_col(7);
        assert_eq!(cursor.col(), 7);
    }

    #[test]
    fn test_move_left_from_middle() {
        let mut cursor = Cursor::new(0, 5);
        assert!(cursor.move_left());
        assert_eq!(cursor.col(), 4);
    }

    #[test]
    fn test_move_left_at_column_zero_returns_false() {
        let mut cursor = Cursor::new(0, 0);
        assert!(!cursor.move_left());
        assert_eq!(cursor.col(), 0);
    }

    #[test]
    fn test_move_right_within_line() {
        let mut cursor = Cursor::new(0, 3);
        assert!(cursor.move_right(10));
        assert_eq!(cursor.col(), 4);
    }

    #[test]
    fn test_move_right_at_line_end_returns_false() {
        let mut cursor = Cursor::new(0, 5);
        assert!(!cursor.move_right(5));
        assert_eq!(cursor.col(), 5);
    }

    #[test]
    fn test_move_up_from_middle() {
        let mut cursor = Cursor::new(3, 5);
        assert!(cursor.move_up(10));
        assert_eq!(cursor.row(), 2);
        assert_eq!(cursor.col(), 5);
    }

    #[test]
    fn test_move_up_clamps_column_to_shorter_line() {
        let mut cursor = Cursor::new(2, 10);
        assert!(cursor.move_up(5));
        assert_eq!(cursor.row(), 1);
        assert_eq!(cursor.col(), 5);
    }

    #[test]
    fn test_move_up_at_first_line_returns_false() {
        let mut cursor = Cursor::new(0, 5);
        assert!(!cursor.move_up(10));
        assert_eq!(cursor.row(), 0);
    }

    #[test]
    fn test_move_down_from_middle() {
        let mut cursor = Cursor::new(1, 5);
        assert!(cursor.move_down(5, 10));
        assert_eq!(cursor.row(), 2);
        assert_eq!(cursor.col(), 5);
    }

    #[test]
    fn test_move_down_clamps_column_to_shorter_line() {
        let mut cursor = Cursor::new(1, 10);
        assert!(cursor.move_down(5, 3));
        assert_eq!(cursor.row(), 2);
        assert_eq!(cursor.col(), 3);
    }

    #[test]
    fn test_move_down_at_last_line_returns_false() {
        let mut cursor = Cursor::new(4, 5);
        assert!(!cursor.move_down(5, 10));
        assert_eq!(cursor.row(), 4);
    }

    #[test]
    fn test_move_to_line_start() {
        let mut cursor = Cursor::new(2, 15);
        cursor.move_to_line_start();
        assert_eq!(cursor.col(), 0);
        assert_eq!(cursor.row(), 2);
    }

    #[test]
    fn test_move_to_line_end() {
        let mut cursor = Cursor::new(2, 5);
        cursor.move_to_line_end(20);
        assert_eq!(cursor.col(), 20);
        assert_eq!(cursor.row(), 2);
    }

    #[test]
    fn test_move_to_prev_line_end() {
        let mut cursor = Cursor::new(2, 0);
        assert!(cursor.move_to_prev_line_end(8));
        assert_eq!(cursor.row(), 1);
        assert_eq!(cursor.col(), 8);
    }

    #[test]
    fn test_move_to_prev_line_end_at_first_line_returns_false() {
        let mut cursor = Cursor::new(0, 0);
        assert!(!cursor.move_to_prev_line_end(10));
        assert_eq!(cursor.row(), 0);
        assert_eq!(cursor.col(), 0);
    }

    #[test]
    fn test_move_to_next_line_start() {
        let mut cursor = Cursor::new(1, 5);
        assert!(cursor.move_to_next_line_start(5));
        assert_eq!(cursor.row(), 2);
        assert_eq!(cursor.col(), 0);
    }

    #[test]
    fn test_move_to_next_line_start_at_last_line_returns_false() {
        let mut cursor = Cursor::new(4, 5);
        assert!(!cursor.move_to_next_line_start(5));
        assert_eq!(cursor.row(), 4);
        assert_eq!(cursor.col(), 5);
    }
}
