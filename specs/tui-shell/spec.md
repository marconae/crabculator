# TUI Shell Specification

## Purpose

Defines the terminal user interface shell that provides the visual layout, event handling, and application lifecycle management for Crabculator.

## Requirements

### Requirement: Application Lifecycle

The application SHALL manage a clean terminal lifecycle with proper initialization and cleanup.

#### Scenario: Terminal enters raw mode on start

- **WHEN** the application starts
- **THEN** the terminal enters raw mode
- **AND** the alternate screen is enabled
- **AND** the cursor is hidden

#### Scenario: Terminal restores on exit

- **WHEN** the application exits (normally or via Ctrl+C)
- **THEN** the terminal exits raw mode
- **AND** the main screen is restored
- **AND** the cursor is visible

### Requirement: Split Panel Layout

The application SHALL display a split panel layout with rounded borders.

#### Scenario: Layout renders with rounded borders

- **WHEN** the application starts
- **THEN** the left panel occupies approximately 80% of terminal width
- **AND** the right panel occupies approximately 20% of terminal width
- **AND** both panels have rounded corner borders
- **AND** borders are colored dark grey

#### Scenario: Layout adapts to terminal resize

- **WHEN** the terminal is resized
- **THEN** panels maintain their proportional widths
- **AND** content reflows to fit new dimensions
- **AND** rounded borders and dark grey color are preserved

### Requirement: Panel Identification

Each panel SHALL be visually identifiable with a branded title.

#### Scenario: Panels have branded titles

- **WHEN** viewing the application
- **THEN** the left panel displays title "🦀CrabCalculator"
- **AND** the right panel displays title "Memory"

#### Scenario: Emoji fallback for unsupported terminals

- **WHEN** terminal does not support emoji rendering
- **THEN** the left panel displays title "CrabCalculator" without emoji

### Requirement: Application Exit

The user SHALL be able to exit the application cleanly.

#### Scenario: Exit via Ctrl+C

- **WHEN** user presses Ctrl+C
- **THEN** the application exits
- **AND** terminal is restored to normal state

#### Scenario: Exit via Ctrl+Q

- **WHEN** user presses Ctrl+Q
- **THEN** the application exits
- **AND** terminal is restored to normal state

### Requirement: Event Loop

The application SHALL run an event loop that processes keyboard input and renders the UI.

#### Scenario: UI updates on input

- **WHEN** user presses any key
- **THEN** the event is captured
- **AND** the UI is re-rendered if necessary

### Requirement: Command Bar

A command bar SHALL be displayed at the bottom of the screen showing available keyboard shortcuts.

#### Scenario: Command bar displays all commands

- **WHEN** viewing the application
- **THEN** a command bar appears at the bottom of the screen
- **AND** displays "CTRL+Q: quit"
- **AND** displays "CTRL+R: clear"
- **AND** displays "CTRL+H: help"
- **AND** displays "↑↓: history"

### Requirement: Current Line Highlighting

The current cursor line SHALL be visually highlighted across the full width of both panels.

#### Scenario: Editor pane highlights current line full-width

- **WHEN** cursor is on a line in the editor
- **THEN** that line's background highlight extends to the full panel width

#### Scenario: Results pane highlights corresponding line full-width

- **WHEN** cursor is on a line in the editor
- **THEN** the corresponding results line's background highlight extends to the full panel width

### Requirement: Scrollable Panes

Both panes SHALL be scrollable when content exceeds the visible area.

#### Scenario: Editor pane scrolls on overflow

- **WHEN** the editor content exceeds the visible height
- **THEN** the pane scrolls to keep the cursor visible
- **AND** only visible lines are rendered

#### Scenario: Results pane scrolls with editor

- **WHEN** the editor pane scrolls
- **THEN** the results pane scrolls to the same position
- **AND** corresponding lines remain aligned

### Requirement: Clear Buffer

The user SHALL be able to clear the editor buffer.

#### Scenario: Clear via CTRL+r

- **WHEN** user presses CTRL+r
- **THEN** all lines are removed from the editor buffer
- **AND** the cursor is reset to row 0, column 0
- **AND** the results pane is cleared

### Requirement: Help Overlay

The system SHALL provide a help overlay panel accessible via CTRL+H that displays usage information and function reference.

#### Scenario: Open help overlay

- **WHEN** user presses CTRL+H
- **THEN** a centered overlay panel appears on top of the main interface
- **AND** the overlay displays a bordered panel with title "Help"

#### Scenario: Close help overlay with CTRL+H

- **WHEN** help overlay is visible and user presses CTRL+H
- **THEN** the overlay closes
- **AND** normal editor input resumes

#### Scenario: Close help overlay with ESC

- **WHEN** help overlay is visible and user presses ESC
- **THEN** the overlay closes
- **AND** normal editor input resumes

#### Scenario: Help overlay content sections

- **WHEN** help overlay is displayed
- **THEN** it shows a "General Usage" section explaining basic calculator operations
- **AND** it shows a "Function Reference" section listing available mathematical functions

#### Scenario: Help overlay scrolling

- **WHEN** help overlay content exceeds the visible area
- **THEN** user can scroll using arrow keys (Up/Down) or Page Up/Page Down
- **AND** scroll position is indicated visually

#### Scenario: Help overlay modal behavior

- **WHEN** help overlay is visible
- **THEN** keyboard input is captured by the overlay
- **AND** editor does not receive input until overlay is closed
