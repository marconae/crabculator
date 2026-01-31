# Feature: Factorial Operator

Postfix factorial operator (`!`) for non-negative integers.

## Background

The `!` operator computes the factorial of its operand. It binds tighter than all binary operators and is restricted to non-negative integers up to 170 (f64 overflow limit).

## Scenarios

### Scenario: Evaluate factorial operator

* *GIVEN* the user has entered an expression with the `!` postfix operator
* *WHEN* the expression is evaluated (e.g., `5!`, `0!`, `(3+2)!`)
* *THEN* the system SHALL compute the factorial of the operand (e.g., `120`, `1`, `120`)
* *AND* `!` SHALL have higher precedence than all binary operators
* *AND* the operand MUST be a non-negative integer; otherwise the system SHALL return an error
* *AND* operands greater than 170 SHALL return an error (overflow for f64)

### Scenario: Factorial combined with other operators

* *GIVEN* the user has entered an expression combining `!` with other operators
* *WHEN* the expression is evaluated (e.g., `3!^2`, `2*4!`, `3!+2!`)
* *THEN* `!` SHALL bind tighter than `^`, `*`, `/`, `+`, `-` (e.g., `36`, `48`, `8`)
