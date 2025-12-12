# Project Setup Specification

## Purpose

Establishes the foundational Rust project structure, dependencies, and tooling configuration required for Crabculator development.

## Requirements

### Requirement: Cargo Project Structure

The project SHALL be a valid Rust 2024 edition Cargo project with a binary target.

#### Scenario: Project compiles successfully

- **WHEN** running `cargo build`
- **THEN** the project compiles without errors
- **AND** produces a binary executable

#### Scenario: Rust edition is 2024

- **WHEN** inspecting `Cargo.toml`
- **THEN** the edition field is set to "2024"

### Requirement: Core Dependencies

The project SHALL include the following dependencies:

- ratatui (TUI framework)
- crossterm (terminal backend)
- evalexpr (math expression parsing)
- dirs (OS-specific directories)
- serde and serde_json (serialization)

#### Scenario: Dependencies are declared

- **WHEN** inspecting `Cargo.toml`
- **THEN** all required dependencies are listed in `[dependencies]`

### Requirement: Code Quality Tooling

The project SHALL be configured for code quality enforcement using standard Cargo tooling.

#### Scenario: Clippy linting passes

- **WHEN** running `cargo clippy`
- **THEN** no warnings or errors are reported

#### Scenario: Code is formatted

- **WHEN** running `cargo fmt --check`
- **THEN** no formatting changes are required

### Requirement: Test Infrastructure

The project SHALL have test infrastructure configured and passing.

#### Scenario: Tests can be executed

- **WHEN** running `cargo test`
- **THEN** the test harness executes successfully
- **AND** all tests pass
