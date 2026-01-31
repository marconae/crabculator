# Feature: Mathematical Constants

Pre-loaded mathematical constants available in the expression evaluator context.

## Background

The expression evaluator supports mathematical constants that are pre-loaded into the variable context at startup. Constants are defined in a central registry and can be referenced by name in any expression.

## Scenarios

### Scenario: Use pi constant

* *GIVEN* the user has entered an expression
* *WHEN* an expression references `pi`
* *THEN* the system SHALL substitute the value 3.141592653589793

### Scenario: Use e constant

* *GIVEN* the user has entered an expression
* *WHEN* an expression references `e`
* *THEN* the system SHALL substitute the value 2.718281828459045

### Scenario: Use tau constant

* *GIVEN* the user has entered an expression
* *WHEN* an expression references `tau`
* *THEN* the system SHALL substitute the value 6.283185307179586

### Scenario: Use phi constant

* *GIVEN* the user has entered an expression
* *WHEN* an expression references `phi`
* *THEN* the system SHALL substitute the value 1.618033988749895

### Scenario: Use sqrt2 constant

* *GIVEN* the user has entered an expression
* *WHEN* an expression references `sqrt2`
* *THEN* the system SHALL substitute the value 1.4142135623730951

### Scenario: Use sqrt3 constant

* *GIVEN* the user has entered an expression
* *WHEN* an expression references `sqrt3`
* *THEN* the system SHALL substitute the value 1.7320508075688772

### Scenario: Use ln2 constant

* *GIVEN* the user has entered an expression
* *WHEN* an expression references `ln2`
* *THEN* the system SHALL substitute the value 0.6931471805599453

### Scenario: Use ln10 constant

* *GIVEN* the user has entered an expression
* *WHEN* an expression references `ln10`
* *THEN* the system SHALL substitute the value 2.302585092994046
