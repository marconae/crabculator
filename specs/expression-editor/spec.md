# Expression Editor Specification

## Purpose

Provides a multi-line text editor interface for entering mathematical expressions, with cursor-based navigation and standard text editing operations.

## Requirements

### Requirement: Text Buffer

The editor SHALL maintain a buffer of text lines that users can edit.

#### Scenario: Initial empty buffer

- **WHEN** the application starts
- **THEN** the buffer contains one empty line
- **AND** the cursor is positioned at row 0, column 0

#### Scenario: Character insertion

- **WHEN** the user types a character
- **THEN** the character is inserted at the cursor position
- **AND** the cursor advances one column to the right

#### Scenario: New line creation

- **WHEN** the user presses Enter
- **THEN** a new line is created below the current line
- **AND** text after the cursor moves to the new line
- **AND** the cursor moves to the start of the new line

#### Scenario: Character deletion with backspace

- **WHEN** the user presses Backspace with cursor not at line start
- **THEN** the character before the cursor is deleted
- **AND** the cursor moves one column to the left

#### Scenario: Line merge with backspace at line start

- **WHEN** the user presses Backspace at the start of a line (not the first line)
- **THEN** the current line content is appended to the previous line
- **AND** the current line is removed
- **AND** the cursor moves to the join position

### Requirement: Cursor Navigation

The editor SHALL provide cursor navigation to move within and between lines.

#### Scenario: Move cursor left

- **WHEN** the user presses the Left arrow key
- **AND** the cursor is not at column 0
- **THEN** the cursor moves one column to the left

#### Scenario: Move cursor left at line start

- **WHEN** the user presses the Left arrow key
- **AND** the cursor is at column 0
- **AND** the cursor is not on the first line
- **THEN** the cursor moves to the end of the previous line

#### Scenario: Move cursor right

- **WHEN** the user presses the Right arrow key
- **AND** the cursor is not at the end of the line
- **THEN** the cursor moves one column to the right

#### Scenario: Move cursor right at line end

- **WHEN** the user presses the Right arrow key
- **AND** the cursor is at the end of the line
- **AND** the cursor is not on the last line
- **THEN** the cursor moves to the start of the next line

#### Scenario: Move cursor up

- **WHEN** the user presses the Up arrow key
- **AND** the cursor is not on the first line
- **THEN** the cursor moves to the previous line
- **AND** the column is preserved or clamped to line length

#### Scenario: Move cursor down

- **WHEN** the user presses the Down arrow key
- **AND** the cursor is not on the last line
- **THEN** the cursor moves to the next line
- **AND** the column is preserved or clamped to line length

#### Scenario: Move cursor to line start

- **WHEN** the user presses the Home key
- **THEN** the cursor moves to column 0 of the current line

#### Scenario: Move cursor to line end

- **WHEN** the user presses the End key
- **THEN** the cursor moves to the end of the current line

### Requirement: Cursor Visibility

The editor SHALL display a visible cursor indicating the current editing position.

#### Scenario: Cursor rendering

- **WHEN** the editor panel is rendered
- **THEN** a cursor indicator is shown at the current cursor position
- **AND** the cursor is visually distinct (e.g., block or line cursor)
