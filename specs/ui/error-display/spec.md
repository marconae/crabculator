# Feature: Error Display

Provides visual feedback for expression errors through syntax highlighting (red underline) and inline error messages.

## Background

The error display system integrates with the expression evaluator to receive error information and renders visual indicators in the TUI. It operates within the editor panel to highlight problematic tokens and display descriptive error messages.

## Scenarios

### Scenario: Highlight syntax error token

* *GIVEN* the user has entered an expression
* *WHEN* an expression contains a syntax error
* *AND* the error position can be determined
* *THEN* the specific invalid token or expression portion SHALL be underlined
* *AND* the underline SHALL use the terminal's semantic red color (Color::Red)

### Scenario: Highlight entire expression on unparseable error

* *GIVEN* the user has entered an expression
* *WHEN* an expression contains an error
* *AND* the specific error position cannot be determined
* *THEN* the entire expression SHALL be underlined in red

### Scenario: Show error message below error line

* *GIVEN* an expression has been evaluated
* *WHEN* an expression evaluation produces an error
* *AND* at least 500 milliseconds have elapsed since the last edit on that line
* *THEN* the error message SHALL be displayed on the line below the expression
* *AND* the error message SHALL be styled with dim and italic modifiers
* *AND* the error message SHALL NOT use a fixed color (adapts to terminal theme)
* *AND* the error message SHALL include a caret (^) pointing to the error location
* *AND* the error message SHALL be indented to align with the error position

### Scenario: Suppress error message while typing

* *GIVEN* the user is editing an expression
* *WHEN* an expression contains an error
* *AND* fewer than 500 milliseconds have elapsed since the last edit
* *THEN* the error message SHALL NOT be displayed
* *AND* the error underline highlighting SHALL still be shown

### Scenario: Error message content

* *GIVEN* an error has occurred during evaluation
* *WHEN* an error message is displayed
* *THEN* the message SHALL describe the error (e.g., "Division by zero", "Unknown variable: x")

### Scenario: Display numeric result

* *GIVEN* the user has entered a valid expression
* *WHEN* an expression evaluates successfully to a number
* *THEN* the result SHALL be displayed in the right panel on the same row as the input
* *AND* integer results SHALL be displayed without decimal places
* *AND* floating-point results SHALL be displayed with appropriate precision

### Scenario: Display assignment result

* *GIVEN* the user has entered an assignment expression
* *WHEN* an assignment expression evaluates successfully
* *THEN* the result value SHALL be displayed in the right panel
* *AND* the variable name MAY optionally be shown (e.g., `a = 8`)

### Scenario: No result for empty line

* *GIVEN* the editor contains an empty line
* *WHEN* a line is empty
* *THEN* no result SHALL be displayed for that row in the right panel

### Scenario: No result for error line

* *GIVEN* a line contains an invalid expression
* *WHEN* a line has an evaluation error
* *THEN* no result value SHALL be displayed in the right panel for that row

### Scenario: Memory pane alignment with error messages

* *GIVEN* the editor contains an expression with an error
* *WHEN* the error message is displayed below the expression
* *THEN* the memory pane SHALL include an empty line corresponding to the error message line
* *AND* subsequent results SHALL remain visually aligned with their input expressions
