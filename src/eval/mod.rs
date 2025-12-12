//! Expression evaluation module for Crabculator.
//!
//! This module provides functionality for parsing and evaluating mathematical
//! expressions, managing variable context, and producing results or errors.

pub mod context;
pub mod error;
pub mod parser;

use evalexpr::Value;

pub use context::EvalContext;
pub use error::{ErrorSpan, EvalError};
pub use parser::{ParsedLine, parse_line};

/// Result of evaluating a single line.
#[derive(Debug, Clone, PartialEq)]
pub enum LineResult {
    /// A successful evaluation with a value.
    Value(Value),
    /// An assignment that stored a value in a variable.
    Assignment {
        /// The variable name that was assigned.
        name: String,
        /// The value that was assigned.
        value: Value,
    },
    /// An empty line (no result).
    Empty,
    /// An evaluation error.
    Error(EvalError),
}

/// Evaluates a single expression string using the given context.
///
/// # Arguments
/// * `expression` - The expression to evaluate
/// * `context` - The evaluation context with variable bindings
///
/// # Returns
/// The result value or an evaluation error.
///
/// # Errors
/// Returns an `EvalError` if the expression is invalid, contains undefined
/// variables, or results in a runtime error (e.g., division by zero).
pub fn evaluate_expression(
    expression: &str,
    context: &mut EvalContext,
) -> Result<Value, EvalError> {
    evalexpr::eval_with_context_mut(expression, context.inner_mut())
        .map_err(|e| convert_evalexpr_error(&e, expression))
}

/// Evaluates a single line and returns the result.
///
/// This function parses the line, evaluates it if necessary, and updates
/// the context for assignments.
///
/// # Arguments
/// * `line` - The line to evaluate
/// * `context` - The evaluation context for variable bindings
///
/// # Returns
/// A `LineResult` indicating the outcome of evaluation.
pub fn evaluate_line(line: &str, context: &mut EvalContext) -> LineResult {
    match parse_line(line) {
        ParsedLine::Empty => LineResult::Empty,
        ParsedLine::Expression(expr) => match evaluate_expression(&expr, context) {
            Ok(value) => LineResult::Value(value),
            Err(e) => LineResult::Error(e),
        },
        ParsedLine::Assignment { name, expression } => {
            match evaluate_expression(&expression, context) {
                Ok(value) => {
                    context.set_variable(&name, value.clone());
                    LineResult::Assignment { name, value }
                }
                Err(e) => LineResult::Error(e),
            }
        }
    }
}

/// Evaluates all lines in order, returning results for each line.
///
/// Lines are evaluated from top to bottom. Variable assignments from earlier
/// lines are available in later lines.
///
/// Note: This creates a new context for each evaluation. To persist variables
/// across evaluations, use `evaluate_all_lines_with_context` instead.
///
/// # Arguments
/// * `lines` - An iterator of lines to evaluate
///
/// # Returns
/// A vector of `LineResult` values, one per input line.
pub fn evaluate_all_lines<'a>(lines: impl IntoIterator<Item = &'a str>) -> Vec<LineResult> {
    let mut context = EvalContext::new();
    evaluate_all_lines_with_context(lines, &mut context)
}

/// Evaluates all lines in order using the provided context.
///
/// Lines are evaluated from top to bottom. Variable assignments from earlier
/// lines are available in later lines. Variables are stored in the provided
/// context, allowing them to be persisted across evaluations.
///
/// # Arguments
/// * `lines` - An iterator of lines to evaluate
/// * `context` - The evaluation context for variable storage
///
/// # Returns
/// A vector of `LineResult` values, one per input line.
pub fn evaluate_all_lines_with_context<'a>(
    lines: impl IntoIterator<Item = &'a str>,
    context: &mut EvalContext,
) -> Vec<LineResult> {
    lines
        .into_iter()
        .map(|line| evaluate_line(line, context))
        .collect()
}

