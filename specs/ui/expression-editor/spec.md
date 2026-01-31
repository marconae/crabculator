# Feature: Expression Editor

Provides a multi-line text editor interface for entering mathematical expressions, with cursor-based navigation and standard text editing operations.

## Background

The expression editor operates within a TUI terminal environment as the primary input component. It maintains a text buffer, handles keyboard input for editing and navigation, and supports syntax highlighting for mathematical expressions.

## Scenarios

### Scenario: Initial empty buffer

* *GIVEN* the user launches Crabculator
* *WHEN* the application starts
* *THEN* the buffer SHALL contain one empty line
* *AND* the cursor SHALL be positioned at row 0, column 0

### Scenario: Character insertion

* *GIVEN* the editor is focused and ready for input
* *WHEN* the user types a character
* *THEN* the character SHALL be inserted at the cursor position
* *AND* the cursor SHALL advance one column to the right

### Scenario: New line creation

* *GIVEN* the cursor is positioned in the editor
* *WHEN* the user presses Enter
* *THEN* a new line SHALL be created below the current line
* *AND* text after the cursor SHALL move to the new line
* *AND* the cursor SHALL move to the start of the new line

### Scenario: Character deletion with backspace

* *GIVEN* the cursor is not at the start of a line
* *WHEN* the user presses Backspace
* *THEN* the character before the cursor SHALL be deleted
* *AND* the cursor SHALL move one column to the left

### Scenario: Line merge with backspace at line start

* *GIVEN* the cursor is at the start of a line (not the first line)
* *WHEN* the user presses Backspace
* *THEN* the current line content SHALL be appended to the previous line
* *AND* the current line SHALL be removed
* *AND* the cursor SHALL move to the join position

### Scenario: Move cursor left

* *GIVEN* the cursor is not at column 0
* *WHEN* the user presses the Left arrow key
* *THEN* the cursor SHALL move one column to the left

### Scenario: Move cursor left at line start

* *GIVEN* the cursor is at column 0 and not on the first line
* *WHEN* the user presses the Left arrow key
* *THEN* the cursor SHALL move to the end of the previous line

### Scenario: Move cursor right

* *GIVEN* the cursor is not at the end of the line
* *WHEN* the user presses the Right arrow key
* *THEN* the cursor SHALL move one column to the right

### Scenario: Move cursor right at line end

* *GIVEN* the cursor is at the end of the line and not on the last line
* *WHEN* the user presses the Right arrow key
* *THEN* the cursor SHALL move to the start of the next line

### Scenario: Move cursor up

* *GIVEN* the cursor is not on the first line
* *WHEN* the user presses the Up arrow key
* *THEN* the cursor SHALL move to the previous line
* *AND* the column SHALL be preserved or clamped to line length

### Scenario: Move cursor down

* *GIVEN* the cursor is not on the last line
* *WHEN* the user presses the Down arrow key
* *THEN* the cursor SHALL move to the next line
* *AND* the column SHALL be preserved or clamped to line length

### Scenario: Move cursor to line start

* *GIVEN* the cursor is positioned in the editor
* *WHEN* the user presses the Home key
* *THEN* the cursor SHALL move to column 0 of the current line

### Scenario: Move cursor to line end

* *GIVEN* the cursor is positioned in the editor
* *WHEN* the user presses the End key
* *THEN* the cursor SHALL move to the end of the current line

### Scenario: Cursor rendering

* *GIVEN* the editor panel is being rendered
* *WHEN* the editor panel is rendered
* *THEN* a cursor indicator SHALL be shown at the current cursor position
* *AND* the cursor SHALL be visually distinct (e.g., block or line cursor)
* *AND* the cursor cell SHALL use a style that contrasts with the current line highlight

### Scenario: Line numbers render for all lines

* *GIVEN* the editor contains N lines
* *WHEN* the editor is rendered
* *THEN* numbers 1 through N SHALL appear in the gutter
* *AND* numbers SHALL be right-aligned within the gutter

### Scenario: Line numbers use subtle styling

* *GIVEN* the editor is being rendered
* *WHEN* viewing line numbers
* *THEN* line numbers SHALL display in dimmed foreground color (Gray)
* *AND* line number foreground color SHALL contrast with the current line highlight background
* *AND* line number background SHALL match the content area background

### Scenario: Variables display in cyan

* *GIVEN* an expression contains variable names
* *WHEN* the expression is rendered
* *THEN* variable names SHALL render in cyan color

### Scenario: Numbers display in default color

* *GIVEN* an expression contains numeric literals
* *WHEN* the expression is rendered
* *THEN* numbers SHALL render in white/default color

### Scenario: Operators display dimmed

* *GIVEN* an expression contains operators (+, -, *, /, %, ^, =)
* *WHEN* the expression is rendered
* *THEN* operators SHALL render in dimmed/grey color

### Scenario: Cursor moves beyond right edge

* *GIVEN* the cursor is near the right edge of the visible area
* *WHEN* the cursor column exceeds the visible width
* *THEN* the viewport SHALL scroll right to keep the cursor visible
* *AND* the cursor SHALL appear within the visible area

### Scenario: Cursor moves before left edge

* *GIVEN* the viewport is scrolled right
* *WHEN* the cursor column is before the horizontal scroll offset
* *THEN* the viewport SHALL scroll left to keep the cursor visible
* *AND* the cursor SHALL appear within the visible area

### Scenario: Cursor within visible area

* *GIVEN* the cursor is within the visible horizontal range
* *WHEN* the cursor column is within the visible horizontal range
* *THEN* the horizontal scroll offset SHALL remain unchanged

### Scenario: Long line input

* *GIVEN* the user is typing a long expression
* *WHEN* the user types characters that extend beyond the viewport width
* *THEN* the viewport SHALL scroll to keep the cursor visible
* *AND* the most recent characters SHALL remain in view

### Scenario: Line numbers align with title icon

* *GIVEN* the calculator pane is being rendered
* *WHEN* viewing the line number gutter
* *THEN* the line numbers SHALL be positioned to align with the crab icon in the title
* *AND* the line number column SHALL start at the same horizontal position as the icon

### Scenario: Math expressions align with title text

* *GIVEN* the calculator pane is being rendered
* *WHEN* viewing math expressions
* *THEN* the first character of each expression SHALL align with the "c" in "crabculator"
* *AND* this alignment SHALL be consistent across all lines
