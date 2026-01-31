# Feature: Base Literals

Hexadecimal, binary, and octal number input literals.

## Background

The tokenizer recognizes integer literals with `0x`, `0b`, and `0o` prefixes, parsing them as base-16, base-2, and base-8 respectively. Parsed values are stored as f64 and displayed in decimal.

## Scenarios

### Scenario: Evaluate hexadecimal literal

* *GIVEN* the user has entered an expression with a hexadecimal literal
* *WHEN* the expression contains a `0x` or `0X` prefix followed by hex digits (e.g., `0xff`, `0xFF`, `0x1A`)
* *THEN* the system SHALL parse the value as base-16 and evaluate it as a decimal number (e.g., `255`, `255`, `26`)

### Scenario: Evaluate binary literal

* *GIVEN* the user has entered an expression with a binary literal
* *WHEN* the expression contains a `0b` or `0B` prefix followed by binary digits (e.g., `0b1010`, `0b11111111`)
* *THEN* the system SHALL parse the value as base-2 and evaluate it as a decimal number (e.g., `10`, `255`)

### Scenario: Evaluate octal literal

* *GIVEN* the user has entered an expression with an octal literal
* *WHEN* the expression contains a `0o` or `0O` prefix followed by octal digits (e.g., `0o77`, `0o10`)
* *THEN* the system SHALL parse the value as base-8 and evaluate it as a decimal number (e.g., `63`, `8`)

### Scenario: Use base literals in expressions

* *GIVEN* the user has entered an expression mixing base literals with operators
* *WHEN* the expression is evaluated (e.g., `0xff + 1`, `0b1010 * 2`, `0o10 + 0x10`)
* *THEN* the system SHALL evaluate correctly using the decimal equivalents (e.g., `256`, `20`, `24`)

### Scenario: Invalid base literal digits

* *GIVEN* the user has entered a base literal with invalid digits
* *WHEN* the expression contains invalid digits for the base (e.g., `0b123`, `0o89`, `0xGH`)
* *THEN* the system SHALL return a parse error indicating invalid digits for the base
