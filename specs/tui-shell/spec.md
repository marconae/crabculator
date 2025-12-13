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

The application SHALL display a split panel layout with an 80/20 width ratio.

#### Scenario: Layout renders correctly

- **WHEN** the application is running
- **THEN** the left panel occupies 80% of the terminal width
- **AND** the right panel occupies 20% of the terminal width
- **AND** panels are separated by a visible border

#### Scenario: Layout adapts to terminal resize

- **WHEN** the terminal is resized
- **THEN** the panels maintain the 80/20 ratio
- **AND** content reflows appropriately

### Requirement: Panel Identification

Each panel SHALL be visually identifiable with a title.

#### Scenario: Panels have titles

- **WHEN** viewing the application
- **THEN** the left panel displays title "Input"
- **AND** the right panel displays title "Results"

### Requirement: Application Exit

The user SHALL be able to exit the application cleanly.

#### Scenario: Exit via Ctrl+C

- **WHEN** user presses Ctrl+C
- **THEN** the application exits
- **AND** terminal is restored to normal state

#### Scenario: Exit via 'q' key

- **WHEN** user presses 'q'
- **THEN** the application exits
- **AND** terminal is restored to normal state

### Requirement: Event Loop

The application SHALL run an event loop that processes keyboard input and renders the UI.

#### Scenario: UI updates on input

- **WHEN** user presses any key
- **THEN** the event is captured
- **AND** the UI is re-rendered if necessary
