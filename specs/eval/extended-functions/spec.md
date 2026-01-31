# Feature: Extended Functions

Additional mathematical functions for sign, truncation, angle conversion, reciprocal trigonometry, and combinatorics.

## Background

These functions extend the evaluator beyond the standard math library, adding sign analysis, fractional extraction, degree/radian conversion, reciprocal trig functions, GCD, and combinatorial computations.

## Scenarios

### Scenario: Evaluate sign function

* *GIVEN* the user has entered an expression with `sgn`
* *WHEN* the expression is evaluated (e.g., `sgn(-5)`, `sgn(0)`, `sgn(3.7)`)
* *THEN* the system SHALL return -1.0 for negative values, 0.0 for zero, and 1.0 for positive values

### Scenario: Evaluate truncate function

* *GIVEN* the user has entered an expression with `trunc`
* *WHEN* the expression is evaluated (e.g., `trunc(3.7)`, `trunc(-2.9)`)
* *THEN* the system SHALL return the integer part by rounding toward zero (e.g., `3`, `-2`)

### Scenario: Evaluate fractional part function

* *GIVEN* the user has entered an expression with `frac`
* *WHEN* the expression is evaluated (e.g., `frac(3.7)`, `frac(-2.9)`)
* *THEN* the system SHALL return the fractional part defined as `x - trunc(x)` (e.g., `0.7`, `-0.9`)

### Scenario: Evaluate degrees function

* *GIVEN* the user has entered an expression with `degrees`
* *WHEN* the expression is evaluated (e.g., `degrees(pi)`)
* *THEN* the system SHALL convert radians to degrees (e.g., `180`)

### Scenario: Evaluate radians function

* *GIVEN* the user has entered an expression with `radians`
* *WHEN* the expression is evaluated (e.g., `radians(180)`)
* *THEN* the system SHALL convert degrees to radians (e.g., the value of `pi`)

### Scenario: Evaluate cotangent function

* *GIVEN* the user has entered an expression with `cot`
* *WHEN* the expression is evaluated (e.g., `cot(pi/4)`)
* *THEN* the system SHALL return the cotangent, computed as `1.0 / tan(x)` (e.g., `1`)

### Scenario: Evaluate secant function

* *GIVEN* the user has entered an expression with `sec`
* *WHEN* the expression is evaluated (e.g., `sec(0)`)
* *THEN* the system SHALL return the secant, computed as `1.0 / cos(x)` (e.g., `1`)

### Scenario: Evaluate cosecant function

* *GIVEN* the user has entered an expression with `csc`
* *WHEN* the expression is evaluated (e.g., `csc(pi/2)`)
* *THEN* the system SHALL return the cosecant, computed as `1.0 / sin(x)` (e.g., `1`)

### Scenario: Evaluate greatest common divisor

* *GIVEN* the user has entered an expression with `gcd`
* *WHEN* the expression is evaluated with two integer arguments (e.g., `gcd(12, 8)`)
* *THEN* the system SHALL return the greatest common divisor (e.g., `4`)
* *AND* non-integer arguments SHALL be truncated toward zero before computing
* *AND* `gcd(0, 0)` SHALL return `0`

### Scenario: Evaluate binomial coefficient

* *GIVEN* the user has entered an expression with `ncr`
* *WHEN* the expression is evaluated with two arguments (e.g., `ncr(5, 2)`)
* *THEN* the system SHALL return the binomial coefficient n-choose-k (e.g., `10`)
* *AND* the system SHALL return an error when k < 0 or k > n
* *AND* both arguments SHALL be truncated to integers

### Scenario: Evaluate permutation coefficient

* *GIVEN* the user has entered an expression with `npr`
* *WHEN* the expression is evaluated with two arguments (e.g., `npr(5, 2)`)
* *THEN* the system SHALL return the permutation coefficient n-pick-k (e.g., `20`)
* *AND* the system SHALL return an error when k < 0 or k > n
* *AND* both arguments SHALL be truncated to integers
