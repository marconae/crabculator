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

### Scenario: Layout renders with rounded borders

* *GIVEN* the terminal is initialized
* *WHEN* the application starts
* *THEN* the left panel SHALL occupy approximately 80% of terminal width
* *AND* the right panel SHALL occupy approximately 20% of terminal width
* *AND* both panels SHALL have rounded corner borders
* *AND* borders SHALL be colored dark grey

### Scenario: Layout adapts to terminal resize

* *GIVEN* the application is displaying its layout
* *WHEN* the terminal is resized
* *THEN* panels SHALL maintain their proportional widths
* *AND* content SHALL reflow to fit new dimensions
* *AND* rounded borders and dark grey color SHALL be preserved

### Scenario: Panels have branded titles

* *GIVEN* the application is displaying its layout
* *WHEN* viewing the application
* *THEN* the left panel SHALL display title "CrabCalculator"
* *AND* the right panel SHALL display title "Memory"

### Scenario: Emoji fallback for unsupported terminals

* *GIVEN* the terminal does not support emoji rendering
* *WHEN* viewing the application
* *THEN* the left panel SHALL display title "CrabCalculator" without emoji

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
