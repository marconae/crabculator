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

    // ============================================================
    // Mathematical function alias tests (short names without math:: prefix)
    // ============================================================

    // Basic math functions
    #[test]
    fn test_sqrt_short_alias() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("sqrt(16)", &mut context);
        assert_eq!(result, Ok(Value::Float(4.0)));
    }

    #[test]
    fn test_cbrt_short_alias() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("cbrt(27)", &mut context);
        assert_eq!(result, Ok(Value::Float(3.0)));
    }

    #[test]
    fn test_abs_short_alias_int() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("abs(-5)", &mut context);
        assert_eq!(result, Ok(Value::Int(5)));
    }

    #[test]
    fn test_abs_short_alias_float() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("abs(-3.5)", &mut context);
        assert_eq!(result, Ok(Value::Float(3.5)));
    }

    #[test]
    fn test_pow_short_alias() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("pow(2, 8)", &mut context);
        assert_eq!(result, Ok(Value::Float(256.0)));
    }

    // Trigonometric functions
    #[test]
    fn test_sin_short_alias() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("sin(pi/2)", &mut context);
        if let Ok(Value::Float(v)) = result {
            assert!((v - 1.0).abs() < 1e-10, "sin(pi/2) should be 1, got {}", v);
        } else {
            panic!("Expected Float result, got {:?}", result);
        }
    }

    #[test]
    fn test_cos_short_alias() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("cos(0)", &mut context);
        assert_eq!(result, Ok(Value::Float(1.0)));
    }

    #[test]
    fn test_tan_short_alias() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("tan(0)", &mut context);
        assert_eq!(result, Ok(Value::Float(0.0)));
    }

    #[test]
    fn test_asin_short_alias() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("asin(1)", &mut context);
        if let Ok(Value::Float(v)) = result {
            assert!(
                (v - std::f64::consts::FRAC_PI_2).abs() < 1e-10,
                "asin(1) should be pi/2, got {}",
                v
            );
        } else {
            panic!("Expected Float result, got {:?}", result);
        }
    }

    #[test]
    fn test_acos_short_alias() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("acos(1)", &mut context);
        assert_eq!(result, Ok(Value::Float(0.0)));
    }

    #[test]
    fn test_atan_short_alias() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("atan(0)", &mut context);
        assert_eq!(result, Ok(Value::Float(0.0)));
    }

    #[test]
    fn test_atan2_short_alias() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("atan2(1, 1)", &mut context);
        if let Ok(Value::Float(v)) = result {
            assert!(
                (v - std::f64::consts::FRAC_PI_4).abs() < 1e-10,
                "atan2(1, 1) should be pi/4, got {}",
                v
            );
        } else {
            panic!("Expected Float result, got {:?}", result);
        }
    }

    // Hyperbolic functions
    #[test]
    fn test_sinh_short_alias() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("sinh(0)", &mut context);
        assert_eq!(result, Ok(Value::Float(0.0)));
    }

    #[test]
    fn test_cosh_short_alias() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("cosh(0)", &mut context);
        assert_eq!(result, Ok(Value::Float(1.0)));
    }

    #[test]
    fn test_tanh_short_alias() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("tanh(0)", &mut context);
        assert_eq!(result, Ok(Value::Float(0.0)));
    }

    #[test]
    fn test_asinh_short_alias() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("asinh(0)", &mut context);
        assert_eq!(result, Ok(Value::Float(0.0)));
    }

    #[test]
    fn test_acosh_short_alias() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("acosh(1)", &mut context);
        assert_eq!(result, Ok(Value::Float(0.0)));
    }

    #[test]
    fn test_atanh_short_alias() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("atanh(0)", &mut context);
        assert_eq!(result, Ok(Value::Float(0.0)));
    }

    // Logarithmic and exponential functions
    #[test]
    fn test_ln_short_alias() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("ln(e)", &mut context);
        if let Ok(Value::Float(v)) = result {
            assert!((v - 1.0).abs() < 1e-10, "ln(e) should be 1, got {}", v);
        } else {
            panic!("Expected Float result, got {:?}", result);
        }
    }

    #[test]
    fn test_log_short_alias() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("log(10)", &mut context);
        if let Ok(Value::Float(v)) = result {
            assert!((v - 1.0).abs() < 1e-10, "log(10) should be 1, got {}", v);
        } else {
            panic!("Expected Float result, got {:?}", result);
        }
    }

    #[test]
    fn test_log2_short_alias() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("log2(8)", &mut context);
        if let Ok(Value::Float(v)) = result {
            assert!((v - 3.0).abs() < 1e-10, "log2(8) should be 3, got {}", v);
        } else {
            panic!("Expected Float result, got {:?}", result);
        }
    }

    #[test]
    fn test_log10_short_alias() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("log10(100)", &mut context);
        if let Ok(Value::Float(v)) = result {
            assert!((v - 2.0).abs() < 1e-10, "log10(100) should be 2, got {}", v);
        } else {
            panic!("Expected Float result, got {:?}", result);
        }
    }

    #[test]
    fn test_exp_short_alias() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("exp(1)", &mut context);
        if let Ok(Value::Float(v)) = result {
            assert!(
                (v - std::f64::consts::E).abs() < 1e-10,
                "exp(1) should be e, got {}",
                v
            );
        } else {
            panic!("Expected Float result, got {:?}", result);
        }
    }

    #[test]
    fn test_exp2_short_alias() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("exp2(3)", &mut context);
        assert_eq!(result, Ok(Value::Float(8.0)));
    }

    // Rounding functions (these already work without prefix in evalexpr)
    #[test]
    fn test_floor_short_alias() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("floor(3.9)", &mut context);
        assert_eq!(result, Ok(Value::Float(3.0)));
    }

    #[test]
    fn test_ceil_short_alias() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("ceil(3.1)", &mut context);
        assert_eq!(result, Ok(Value::Float(4.0)));
    }

    #[test]
    fn test_round_short_alias() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("round(3.5)", &mut context);
        // Note: round(3.5) in evalexpr rounds to 4.0 (round half up)
        assert_eq!(result, Ok(Value::Float(4.0)));
    }

    // Utility functions
    #[test]
    fn test_min_short_alias() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("min(3, 7)", &mut context);
        assert_eq!(result, Ok(Value::Int(3)));
    }

    #[test]
    fn test_max_short_alias() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("max(3, 7)", &mut context);
        assert_eq!(result, Ok(Value::Int(7)));
    }

    #[test]
    fn test_hypot_short_alias() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("hypot(3, 4)", &mut context);
        assert_eq!(result, Ok(Value::Float(5.0)));
    }

    // ============================================================
    // Tests for functions with variable arguments
    // ============================================================

    #[test]
    fn test_sqrt_with_variable() {
        let mut context = EvalContext::new();
        context.set_variable("x", Value::Int(9));
        let result = evaluate_expression("sqrt(x)", &mut context);
        assert_eq!(result, Ok(Value::Float(3.0)));
    }

    #[test]
    fn test_sin_with_pi_variable() {
        let mut context = EvalContext::new();
        // pi is already predefined
        let result = evaluate_expression("sin(pi)", &mut context);
        if let Ok(Value::Float(v)) = result {
            assert!(v.abs() < 1e-10, "sin(pi) should be ~0, got {}", v);
        } else {
            panic!("Expected Float result, got {:?}", result);
        }
    }

    #[test]
    fn test_pow_with_variables() {
        let mut context = EvalContext::new();
        context.set_variable("base", Value::Int(2));
        context.set_variable("exp", Value::Int(10));
        let result = evaluate_expression("pow(base, exp)", &mut context);
        assert_eq!(result, Ok(Value::Float(1024.0)));
    }

    // ============================================================
    // Tests for error cases
    // ============================================================

    #[test]
    fn test_sqrt_negative_returns_nan() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("sqrt(-1)", &mut context);
        if let Ok(Value::Float(v)) = result {
            assert!(v.is_nan(), "sqrt(-1) should return NaN, got {}", v);
        } else {
            // Some implementations might return an error instead
            // That's also acceptable per the spec
            assert!(result.is_err() || matches!(result, Ok(Value::Float(f)) if f.is_nan()));
        }
    }

    #[test]
    fn test_log_zero_returns_neg_infinity() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("log(0)", &mut context);
        if let Ok(Value::Float(v)) = result {
            assert!(
                v.is_infinite() && v < 0.0,
                "log(0) should return -infinity, got {}",
                v
            );
        } else {
            // Error is also acceptable
            assert!(result.is_err() || matches!(result, Ok(Value::Float(f)) if f.is_infinite()));
        }
    }

    #[test]
    fn test_acosh_less_than_one_returns_nan() {
        let mut context = EvalContext::new();
        let result = evaluate_expression("acosh(0.5)", &mut context);
        if let Ok(Value::Float(v)) = result {
            assert!(v.is_nan(), "acosh(0.5) should return NaN, got {}", v);
        } else {
            // Error is also acceptable
            assert!(result.is_err() || matches!(result, Ok(Value::Float(f)) if f.is_nan()));
        }
    }
}
