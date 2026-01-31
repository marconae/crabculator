# Feature: Built-in Functions

Catalog of built-in mathematical functions available in the expression evaluator.

## Background

The evaluator provides a comprehensive set of mathematical functions covering basic math, trigonometry, hyperbolic functions, logarithms, rounding, and utility operations.

## Scenarios

### Scenario: Evaluate expression with built-in functions

* *GIVEN* the user has entered an expression with a function call
* *WHEN* an expression uses built-in functions (e.g., `sqrt(9)`, `sin(pi/2)`, `sgn(-3)`, `cot(pi/4)`, `gcd(12,8)`, `ncr(5,2)`)
* *THEN* the system SHALL evaluate the function and return the result

### Scenario: Evaluate basic math function with literal

* *GIVEN* the user has entered a function call with a literal
* *WHEN* an expression uses a basic math function with a numeric literal (e.g., `sqrt(16)`)
* *THEN* the system SHALL evaluate the function and return the result (e.g., `4`)

### Scenario: Evaluate function with variable argument

* *GIVEN* a variable `x = 9` has been assigned
* *WHEN* an expression uses a function with a variable argument (e.g., `sqrt(x)`)
* *THEN* the system SHALL substitute the variable value and evaluate the function (e.g., `3`)

### Scenario: Evaluate trigonometric function with constant

* *GIVEN* the user has entered a trigonometric expression
* *WHEN* an expression uses a trigonometric function with pi (e.g., `sin(pi/2)`)
* *THEN* the system SHALL evaluate using the mathematical constant (e.g., `1`)

### Scenario: Function with invalid argument

* *GIVEN* the user has entered a function with an invalid argument
* *WHEN* an expression uses a function with an invalid argument (e.g., `sqrt(-1)`, `log(0)`)
* *THEN* the system SHALL return an appropriate error or NaN value

### Scenario: Basic math functions

* *GIVEN* the user calls a basic math function
* *WHEN* user calls `sqrt(x)`, `cbrt(x)`, `abs(x)`, or `pow(base, exp)`
* *THEN* the system SHALL return square root, cube root, absolute value, or power respectively

### Scenario: Trigonometric functions

* *GIVEN* the user calls a trigonometric function
* *WHEN* user calls `sin(x)`, `cos(x)`, `tan(x)`, `asin(x)`, `acos(x)`, `atan(x)`, or `atan2(y, x)`
* *THEN* the system SHALL return the trigonometric result (angles in radians)

### Scenario: Hyperbolic functions

* *GIVEN* the user calls a hyperbolic function
* *WHEN* user calls `sinh(x)`, `cosh(x)`, `tanh(x)`, `asinh(x)`, `acosh(x)`, or `atanh(x)`
* *THEN* the system SHALL return the hyperbolic function result

### Scenario: Logarithmic and exponential functions

* *GIVEN* the user calls a logarithmic or exponential function
* *WHEN* user calls `ln(x)`, `log(x)`, `log2(x)`, `log10(x)`, `exp(x)`, or `exp2(x)`
* *THEN* the system SHALL return natural log, base-10 log, base-2 log, base-10 log, e^x, or 2^x respectively

### Scenario: Rounding functions

* *GIVEN* the user calls a rounding function
* *WHEN* user calls `floor(x)`, `ceil(x)`, or `round(x)`
* *THEN* the system SHALL return the rounded value (floor down, ceil up, round to nearest)

### Scenario: Utility functions

* *GIVEN* the user calls a utility function
* *WHEN* user calls `min(a, b)`, `max(a, b)`, or `hypot(a, b)`
* *THEN* the system SHALL return minimum, maximum, or hypotenuse (sqrt(a^2 + b^2)) respectively
