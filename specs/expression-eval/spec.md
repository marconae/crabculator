# Expression Evaluation Specification

## Purpose

Parses and evaluates mathematical expressions from editor lines, managing variable context and producing results or errors for each line.

## Requirements

### Requirement: Expression Parsing

The system SHALL parse each line to determine whether it is an assignment, an expression, or empty.

#### Scenario: Parse assignment expression

- **WHEN** a line contains `name = expression` format
- **THEN** the parser identifies it as an assignment
- **AND** extracts the variable name and expression separately

#### Scenario: Parse standalone expression

- **WHEN** a line contains an expression without `=`
- **THEN** the parser identifies it as a standalone expression

#### Scenario: Parse empty line

- **WHEN** a line is empty or contains only whitespace
- **THEN** the parser identifies it as empty
- **AND** no evaluation is performed

### Requirement: Expression Evaluation

The system SHALL evaluate mathematical expressions and return results.

#### Scenario: Evaluate arithmetic expression

- **WHEN** a valid arithmetic expression is evaluated (e.g., `5 + 3 * 2`)
- **THEN** the system returns the computed numeric result (e.g., `11`)

#### Scenario: Evaluate expression with parentheses

- **WHEN** an expression with parentheses is evaluated (e.g., `(5 + 3) * 2`)
- **THEN** the system respects operator precedence and grouping (e.g., `16`)

#### Scenario: Evaluate expression with built-in functions

- **WHEN** an expression uses built-in functions (e.g., `sqrt(16)`, `sin(pi/2)`)
- **THEN** the system evaluates the function and returns the result

#### Scenario: Evaluate invalid expression

- **WHEN** an invalid expression is evaluated (e.g., `5 + + 3`, `5 / 0`)
- **THEN** the system returns an error with a descriptive message

### Requirement: Variable Context

The system SHALL maintain a variable context that persists across line evaluations.

#### Scenario: Store variable from assignment

- **WHEN** an assignment expression is evaluated (e.g., `a = 5 + 3`)
- **THEN** the result is stored in the variable context under the given name
- **AND** the result value is returned (e.g., `8`)

#### Scenario: Reference stored variable

- **WHEN** an expression references a defined variable (e.g., `a * 2` after `a = 5`)
- **THEN** the variable's value is substituted and the expression is evaluated (e.g., `10`)

#### Scenario: Reference undefined variable

- **WHEN** an expression references an undefined variable
- **THEN** the system returns an error indicating the variable is not defined

### Requirement: Line-by-Line Evaluation

The system SHALL evaluate all lines in order, updating results when content changes.

#### Scenario: Evaluate all lines on change

- **WHEN** the buffer content changes
- **THEN** all lines are re-evaluated from top to bottom
- **AND** variable assignments are processed in order
- **AND** results for each line are updated

#### Scenario: Variable dependency across lines

- **WHEN** line 1 contains `a = 10` and line 2 contains `a + 5`
- **THEN** line 2 evaluates to `15` using the value from line 1
