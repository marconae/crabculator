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

### Requirement: Mathematical Functions

The system SHALL provide built-in mathematical functions accessible without namespace prefixes.

#### Scenario: Evaluate basic math function with literal

- **WHEN** an expression uses a basic math function with a numeric literal (e.g., `sqrt(16)`)
- **THEN** the system evaluates the function and returns the result (e.g., `4`)

#### Scenario: Evaluate function with variable argument

- **WHEN** an expression uses a function with a variable argument (e.g., `sqrt(x)` where `x = 9`)
- **THEN** the system substitutes the variable value and evaluates the function (e.g., `3`)

#### Scenario: Evaluate trigonometric function with constant

- **WHEN** an expression uses a trigonometric function with pi (e.g., `sin(pi/2)`)
- **THEN** the system evaluates using the mathematical constant (e.g., `1`)

#### Scenario: Function with invalid argument

- **WHEN** an expression uses a function with an invalid argument (e.g., `sqrt(-1)`, `log(0)`)
- **THEN** the system returns an appropriate error or NaN value

### Requirement: Mathematical Constants

The system SHALL provide built-in mathematical constants as pre-defined variables.

#### Scenario: Use pi constant

- **WHEN** an expression references `pi`
- **THEN** the system substitutes the value 3.141592653589793

#### Scenario: Use e constant

- **WHEN** an expression references `e`
- **THEN** the system substitutes the value 2.718281828459045

### Requirement: Supported Functions

The system SHALL support the following mathematical functions:

#### Scenario: Basic math functions

- **WHEN** user calls `sqrt(x)`, `cbrt(x)`, `abs(x)`, or `pow(base, exp)`
- **THEN** the system returns square root, cube root, absolute value, or power respectively

#### Scenario: Trigonometric functions

- **WHEN** user calls `sin(x)`, `cos(x)`, `tan(x)`, `asin(x)`, `acos(x)`, `atan(x)`, or `atan2(y, x)`
- **THEN** the system returns the trigonometric result (angles in radians)

#### Scenario: Hyperbolic functions

- **WHEN** user calls `sinh(x)`, `cosh(x)`, `tanh(x)`, `asinh(x)`, `acosh(x)`, or `atanh(x)`
- **THEN** the system returns the hyperbolic function result

#### Scenario: Logarithmic and exponential functions

- **WHEN** user calls `ln(x)`, `log(x)`, `log2(x)`, `log10(x)`, `exp(x)`, or `exp2(x)`
- **THEN** the system returns natural log, base-10 log, base-2 log, base-10 log, e^x, or 2^x respectively

#### Scenario: Rounding functions

- **WHEN** user calls `floor(x)`, `ceil(x)`, or `round(x)`
- **THEN** the system returns the rounded value (floor down, ceil up, round to nearest)

#### Scenario: Utility functions

- **WHEN** user calls `min(a, b)`, `max(a, b)`, or `hypot(a, b)`
- **THEN** the system returns minimum, maximum, or hypotenuse (sqrt(a^2 + b^2)) respectively