/// Converts an evalexpr error into our `EvalError` type.
fn convert_evalexpr_error(error: &evalexpr::EvalexprError, _expression: &str) -> EvalError {
    // Extract the error message
    let message = format!("{error}");

    // Try to extract span information if available
    // Note: evalexpr doesn't always provide position info, so we may not have a span
    // For now, we return the error without span info
    // TODO: Parse error messages to extract position hints

    EvalError::new(message)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Basic expression evaluation tests
    #[test]
    fn test_evaluate_simple_addition() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("5 + 3", &mut context);
        assert_eq!(result, Ok(Value::Int(8)));
    }

    #[test]
    fn test_evaluate_simple_subtraction() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("10 - 4", &mut context);
        assert_eq!(result, Ok(Value::Int(6)));
    }

    #[test]
    fn test_evaluate_simple_multiplication() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("6 * 7", &mut context);
        assert_eq!(result, Ok(Value::Int(42)));
    }

    #[test]
    fn test_evaluate_simple_division() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("20 / 4", &mut context);
        assert_eq!(result, Ok(Value::Int(5)));
    }

    #[test]
    fn test_evaluate_modulo() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("17 % 5", &mut context);
        assert_eq!(result, Ok(Value::Int(2)));
    }

    // Operator precedence tests
    #[test]
    fn test_evaluate_operator_precedence() {
        let mut context = EvalContext::new();
        // 5 + 3 * 2 = 5 + 6 = 11 (multiplication before addition)
        let result = evaluate_expression("5 + 3 * 2", &mut context);
        assert_eq!(result, Ok(Value::Int(11)));
    }

    #[test]
    fn test_evaluate_parentheses() {
        let mut context = EvalContext::new();
        // (5 + 3) * 2 = 8 * 2 = 16
        let result = evaluate_expression("(5 + 3) * 2", &mut context);
        assert_eq!(result, Ok(Value::Int(16)));
    }

    #[test]
    fn test_evaluate_nested_parentheses() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("((2 + 3) * (4 + 1))", &mut context);
        assert_eq!(result, Ok(Value::Int(25)));
    }

    // Built-in function tests
    #[test]
    fn test_evaluate_sqrt() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("math::sqrt(16)", &mut context);
        assert_eq!(result, Ok(Value::Float(4.0)));
    }

    #[test]
    fn test_evaluate_floor() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("floor(3.7)", &mut context);
        assert_eq!(result, Ok(Value::Float(3.0)));
    }

    #[test]
    fn test_evaluate_ceil() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("ceil(3.2)", &mut context);
        assert_eq!(result, Ok(Value::Float(4.0)));
    }

    #[test]
    fn test_evaluate_abs() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("math::abs(-5)", &mut context);
        assert_eq!(result, Ok(Value::Int(5)));
    }

    // Float tests
    #[test]
    fn test_evaluate_float_expression() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("2.5 * 4.0", &mut context);
        assert_eq!(result, Ok(Value::Float(10.0)));
    }

    #[test]
    fn test_evaluate_integer_division_to_float() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("5 / 2", &mut context);
        assert_eq!(result, Ok(Value::Int(2))); // Integer division
    }

    // Error tests
    #[test]
    fn test_evaluate_syntax_error() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("5 + + 3", &mut context);
        assert!(result.is_err());
    }

    #[test]
    fn test_evaluate_undefined_variable() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("undefined_var + 1", &mut context);
        assert!(result.is_err());
    }

    #[test]
    fn test_evaluate_unclosed_parenthesis() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("(5 + 3", &mut context);
        assert!(result.is_err());
    }

    // Variable context tests
    #[test]
    fn test_evaluate_with_predefined_variable() {
        let mut context = EvalContext::new();
        context.set_variable("x", Value::Int(10));
        let result = evaluate_expression("x + 5", &mut context);
        assert_eq!(result, Ok(Value::Int(15)));
    }

    #[test]
    fn test_evaluate_with_multiple_variables() {
        let mut context = EvalContext::new();
        context.set_variable("a", Value::Int(3));
        context.set_variable("b", Value::Int(4));
        let result = evaluate_expression("a * b", &mut context);
        assert_eq!(result, Ok(Value::Int(12)));
    }

    // evaluate_line tests
    #[test]
    fn test_evaluate_line_empty() {
        let mut context = EvalContext::new();
        let result = evaluate_line("", &mut context);
        assert_eq!(result, LineResult::Empty);
    }

    #[test]
    fn test_evaluate_line_whitespace() {
        let mut context = EvalContext::new();
        let result = evaluate_line("   ", &mut context);
        assert_eq!(result, LineResult::Empty);
    }

    #[test]
    fn test_evaluate_line_expression() {
        let mut context = EvalContext::new();
        let result = evaluate_line("5 + 3", &mut context);
        assert_eq!(result, LineResult::Value(Value::Int(8)));
    }

    #[test]
    fn test_evaluate_line_assignment() {
        let mut context = EvalContext::new();
        let result = evaluate_line("a = 5 + 3", &mut context);
        assert_eq!(
            result,
            LineResult::Assignment {
                name: "a".to_string(),
                value: Value::Int(8),
            }
        );
        // Verify the variable was stored
        assert_eq!(context.get_variable("a"), Some(&Value::Int(8)));
    }

    #[test]
    fn test_evaluate_line_error() {
        let mut context = EvalContext::new();
        let result = evaluate_line("5 / undefined", &mut context);
        assert!(matches!(result, LineResult::Error(_)));
    }

    // evaluate_all_lines tests
    #[test]
    fn test_evaluate_all_lines_simple() {
        let lines = ["5 + 3", "10 - 2"];
        let results = evaluate_all_lines(lines);

        assert_eq!(results.len(), 2);
        assert_eq!(results[0], LineResult::Value(Value::Int(8)));
        assert_eq!(results[1], LineResult::Value(Value::Int(8)));
    }

    #[test]
    fn test_evaluate_all_lines_with_assignment() {
        let lines = ["a = 10", "a + 5"];
        let results = evaluate_all_lines(lines);

        assert_eq!(results.len(), 2);
        assert_eq!(
            results[0],
            LineResult::Assignment {
                name: "a".to_string(),
                value: Value::Int(10),
            }
        );
        assert_eq!(results[1], LineResult::Value(Value::Int(15)));
    }

    #[test]
    fn test_evaluate_all_lines_variable_dependency() {
        let lines = ["a = 10", "b = a * 2", "b + a"];
        let results = evaluate_all_lines(lines);

        assert_eq!(results.len(), 3);
        assert_eq!(
            results[0],
            LineResult::Assignment {
                name: "a".to_string(),
                value: Value::Int(10),
            }
        );
        assert_eq!(
            results[1],
            LineResult::Assignment {
                name: "b".to_string(),
                value: Value::Int(20),
            }
        );
        assert_eq!(results[2], LineResult::Value(Value::Int(30)));
    }

    #[test]
    fn test_evaluate_all_lines_with_empty() {
        let lines = ["5 + 3", "", "10 - 2"];
        let results = evaluate_all_lines(lines);

        assert_eq!(results.len(), 3);
        assert_eq!(results[0], LineResult::Value(Value::Int(8)));
        assert_eq!(results[1], LineResult::Empty);
        assert_eq!(results[2], LineResult::Value(Value::Int(8)));
    }

    #[test]
    fn test_evaluate_all_lines_error_doesnt_stop_evaluation() {
        let lines = ["5 + 3", "undefined_var", "10 - 2"];
        let results = evaluate_all_lines(lines);

        assert_eq!(results.len(), 3);
        assert_eq!(results[0], LineResult::Value(Value::Int(8)));
        assert!(matches!(results[1], LineResult::Error(_)));
        assert_eq!(results[2], LineResult::Value(Value::Int(8)));
    }

    // Complex expression tests
    #[test]
    fn test_evaluate_complex_expression() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("(10 + 5) * 2 - 8 / 4", &mut context);
        // (10 + 5) * 2 - 8 / 4 = 15 * 2 - 2 = 30 - 2 = 28
        assert_eq!(result, Ok(Value::Int(28)));
    }

    #[test]
    fn test_evaluate_negative_numbers() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("-5 + 3", &mut context);
        assert_eq!(result, Ok(Value::Int(-2)));
    }

    #[test]
    fn test_evaluate_single_number() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("42", &mut context);
        assert_eq!(result, Ok(Value::Int(42)));
    }
}
