# Feature: TUI Shell

Defines the terminal user interface shell that provides the visual layout, event handling, and application lifecycle management for Crabculator.

## Background

The TUI shell operates within a terminal environment using ratatui and crossterm for rendering and input handling. It manages the overall application window, panel layout, and coordinates between the expression editor and results display.

## Scenarios

### Scenario: Terminal enters raw mode on start

* *GIVEN* the user launches Crabculator
* *WHEN* the application starts
* *THEN* the terminal SHALL enter raw mode
* *AND* the alternate screen SHALL be enabled
* *AND* the cursor SHALL be hidden

### Scenario: Terminal restores on exit

* *GIVEN* the application is running
* *WHEN* the application exits (normally or via Ctrl+C)
* *THEN* the terminal SHALL exit raw mode
* *AND* the main screen SHALL be restored
* *AND* the cursor SHALL be visible

### Scenario: Layout adapts to terminal resize

* *GIVEN* the application is displaying its layout
* *WHEN* the terminal is resized
* *THEN* panels SHALL maintain their proportional widths
* *AND* content SHALL reflow to fit new dimensions

### Scenario: Panels have branded titles

* *GIVEN* the application is displaying its layout
* *WHEN* viewing the application
* *THEN* the calculator pane SHALL display title "🦀 crabculator"
* *AND* the memory pane SHALL display title "Memory"

### Scenario: Emoji fallback for unsupported terminals

* *GIVEN* the terminal does not support emoji rendering
* *WHEN* viewing the application
* *THEN* the calculator pane SHALL display title "crabculator" without emoji

### Scenario: Memory pane styling

* *GIVEN* the application is displaying its layout
* *WHEN* viewing the memory pane
* *THEN* the memory pane SHALL have a dark grey background
* *AND* the memory pane SHALL NOT have rounded corner borders
* *AND* the memory pane SHALL have a rusty-red border on the side adjacent to the calculator pane only

### Scenario: Memory pane border on right when pane is left

* *GIVEN* the application is displaying its layout
* *WHEN* the memory pane is on the left side
* *THEN* the memory pane border SHALL be on the right edge only

### Scenario: Memory pane border on left when pane is right

* *GIVEN* the application is displaying its layout
* *WHEN* the memory pane is on the right side
* *THEN* the memory pane border SHALL be on the left edge only

### Scenario: Calculator pane styling

* *GIVEN* the application is displaying its layout
* *WHEN* viewing the calculator pane
* *THEN* the calculator pane SHALL NOT have any side borders
* *AND* the calculator pane SHALL have a rusty-red underline below the title

### Scenario: Current line highlight uses rusty-red color

* *GIVEN* the editor contains multiple lines
* *WHEN* cursor is on a line
* *THEN* that line's background highlight SHALL use a rusty-red color

### Scenario: Command bar keyboard shortcuts use accent color

* *GIVEN* the application is displaying its layout
* *WHEN* viewing the command bar
* *THEN* keyboard shortcut text (e.g., "CTRL+Q", "CTRL+R") SHALL be displayed in rusty-red
* *AND* shortcut descriptions SHALL be displayed in default color

### Scenario: Exit via Ctrl+C

* *GIVEN* the application is running
* *WHEN* user presses Ctrl+C
* *THEN* the application SHALL exit
* *AND* terminal SHALL be restored to normal state

### Scenario: Exit via Ctrl+Q

* *GIVEN* the application is running
* *WHEN* user presses Ctrl+Q
* *THEN* the application SHALL exit
* *AND* terminal SHALL be restored to normal state

### Scenario: UI updates on input

* *GIVEN* the application is running its event loop
* *WHEN* user presses any key
* *THEN* the event SHALL be captured
* *AND* the UI SHALL be re-rendered if necessary

### Scenario: Command bar displays all commands

* *GIVEN* the application is displaying its layout
* *WHEN* viewing the application
* *THEN* a command bar SHALL appear at the bottom of the screen
* *AND* it SHALL display "CTRL+Q: quit"
* *AND* it SHALL display "CTRL+R: clear"
* *AND* it SHALL display "CTRL+H: help"
* *AND* it SHALL display "arrow-up/arrow-down: history"

### Scenario: Editor pane highlights current line full-width

* *GIVEN* the editor contains multiple lines
* *WHEN* cursor is on a line in the editor
* *THEN* that line's background highlight SHALL extend to the full panel width

### Scenario: Results pane highlights corresponding line full-width

* *GIVEN* the editor contains multiple lines with results
* *WHEN* cursor is on a line in the editor
* *THEN* the corresponding results line's background highlight SHALL extend to the full panel width

### Scenario: Editor pane scrolls on overflow

* *GIVEN* the editor content exceeds the visible height
* *WHEN* the user navigates through the content
* *THEN* the pane SHALL scroll to keep the cursor visible
* *AND* only visible lines SHALL be rendered

