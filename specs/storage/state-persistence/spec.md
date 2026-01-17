# Feature: State Persistence

Enable users to preserve their work across application sessions. The system automatically saves and restores buffer content, ensuring no work is lost when closing and reopening Crabculator.

## Background

State persistence uses the filesystem to store application state in a platform-specific user data directory. The state is stored as plain text, with one buffer line per file line. Variables are not persisted; they are recomputed by evaluating the buffer lines on load.

## Scenarios

### Scenario: Directory location on Unix

* *GIVEN* the application is running on Linux or macOS
* *WHEN* the application resolves the state directory
* *THEN* the path SHALL be `$HOME/.crabcalculator/`

### Scenario: Directory location on Windows

* *GIVEN* the application is running on Windows
* *WHEN* the application resolves the state directory
* *THEN* the path SHALL be `%USERPROFILE%\.crabcalculator\`

### Scenario: Directory creation

* *GIVEN* the state directory does not exist
* *WHEN* the application attempts to save state
* *THEN* the directory SHALL be created automatically

### Scenario: State file structure

* *GIVEN* the application has state to persist
* *WHEN* state is persisted
* *THEN* the file SHALL be a plain text file
* *AND* each line of the buffer SHALL be stored as one line in the file
* *AND* the file SHALL NOT contain any JSON or structured data
* *AND* the file SHALL NOT contain variable definitions

### Scenario: State file location

* *GIVEN* the application is running
* *WHEN* the application resolves the state file
* *THEN* the path SHALL be `~/.crabculator/state.txt` on Unix
* *AND* the path SHALL be the equivalent user data directory on Windows

### Scenario: Save on buffer change

* *GIVEN* the application is running with state persistence enabled
* *WHEN* the user modifies the buffer content
* *THEN* the current state SHALL be saved to the state file

### Scenario: Save on variable assignment

* *GIVEN* the application is running with state persistence enabled
* *WHEN* a variable is assigned via expression evaluation
* *THEN* the current state SHALL be saved to the state file

### Scenario: Load existing state

* *GIVEN* a valid state file exists
* *WHEN* the application starts
* *THEN* the buffer SHALL be populated with the saved lines
* *AND* the variable context SHALL be populated by evaluating the buffer lines

### Scenario: Handle missing state file

* *GIVEN* no state file exists
* *WHEN* the application starts
* *THEN* the application SHALL start with an empty buffer
* *AND* the application SHALL start with an empty variable context

### Scenario: Handle corrupted state file

* *GIVEN* the state file exists but contains invalid UTF-8
* *WHEN* the application starts
* *THEN* the application SHALL start with an empty buffer
* *AND* the application SHALL start with an empty variable context
* *AND* the application SHALL NOT crash
