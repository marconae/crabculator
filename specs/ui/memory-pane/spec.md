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