### Scenario: Results pane scrolls with editor

* *GIVEN* the editor pane has scrolled
* *WHEN* the editor pane scrolls
* *THEN* the results pane SHALL scroll to the same position
* *AND* corresponding lines SHALL remain aligned

### Scenario: Clear via CTRL+r

* *GIVEN* the editor contains content
* *WHEN* user presses CTRL+r
* *THEN* all lines SHALL be removed from the editor buffer
* *AND* the cursor SHALL be reset to row 0, column 0
* *AND* the results pane SHALL be cleared

### Scenario: Open help overlay

* *GIVEN* the application is running
* *WHEN* user presses CTRL+H
* *THEN* a centered overlay panel SHALL appear on top of the main interface
* *AND* the overlay SHALL display a bordered panel with title "Help"

### Scenario: Close help overlay with CTRL+H

* *GIVEN* help overlay is visible
* *WHEN* user presses CTRL+H
* *THEN* the overlay SHALL close
* *AND* normal editor input SHALL resume

### Scenario: Close help overlay with ESC

* *GIVEN* help overlay is visible
* *WHEN* user presses ESC
* *THEN* the overlay SHALL close
* *AND* normal editor input SHALL resume

### Scenario: Help overlay content sections

* *GIVEN* the help overlay is triggered
* *WHEN* help overlay is displayed
* *THEN* it SHALL show a "General Usage" section explaining basic calculator operations
* *AND* it SHALL show a "Function Reference" section listing available mathematical functions

### Scenario: Help overlay scrolling

* *GIVEN* the help overlay is displayed
* *WHEN* help overlay content exceeds the visible area
* *THEN* user MAY scroll using arrow keys (Up/Down) or Page Up/Page Down
* *AND* scroll position SHALL be indicated visually

### Scenario: Help overlay modal behavior

* *GIVEN* the help overlay is displayed
* *WHEN* help overlay is visible
* *THEN* keyboard input SHALL be captured by the overlay
* *AND* editor SHALL NOT receive input until overlay is closed

### Scenario: Memory pane defaults to left position

* *GIVEN* the application starts
* *WHEN* the UI renders
* *THEN* the memory pane SHALL be positioned on the left side
* *AND* the calculator pane SHALL be positioned on the right side
* *AND* the memory pane content SHALL be right-aligned

### Scenario: Move memory pane right via CTRL+Right

* *GIVEN* the memory pane is on the left
* *WHEN* user presses CTRL+Right arrow
* *THEN* the memory pane SHALL move to the right side
* *AND* the calculator pane SHALL move to the left side
* *AND* the memory pane content SHALL be left-aligned

### Scenario: Move memory pane left via CTRL+Left

* *GIVEN* the memory pane is on the right
* *WHEN* user presses CTRL+Left arrow
* *THEN* the memory pane SHALL move to the left side
* *AND* the calculator pane SHALL move to the right side
* *AND* the memory pane content SHALL be right-aligned

### Scenario: Memory pane position toggle is idempotent

* *GIVEN* the memory pane is already on the left
* *WHEN* user presses CTRL+Left arrow
* *THEN* the memory pane SHALL remain on the left side

### Scenario: Command bar displays memory pane shortcut

* *GIVEN* the application is displaying its layout
* *WHEN* viewing the application
* *THEN* the command bar SHALL display "CTRL+←/→: move memory"

### Scenario: Panel titles have underline border

* *GIVEN* the application is displaying its layout
* *WHEN* viewing any panel
* *THEN* the panel SHALL display a full-width rusty-red underline below the title row
* *AND* the underline SHALL span the entire panel width

### Scenario: Memory pane title right-aligned when pane is left

* *GIVEN* the application is displaying its layout
* *WHEN* the memory pane is on the left side
* *THEN* the memory pane title "Memory" SHALL be right-aligned

### Scenario: Memory pane title left-aligned when pane is right

* *GIVEN* the application is displaying its layout
* *WHEN* the memory pane is on the right side
* *THEN* the memory pane title "Memory" SHALL be left-aligned

### Scenario: Application uses unified accent color

* *GIVEN* the application is displaying its layout
* *WHEN* viewing any accent-colored element
* *THEN* the element SHALL use rusty-red color (RGB 139, 69, 19)
* *AND* this SHALL include title underlines, panel dividers, error messages, and keyboard shortcuts

### Scenario: Help overlay uses accent color border

* *GIVEN* the help overlay is displayed
* *WHEN* viewing the overlay
* *THEN* the overlay border SHALL use rusty-red color
* *AND* the overlay border SHALL NOT use cyan

### Scenario: Help overlay headers use white color

* *GIVEN* the help overlay is displayed
* *WHEN* viewing section headers
* *THEN* headers (lines starting with "===") SHALL be displayed in white
* *AND* headers SHALL NOT be displayed in yellow
