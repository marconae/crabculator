# Feature: Error Display

Provides visual feedback for expression errors through syntax highlighting (red underline) and inline error messages.

## Background

The error display system integrates with the expression evaluator to receive error information and renders visual indicators in the TUI. It operates within the editor panel to highlight problematic tokens and display descriptive error messages.

## Scenarios

### Scenario: Highlight syntax error token

* *GIVEN* the user has entered an expression
* *WHEN* an expression contains a syntax error
* *AND* the error position can be determined
* *THEN* the invalid token or position SHALL be underlined in red

### Scenario: Highlight entire expression on unparseable error

* *GIVEN* the user has entered an expression
* *WHEN* an expression contains an error
* *AND* the specific error position cannot be determined
* *THEN* the entire expression SHALL be underlined in red

### Scenario: Show error message below error line

* *GIVEN* an expression has been evaluated
* *WHEN* an expression evaluation produces an error
* *THEN* the error message SHALL be displayed on the line below the expression
* *AND* the error message SHALL be styled in red text
* *AND* the error message SHALL be indented to align with the expression

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
