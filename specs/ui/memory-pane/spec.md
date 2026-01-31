# Feature: Memory Pane

Displays evaluation results in a dedicated panel alongside the expression editor, with intelligent formatting for readability.

## Background

The memory pane renders on either side of the calculator pane and shows computed values aligned with their corresponding input expressions. Results are formatted for display within the constrained pane width.

## Scenarios

### Scenario: Display numeric result with truncation

* *GIVEN* an expression evaluates to a number
* *WHEN* the formatted result exceeds 12 characters
* *THEN* only the first 9 characters SHALL be displayed followed by "..."
* *AND* negative signs and decimal points SHALL count toward the character limit

### Scenario: Full precision preserved for calculations

* *GIVEN* a variable is assigned a value that displays truncated in the memory pane
* *WHEN* the variable is referenced in a subsequent expression
* *THEN* the full precision value SHALL be used for the calculation
* *AND* truncation SHALL only affect the visual display

### Scenario: Short results displayed in full

* *GIVEN* an expression evaluates to a number
* *WHEN* the formatted result is 12 characters or fewer
* *THEN* the full result SHALL be displayed without truncation

### Scenario: Display exact constant match

* *GIVEN* an expression evaluates to a value
* *WHEN* the result exactly equals a known mathematical constant
* *THEN* the memory pane SHALL display `(truncated_value) name`
* *AND* for example pi SHALL display as `(3.14159...) pi`

### Scenario: Display integer multiple of constant

* *GIVEN* an expression evaluates to a value
* *WHEN* the result equals an integer multiple (2-4) of a known constant
* *THEN* the memory pane SHALL display `(truncated_value) Nc`
* *AND* for example 2*pi SHALL display as `(6.28318...) 2pi`

### Scenario: Display simple fraction of constant

* *GIVEN* an expression evaluates to a value
* *WHEN* the result equals a known constant divided by an integer (2-4)
* *THEN* the memory pane SHALL display `(truncated_value) c/N`
* *AND* for example pi/2 SHALL display as `(1.5707...) pi/2`

### Scenario: No constant annotation for non-matching values

* *GIVEN* an expression evaluates to a value
* *WHEN* the result does not match any known constant, multiple, or fraction
* *THEN* the memory pane SHALL display the numeric value without annotation

### Scenario: Constant recognition uses epsilon comparison

* *GIVEN* an expression evaluates to a value
* *WHEN* checking for constant matches
* *THEN* the comparison SHALL use floating-point epsilon tolerance (1e-10)
* *AND* values within the tolerance SHALL be treated as matching

### Scenario: Constant recognition precedence

* *GIVEN* an expression evaluates to a value that matches multiple constants
* *WHEN* displaying the result
* *THEN* the exact constant match SHALL take precedence over multiples or fractions
* *AND* the first matching constant in definition order SHALL be used
