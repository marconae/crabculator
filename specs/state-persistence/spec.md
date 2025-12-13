# State Persistence Specification

## Purpose

Enable users to preserve their work across application sessions. The system automatically saves and restores buffer content and variable definitions, ensuring no work is lost when closing and reopening Crabculator.

## Requirements

### Requirement: State Directory

The system SHALL use `~/.crabcalculator/` as the state directory on all platforms (Linux, macOS, Windows).

#### Scenario: Directory location on Unix

- **WHEN** the application resolves the state directory on Linux or macOS
- **THEN** the path SHALL be `$HOME/.crabcalculator/`

#### Scenario: Directory location on Windows

- **WHEN** the application resolves the state directory on Windows
- **THEN** the path SHALL be `%USERPROFILE%\.crabcalculator\`

#### Scenario: Directory creation

- **WHEN** the state directory does not exist
- **AND** the application attempts to save state
- **THEN** the directory SHALL be created automatically

### Requirement: State File Format

The system SHALL persist state in JSON format at `~/.crabcalculator/state.json`.

#### Scenario: State file structure

- **WHEN** state is persisted
- **THEN** the file SHALL contain a JSON object with `buffer_lines` (array of strings) and `variables` (object mapping names to numeric values)

### Requirement: Auto-Save

The system SHALL automatically save state when changes occur.

#### Scenario: Save on buffer change

- **WHEN** the user modifies the buffer content
- **THEN** the current state SHALL be saved to the state file

#### Scenario: Save on variable assignment

- **WHEN** a variable is assigned via expression evaluation
- **THEN** the current state SHALL be saved to the state file

### Requirement: Auto-Load

The system SHALL automatically load state on startup.

#### Scenario: Load existing state

- **WHEN** the application starts
- **AND** a valid state file exists
- **THEN** the buffer SHALL be populated with the saved lines
- **AND** the variable context SHALL be populated with saved variables

#### Scenario: Start with no state file

- **WHEN** the application starts
- **AND** no state file exists
- **THEN** the application SHALL start with an empty buffer
- **AND** the application SHALL start with an empty variable context

#### Scenario: Handle corrupted state file

- **WHEN** the application starts
- **AND** the state file exists but is corrupted or invalid
- **THEN** the application SHALL start with an empty buffer
- **AND** the application SHALL start with an empty variable context
- **AND** the application SHALL NOT crash
