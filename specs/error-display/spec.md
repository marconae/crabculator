# Error Display Specification

## Purpose

Provides visual feedback for expression errors through syntax highlighting (red underline) and inline error messages.

## Requirements

### Requirement: Error Highlighting

The system SHALL highlight invalid tokens in expressions with a red underline.

#### Scenario: Highlight syntax error token

- **WHEN** an expression contains a syntax error
- **AND** the error position can be determined
- **THEN** the invalid token or position is underlined in red

#### Scenario: Highlight entire expression on unparseable error

- **WHEN** an expression contains an error
- **AND** the specific error position cannot be determined
- **THEN** the entire expression is underlined in red

### Requirement: Error Message Display

The system SHALL display error messages below lines with errors.

#### Scenario: Show error message below error line

- **WHEN** an expression evaluation produces an error
- **THEN** the error message is displayed on the line below the expression
- **AND** the error message is styled in red text
- **AND** the error message is indented to align with the expression

#### Scenario: Error message content

- **WHEN** an error message is displayed
- **THEN** the message describes the error (e.g., "Division by zero", "Unknown variable: x")

### Requirement: Result Display

The system SHALL display evaluation results in the right panel aligned with input lines.

#### Scenario: Display numeric result

- **WHEN** an expression evaluates successfully to a number
- **THEN** the result is displayed in the right panel on the same row as the input
- **AND** integer results are displayed without decimal places
- **AND** floating-point results are displayed with appropriate precision

#### Scenario: Display assignment result

- **WHEN** an assignment expression evaluates successfully
- **THEN** the result value is displayed in the right panel
- **AND** the variable name may optionally be shown (e.g., `a = 8`)

#### Scenario: No result for empty line

- **WHEN** a line is empty
- **THEN** no result is displayed for that row in the right panel

#### Scenario: No result for error line

- **WHEN** a line has an evaluation error
- **THEN** no result value is displayed in the right panel for that row
