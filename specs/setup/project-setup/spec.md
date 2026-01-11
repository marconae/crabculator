# Feature: Project Setup

Establishes the foundational Rust project structure, dependencies, and tooling configuration required for Crabculator development.

## Background

The project setup defines the Cargo workspace configuration, required dependencies, and code quality tooling that ensures consistent development practices across the team. Required dependencies include: ratatui (TUI framework), crossterm (terminal backend), dirs (OS-specific directories), serde and serde_json (serialization).

## Scenarios

### Scenario: Project compiles successfully

* *GIVEN* the project source code is present
* *WHEN* running `cargo build`
* *THEN* the project SHALL compile without errors
* *AND* it SHALL produce a binary executable

### Scenario: Rust edition is 2024

* *GIVEN* the project is configured
* *WHEN* inspecting `Cargo.toml`
* *THEN* the edition field SHALL be set to "2024"

### Scenario: Dependencies are declared

* *GIVEN* the project is configured
* *WHEN* inspecting `Cargo.toml`
* *THEN* all required dependencies SHALL be listed in `[dependencies]`

### Scenario: Clippy linting passes

* *GIVEN* the project source code is present
* *WHEN* running `cargo clippy`
* *THEN* no warnings or errors SHALL be reported

### Scenario: Code is formatted

* *GIVEN* the project source code is present
* *WHEN* running `cargo fmt --check`
* *THEN* no formatting changes SHALL be required

### Scenario: Tests can be executed

* *GIVEN* the project and test infrastructure are present
* *WHEN* running `cargo test`
* *THEN* the test harness SHALL execute successfully
* *AND* all tests SHALL pass
