# Feature: Implicit Multiplication

Conservative implicit multiplication between adjacent tokens.

## Background

The parser inserts an implicit `*` between adjacent tokens in four specific cases: number before identifier, number before `(`, `)` before `(`, and `)` before identifier. Function calls are not affected.

## Scenarios

### Scenario: Implicit multiplication between number and identifier

* *GIVEN* the user has entered an expression with a number directly before an identifier
* *WHEN* the expression is evaluated (e.g., `2pi`, `3e`, `5x` where x is a defined variable)
* *THEN* the system SHALL treat this as multiplication (e.g., `2*pi`, `3*e`, `5*x`)

### Scenario: Implicit multiplication between number and opening parenthesis

* *GIVEN* the user has entered an expression with a number directly before an opening parenthesis
* *WHEN* the expression is evaluated (e.g., `3(4+5)`, `2(1+1)`)
* *THEN* the system SHALL treat this as multiplication (e.g., `3*(4+5) = 27`, `2*(1+1) = 4`)

### Scenario: Implicit multiplication between closing and opening parenthesis

* *GIVEN* the user has entered an expression with a closing parenthesis directly before an opening parenthesis
* *WHEN* the expression is evaluated (e.g., `(2+3)(4+5)`)
* *THEN* the system SHALL treat this as multiplication (e.g., `(2+3)*(4+5) = 45`)

### Scenario: Implicit multiplication does not affect function calls

* *GIVEN* the user has entered an expression with a function call
* *WHEN* the expression contains an identifier followed by parentheses (e.g., `sqrt(9)`, `sin(pi)`)
* *THEN* the system SHALL treat this as a function call, NOT implicit multiplication
* *AND* `2sqrt(9)` SHALL evaluate as `2 * sqrt(9) = 6` (number before function call)

### Scenario: Implicit multiplication precedence

* *GIVEN* the user has entered an expression with implicit multiplication
* *WHEN* implicit multiplication is combined with explicit operators
* *THEN* implicit multiplication SHALL have the same precedence as explicit `*`
* *AND* `2pi^2` SHALL evaluate as `2 * (pi^2)` since `^` binds tighter than `*`

### Scenario: Implicit multiplication between closing parenthesis and identifier

* *GIVEN* the user has entered an expression with a closing parenthesis directly before an identifier
* *WHEN* the expression is evaluated (e.g., `(2+3)pi`, `(4)x` where x is defined)
* *THEN* the system SHALL treat this as multiplication (e.g., `(2+3)*pi`, `(4)*x`)
