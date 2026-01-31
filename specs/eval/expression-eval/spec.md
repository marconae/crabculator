# Feature: Expression Evaluation

Parses and evaluates mathematical expressions from editor lines, managing variable context and producing results or errors for each line.

## Background

The expression evaluator is the computational core of Crabculator. It processes each line of input, determines whether it is an assignment or standalone expression, evaluates mathematical operations, and maintains a persistent variable context across lines.

## Scenarios

### Scenario: Parse assignment expression

* *GIVEN* the user has entered a line in the editor
* *WHEN* a line contains `name = expression` format
* *THEN* the parser SHALL identify it as an assignment
* *AND* the parser SHALL extract the variable name and expression separately

### Scenario: Parse standalone expression

* *GIVEN* the user has entered a line in the editor
* *WHEN* a line contains an expression without `=`
* *THEN* the parser SHALL identify it as a standalone expression

### Scenario: Parse empty line

* *GIVEN* the user has an empty line in the editor
* *WHEN* a line is empty or contains only whitespace
* *THEN* the parser SHALL identify it as empty
* *AND* no evaluation SHALL be performed

### Scenario: Evaluate arithmetic expression

* *GIVEN* the user has entered a valid arithmetic expression
* *WHEN* a valid arithmetic expression is evaluated (e.g., `5 + 3 * 2`)
* *THEN* the system SHALL return the computed numeric result (e.g., `11`)

### Scenario: Evaluate floating point expression

* *GIVEN* the user has entered an expression with floating point numbers
* *WHEN* an expression contains floating point numbers (e.g., `2.45565656 + 2.232323`)
* *THEN* the system SHALL parse decimal notation correctly
* *AND* the system SHALL return the floating point result (e.g., `4.68797956`)

### Scenario: Evaluate expression with parentheses

* *GIVEN* the user has entered an expression with parentheses
* *WHEN* an expression with parentheses is evaluated (e.g., `(5 + 3) * 2`)
* *THEN* the system SHALL respect operator precedence and grouping (e.g., `16`)

### Scenario: Evaluate expression with power operator

* *GIVEN* the user has entered an expression with exponentiation
* *WHEN* an expression uses the `^` operator (e.g., `2^(a+5)` where `a = 3`)
* *THEN* the system SHALL evaluate exponentiation with correct precedence (e.g., `256`)
* *AND* the `^` operator SHALL be right-associative (e.g., `2^3^2` equals `2^9` = `512`)

### Scenario: Evaluate invalid expression

* *GIVEN* the user has entered an invalid expression
* *WHEN* an invalid expression is evaluated (e.g., `5 + + 3`, `5 / 0`)
* *THEN* the system SHALL return an error with a descriptive message

### Scenario: Store variable from assignment

* *GIVEN* the user has entered an assignment expression
* *WHEN* an assignment expression is evaluated (e.g., `a = 5 + 3`)
* *THEN* the result SHALL be stored in the variable context under the given name
* *AND* the result value SHALL be returned (e.g., `8`)

### Scenario: Reference stored variable

* *GIVEN* a variable has been previously assigned (e.g., `a = 5`)
* *WHEN* an expression references a defined variable (e.g., `a * 2`)
* *THEN* the variable's value SHALL be substituted and the expression evaluated (e.g., `10`)

### Scenario: Reference undefined variable

* *GIVEN* no variable with a given name has been assigned
* *WHEN* an expression references an undefined variable
* *THEN* the system SHALL return an error indicating the variable is not defined

### Scenario: Evaluate all lines on change

* *GIVEN* the buffer contains multiple lines
* *WHEN* the buffer content changes
* *THEN* all lines SHALL be re-evaluated from top to bottom
* *AND* variable assignments SHALL be processed in order
* *AND* results for each line SHALL be updated

### Scenario: Variable dependency across lines

* *GIVEN* line 1 contains `a = 10`
* *WHEN* line 2 contains `a + 5`
* *THEN* line 2 SHALL evaluate to `15` using the value from line 1
