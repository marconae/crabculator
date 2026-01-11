# Feature: State Persistence

Enable users to preserve their work across application sessions. The system automatically saves and restores buffer content and variable definitions, ensuring no work is lost when closing and reopening Crabculator.

## Background

State persistence uses the filesystem to store application state in a platform-specific user data directory. The state includes the editor buffer contents and all defined variables, serialized as JSON.

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
* *THEN* the file SHALL contain a JSON object with `buffer_lines` (array of strings) and `variables` (object mapping names to numeric values)

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
* *AND* the variable context SHALL be populated with saved variables

### Scenario: Start with no state file

* *GIVEN* no state file exists
* *WHEN* the application starts
* *THEN* the application SHALL start with an empty buffer
* *AND* the application SHALL start with an empty variable context

### Scenario: Handle corrupted state file

* *GIVEN* the state file exists but is corrupted or invalid
* *WHEN* the application starts
* *THEN* the application SHALL start with an empty buffer
* *AND* the application SHALL start with an empty variable context
* *AND* the application SHALL NOT crash
