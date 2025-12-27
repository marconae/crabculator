//! Expression evaluator for the custom math expression parser.
//!
//! This module provides the `evaluate` function that evaluates an AST
//! against a variable context, returning a numeric result or an error.

use std::collections::HashMap;
use std::hash::BuildHasher;

use crate::eval::ast::{BinaryOp, Expr};
use crate::eval::error::EvalError;

/// Evaluates an expression AST with the given variable bindings.
///
/// # Arguments
/// * `expr` - The expression AST to evaluate
/// * `variables` - A map of variable names to their f64 values
///
/// # Returns
/// The numeric result of the expression, or an error if evaluation fails.
///
/// # Errors
/// Returns an `EvalError` if:
/// - A variable is referenced but not defined in `variables`
/// - A function is called that doesn't exist
/// - A function is called with the wrong number of arguments
pub fn evaluate<S: BuildHasher>(
    expr: &Expr,
    variables: &HashMap<String, f64, S>,
) -> Result<f64, EvalError> {
    match expr {
        Expr::Number(n) => Ok(*n),
        Expr::Variable(name) => variables
            .get(name)
            .copied()
            .ok_or_else(|| EvalError::undefined_variable(name)),
        Expr::BinaryOp { left, op, right } => {
            let left_val = evaluate(left, variables)?;
            let right_val = evaluate(right, variables)?;
            Ok(apply_binary_op(left_val, op, right_val))
        }
        Expr::UnaryMinus(inner) => {
            let val = evaluate(inner, variables)?;
            Ok(-val)
        }
        Expr::FunctionCall { name, args } => {
            let evaluated_args: Result<Vec<f64>, EvalError> =
                args.iter().map(|arg| evaluate(arg, variables)).collect();
            call_builtin(name, &evaluated_args?)
        }
    }
}

/// Applies a binary operator to two f64 operands.
fn apply_binary_op(left: f64, op: &BinaryOp, right: f64) -> f64 {
    match op {
        BinaryOp::Add => left + right,
        BinaryOp::Sub => left - right,
        BinaryOp::Mul => left * right,
        BinaryOp::Div => left / right,
        BinaryOp::Mod => left % right,
        BinaryOp::Pow => left.powf(right),
    }
}

/// Calls a built-in mathematical function.
///
/// # Arguments
/// * `name` - The function name
/// * `args` - The evaluated arguments
///
/// # Returns
/// The function result, or an error if the function is unknown or has wrong arity.
fn call_builtin(name: &str, args: &[f64]) -> Result<f64, EvalError> {
    let arg_count = args.len();

    match name {
        // Basic functions (1 arg)
        "sqrt" => expect_args(name, 1, arg_count).map(|()| args[0].sqrt()),
        "cbrt" => expect_args(name, 1, arg_count).map(|()| args[0].cbrt()),
        "abs" => expect_args(name, 1, arg_count).map(|()| args[0].abs()),

        // Basic functions (2 args)
        "pow" => expect_args(name, 2, arg_count).map(|()| args[0].powf(args[1])),

        // Trigonometric functions (1 arg)
        "sin" => expect_args(name, 1, arg_count).map(|()| args[0].sin()),
        "cos" => expect_args(name, 1, arg_count).map(|()| args[0].cos()),
        "tan" => expect_args(name, 1, arg_count).map(|()| args[0].tan()),
        "asin" => expect_args(name, 1, arg_count).map(|()| args[0].asin()),
        "acos" => expect_args(name, 1, arg_count).map(|()| args[0].acos()),
        "atan" => expect_args(name, 1, arg_count).map(|()| args[0].atan()),

        // Trigonometric functions (2 args)
        "atan2" => expect_args(name, 2, arg_count).map(|()| args[0].atan2(args[1])),

        // Hyperbolic functions (1 arg)
        "sinh" => expect_args(name, 1, arg_count).map(|()| args[0].sinh()),
        "cosh" => expect_args(name, 1, arg_count).map(|()| args[0].cosh()),
        "tanh" => expect_args(name, 1, arg_count).map(|()| args[0].tanh()),
        "asinh" => expect_args(name, 1, arg_count).map(|()| args[0].asinh()),
        "acosh" => expect_args(name, 1, arg_count).map(|()| args[0].acosh()),
        "atanh" => expect_args(name, 1, arg_count).map(|()| args[0].atanh()),

        // Logarithmic functions (1 arg)
        "ln" => expect_args(name, 1, arg_count).map(|()| args[0].ln()),
        "log2" => expect_args(name, 1, arg_count).map(|()| args[0].log2()),
        "log10" => expect_args(name, 1, arg_count).map(|()| args[0].log10()),
        "exp" => expect_args(name, 1, arg_count).map(|()| args[0].exp()),
        "exp2" => expect_args(name, 1, arg_count).map(|()| args[0].exp2()),

        // Logarithmic functions (2 args)
        "log" => expect_args(name, 2, arg_count).map(|()| args[0].log(args[1])),

        // Rounding functions (1 arg)
        "floor" => expect_args(name, 1, arg_count).map(|()| args[0].floor()),
        "ceil" => expect_args(name, 1, arg_count).map(|()| args[0].ceil()),
        "round" => expect_args(name, 1, arg_count).map(|()| args[0].round()),

        // Utility functions (2 args)
        "min" => expect_args(name, 2, arg_count).map(|()| args[0].min(args[1])),
        "max" => expect_args(name, 2, arg_count).map(|()| args[0].max(args[1])),
        "hypot" => expect_args(name, 2, arg_count).map(|()| args[0].hypot(args[1])),

        _ => Err(EvalError::unknown_function(name)),
    }
}

/// Helper to check argument count.
fn expect_args(name: &str, expected: usize, got: usize) -> Result<(), EvalError> {
    if expected == got {
        Ok(())
    } else {
        Err(EvalError::invalid_argument_count(name, expected, got))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::{E, FRAC_PI_2, FRAC_PI_4, PI};

    // Helper to create an empty variable context
    fn empty_vars() -> HashMap<String, f64> {
        HashMap::new()
    }

    // Helper to create a variable context with given bindings
    fn vars(bindings: &[(&str, f64)]) -> HashMap<String, f64> {
        bindings
            .iter()
            .map(|(k, v)| ((*k).to_string(), *v))
            .collect()
    }

    // ============================================================
    // Tests for number literals (3.1.1)
    // ============================================================

    #[test]
    fn test_evaluate_integer_literal() {
        let expr = Expr::Number(42.0);
        let result = evaluate(&expr, &empty_vars());
        assert_eq!(result, Ok(42.0));
    }

    #[test]
    fn test_evaluate_float_literal() {
        let expr = Expr::Number(2.5);
        let result = evaluate(&expr, &empty_vars());
        assert_eq!(result, Ok(2.5));
    }

    #[test]
    fn test_evaluate_zero() {
        let expr = Expr::Number(0.0);
        let result = evaluate(&expr, &empty_vars());
        assert_eq!(result, Ok(0.0));
    }

    #[test]
    fn test_evaluate_negative_literal() {
        let expr = Expr::Number(-5.5);
        let result = evaluate(&expr, &empty_vars());
        assert_eq!(result, Ok(-5.5));
    }

    // ============================================================
    // Tests for variable references (3.1.2)
    // ============================================================

    #[test]
    fn test_evaluate_variable_found() {
        let expr = Expr::Variable("x".to_string());
        let result = evaluate(&expr, &vars(&[("x", 10.0)]));
        assert_eq!(result, Ok(10.0));
    }

    #[test]
    fn test_evaluate_variable_with_special_value() {
        let expr = Expr::Variable("pi".to_string());
        let result = evaluate(&expr, &vars(&[("pi", PI)]));
        assert_eq!(result, Ok(PI));
    }

    #[test]
    fn test_evaluate_undefined_variable() {
        let expr = Expr::Variable("undefined".to_string());
        let result = evaluate(&expr, &empty_vars());
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().message(),
            "undefined variable 'undefined'"
        );
    }

    #[test]
    fn test_evaluate_variable_case_sensitive() {
        let expr = Expr::Variable("X".to_string());
        let result = evaluate(&expr, &vars(&[("x", 10.0)]));
        // X is not defined, only x is
        assert!(result.is_err());
    }

    // ============================================================
    // Tests for binary operations (3.1.3)
    // ============================================================

    #[test]
    fn test_evaluate_addition() {
        let expr = Expr::BinaryOp {
            left: Box::new(Expr::Number(5.0)),
            op: BinaryOp::Add,
            right: Box::new(Expr::Number(3.0)),
        };
        let result = evaluate(&expr, &empty_vars());
        assert_eq!(result, Ok(8.0));
    }

    #[test]
    fn test_evaluate_subtraction() {
        let expr = Expr::BinaryOp {
            left: Box::new(Expr::Number(10.0)),
            op: BinaryOp::Sub,
            right: Box::new(Expr::Number(4.0)),
        };
        let result = evaluate(&expr, &empty_vars());
        assert_eq!(result, Ok(6.0));
    }

    #[test]
    fn test_evaluate_multiplication() {
        let expr = Expr::BinaryOp {
            left: Box::new(Expr::Number(6.0)),
            op: BinaryOp::Mul,
            right: Box::new(Expr::Number(7.0)),
        };
        let result = evaluate(&expr, &empty_vars());
        assert_eq!(result, Ok(42.0));
    }

    #[test]
    fn test_evaluate_division() {
        let expr = Expr::BinaryOp {
            left: Box::new(Expr::Number(20.0)),
            op: BinaryOp::Div,
            right: Box::new(Expr::Number(4.0)),
        };
        let result = evaluate(&expr, &empty_vars());
        assert_eq!(result, Ok(5.0));
    }

    #[test]
    fn test_evaluate_division_by_zero() {
        let expr = Expr::BinaryOp {
            left: Box::new(Expr::Number(1.0)),
            op: BinaryOp::Div,
            right: Box::new(Expr::Number(0.0)),
        };
        let result = evaluate(&expr, &empty_vars());
        // Rust f64 division by zero yields infinity, not an error
        assert!(result.unwrap().is_infinite());
    }

    #[test]
    fn test_evaluate_modulo() {
        let expr = Expr::BinaryOp {
            left: Box::new(Expr::Number(17.0)),
            op: BinaryOp::Mod,
            right: Box::new(Expr::Number(5.0)),
        };
        let result = evaluate(&expr, &empty_vars());
        assert_eq!(result, Ok(2.0));
    }

    #[test]
    fn test_evaluate_power() {
        let expr = Expr::BinaryOp {
            left: Box::new(Expr::Number(2.0)),
            op: BinaryOp::Pow,
            right: Box::new(Expr::Number(10.0)),
        };
        let result = evaluate(&expr, &empty_vars());
        assert_eq!(result, Ok(1024.0));
    }

    #[test]
    fn test_evaluate_power_fractional() {
        let expr = Expr::BinaryOp {
            left: Box::new(Expr::Number(9.0)),
            op: BinaryOp::Pow,
            right: Box::new(Expr::Number(0.5)),
        };
        let result = evaluate(&expr, &empty_vars());
        assert_eq!(result, Ok(3.0));
    }

    #[test]
    fn test_evaluate_nested_binary_ops() {
        // (2 + 3) * 4 = 20
        let expr = Expr::BinaryOp {
            left: Box::new(Expr::BinaryOp {
                left: Box::new(Expr::Number(2.0)),
                op: BinaryOp::Add,
                right: Box::new(Expr::Number(3.0)),
            }),
            op: BinaryOp::Mul,
            right: Box::new(Expr::Number(4.0)),
        };
        let result = evaluate(&expr, &empty_vars());
        assert_eq!(result, Ok(20.0));
    }

    #[test]
    fn test_evaluate_binary_op_with_variable() {
        // x + 5 where x = 10
        let expr = Expr::BinaryOp {
            left: Box::new(Expr::Variable("x".to_string())),
            op: BinaryOp::Add,
            right: Box::new(Expr::Number(5.0)),
        };
        let result = evaluate(&expr, &vars(&[("x", 10.0)]));
        assert_eq!(result, Ok(15.0));
    }

    // ============================================================
    // Tests for unary minus (3.1.4)
    // ============================================================

    #[test]
    fn test_evaluate_unary_minus() {
        let expr = Expr::UnaryMinus(Box::new(Expr::Number(5.0)));
        let result = evaluate(&expr, &empty_vars());
        assert_eq!(result, Ok(-5.0));
    }

    #[test]
    fn test_evaluate_double_unary_minus() {
        let expr = Expr::UnaryMinus(Box::new(Expr::UnaryMinus(Box::new(Expr::Number(5.0)))));
        let result = evaluate(&expr, &empty_vars());
        assert_eq!(result, Ok(5.0));
    }

    #[test]
    fn test_evaluate_unary_minus_variable() {
        let expr = Expr::UnaryMinus(Box::new(Expr::Variable("x".to_string())));
        let result = evaluate(&expr, &vars(&[("x", 7.0)]));
        assert_eq!(result, Ok(-7.0));
    }

    #[test]
    fn test_evaluate_unary_minus_in_expression() {
        // 3 + (-2) = 1
        let expr = Expr::BinaryOp {
            left: Box::new(Expr::Number(3.0)),
            op: BinaryOp::Add,
            right: Box::new(Expr::UnaryMinus(Box::new(Expr::Number(2.0)))),
        };
        let result = evaluate(&expr, &empty_vars());
        assert_eq!(result, Ok(1.0));
    }

    // ============================================================
    // Tests for basic math functions (3.2.1)
    // ============================================================

    #[test]
    fn test_function_sqrt() {
        let expr = Expr::FunctionCall {
            name: "sqrt".to_string(),
            args: vec![Expr::Number(16.0)],
        };
        let result = evaluate(&expr, &empty_vars());
        assert_eq!(result, Ok(4.0));
    }

    #[test]
    fn test_function_sqrt_of_two() {
        let expr = Expr::FunctionCall {
            name: "sqrt".to_string(),
            args: vec![Expr::Number(2.0)],
        };
        let result = evaluate(&expr, &empty_vars());
        assert!((result.unwrap() - 2.0_f64.sqrt()).abs() < 1e-10);
    }

    #[test]
    fn test_function_cbrt() {
        let expr = Expr::FunctionCall {
            name: "cbrt".to_string(),
            args: vec![Expr::Number(27.0)],
        };
        let result = evaluate(&expr, &empty_vars());
        assert_eq!(result, Ok(3.0));
    }

    #[test]
    fn test_function_abs_positive() {
        let expr = Expr::FunctionCall {
            name: "abs".to_string(),
            args: vec![Expr::Number(5.0)],
        };
        let result = evaluate(&expr, &empty_vars());
        assert_eq!(result, Ok(5.0));
    }

    #[test]
    fn test_function_abs_negative() {
        let expr = Expr::FunctionCall {
            name: "abs".to_string(),
            args: vec![Expr::Number(-5.0)],
        };
        let result = evaluate(&expr, &empty_vars());
        assert_eq!(result, Ok(5.0));
    }

    #[test]
    fn test_function_pow() {
        let expr = Expr::FunctionCall {
            name: "pow".to_string(),
            args: vec![Expr::Number(2.0), Expr::Number(8.0)],
        };
        let result = evaluate(&expr, &empty_vars());
        assert_eq!(result, Ok(256.0));
    }

    // ============================================================
    // Tests for trigonometric functions (3.2.2)
    // ============================================================

    #[test]
    fn test_function_sin_zero() {
        let expr = Expr::FunctionCall {
            name: "sin".to_string(),
            args: vec![Expr::Number(0.0)],
        };
        let result = evaluate(&expr, &empty_vars());
        assert_eq!(result, Ok(0.0));
    }

    #[test]
    fn test_function_sin_pi_over_two() {
        let expr = Expr::FunctionCall {
            name: "sin".to_string(),
            args: vec![Expr::Number(FRAC_PI_2)],
        };
        let result = evaluate(&expr, &empty_vars());
        assert!((result.unwrap() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_function_cos_zero() {
        let expr = Expr::FunctionCall {
            name: "cos".to_string(),
            args: vec![Expr::Number(0.0)],
        };
        let result = evaluate(&expr, &empty_vars());
        assert_eq!(result, Ok(1.0));
    }

    #[test]
    fn test_function_cos_pi() {
        let expr = Expr::FunctionCall {
            name: "cos".to_string(),
            args: vec![Expr::Number(PI)],
        };
        let result = evaluate(&expr, &empty_vars());
        assert!((result.unwrap() - (-1.0)).abs() < 1e-10);
    }

    #[test]
    fn test_function_tan_zero() {
        let expr = Expr::FunctionCall {
            name: "tan".to_string(),
            args: vec![Expr::Number(0.0)],
        };
        let result = evaluate(&expr, &empty_vars());
        assert_eq!(result, Ok(0.0));
    }

    #[test]
    fn test_function_asin() {
        let expr = Expr::FunctionCall {
            name: "asin".to_string(),
            args: vec![Expr::Number(1.0)],
        };
        let result = evaluate(&expr, &empty_vars());
        assert!((result.unwrap() - FRAC_PI_2).abs() < 1e-10);
    }

    #[test]
    fn test_function_acos() {
        let expr = Expr::FunctionCall {
            name: "acos".to_string(),
            args: vec![Expr::Number(1.0)],
        };
        let result = evaluate(&expr, &empty_vars());
        assert_eq!(result, Ok(0.0));
    }

    #[test]
    fn test_function_atan_zero() {
        let expr = Expr::FunctionCall {
            name: "atan".to_string(),
            args: vec![Expr::Number(0.0)],
        };
        let result = evaluate(&expr, &empty_vars());
        assert_eq!(result, Ok(0.0));
    }

    #[test]
    fn test_function_atan2() {
        let expr = Expr::FunctionCall {
            name: "atan2".to_string(),
            args: vec![Expr::Number(1.0), Expr::Number(1.0)],
        };
        let result = evaluate(&expr, &empty_vars());
        assert!((result.unwrap() - FRAC_PI_4).abs() < 1e-10);
    }

    // ============================================================
    // Tests for hyperbolic functions (3.2.3)
    // ============================================================

    #[test]
    fn test_function_sinh_zero() {
        let expr = Expr::FunctionCall {
            name: "sinh".to_string(),
            args: vec![Expr::Number(0.0)],
        };
        let result = evaluate(&expr, &empty_vars());
        assert_eq!(result, Ok(0.0));
    }

    #[test]
    fn test_function_cosh_zero() {
        let expr = Expr::FunctionCall {
            name: "cosh".to_string(),
            args: vec![Expr::Number(0.0)],
        };
        let result = evaluate(&expr, &empty_vars());
        assert_eq!(result, Ok(1.0));
    }

    #[test]
    fn test_function_tanh_zero() {
        let expr = Expr::FunctionCall {
            name: "tanh".to_string(),
            args: vec![Expr::Number(0.0)],
        };
        let result = evaluate(&expr, &empty_vars());
        assert_eq!(result, Ok(0.0));
    }

    #[test]
    fn test_function_asinh_zero() {
        let expr = Expr::FunctionCall {
            name: "asinh".to_string(),
            args: vec![Expr::Number(0.0)],
        };
        let result = evaluate(&expr, &empty_vars());
        assert_eq!(result, Ok(0.0));
    }

    #[test]
    fn test_function_acosh_one() {
        let expr = Expr::FunctionCall {
            name: "acosh".to_string(),
            args: vec![Expr::Number(1.0)],
        };
        let result = evaluate(&expr, &empty_vars());
        assert_eq!(result, Ok(0.0));
    }

    #[test]
    fn test_function_atanh_zero() {
        let expr = Expr::FunctionCall {
            name: "atanh".to_string(),
            args: vec![Expr::Number(0.0)],
        };
        let result = evaluate(&expr, &empty_vars());
        assert_eq!(result, Ok(0.0));
    }

    // ============================================================
    // Tests for logarithmic functions (3.2.4)
    // ============================================================

    #[test]
    fn test_function_ln_e() {
        let expr = Expr::FunctionCall {
            name: "ln".to_string(),
            args: vec![Expr::Number(E)],
        };
        let result = evaluate(&expr, &empty_vars());
        assert!((result.unwrap() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_function_ln_one() {
        let expr = Expr::FunctionCall {
            name: "ln".to_string(),
            args: vec![Expr::Number(1.0)],
        };
        let result = evaluate(&expr, &empty_vars());
        assert_eq!(result, Ok(0.0));
    }

    #[test]
    fn test_function_log_with_base() {
        // log(8, 2) = 3 (log base 2 of 8)
        let expr = Expr::FunctionCall {
            name: "log".to_string(),
            args: vec![Expr::Number(8.0), Expr::Number(2.0)],
        };
        let result = evaluate(&expr, &empty_vars());
        assert!((result.unwrap() - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_function_log2() {
        let expr = Expr::FunctionCall {
            name: "log2".to_string(),
            args: vec![Expr::Number(8.0)],
        };
        let result = evaluate(&expr, &empty_vars());
        assert_eq!(result, Ok(3.0));
    }

    #[test]
    fn test_function_log10() {
        let expr = Expr::FunctionCall {
            name: "log10".to_string(),
            args: vec![Expr::Number(100.0)],
        };
        let result = evaluate(&expr, &empty_vars());
        assert_eq!(result, Ok(2.0));
    }

    #[test]
    fn test_function_exp_one() {
        let expr = Expr::FunctionCall {
            name: "exp".to_string(),
            args: vec![Expr::Number(1.0)],
        };
        let result = evaluate(&expr, &empty_vars());
        assert!((result.unwrap() - E).abs() < 1e-10);
    }

    #[test]
    fn test_function_exp_zero() {
        let expr = Expr::FunctionCall {
            name: "exp".to_string(),
            args: vec![Expr::Number(0.0)],
        };
        let result = evaluate(&expr, &empty_vars());
        assert_eq!(result, Ok(1.0));
    }

    #[test]
    fn test_function_exp2() {
        let expr = Expr::FunctionCall {
            name: "exp2".to_string(),
            args: vec![Expr::Number(3.0)],
        };
        let result = evaluate(&expr, &empty_vars());
        assert_eq!(result, Ok(8.0));
    }

    // ============================================================
    // Tests for rounding functions (3.2.5)
    // ============================================================

    #[test]
    fn test_function_floor() {
        let expr = Expr::FunctionCall {
            name: "floor".to_string(),
            args: vec![Expr::Number(3.9)],
        };
        let result = evaluate(&expr, &empty_vars());
        assert_eq!(result, Ok(3.0));
    }

    #[test]
    fn test_function_floor_negative() {
        let expr = Expr::FunctionCall {
            name: "floor".to_string(),
            args: vec![Expr::Number(-3.1)],
        };
        let result = evaluate(&expr, &empty_vars());
        assert_eq!(result, Ok(-4.0));
    }

    #[test]
    fn test_function_ceil() {
        let expr = Expr::FunctionCall {
            name: "ceil".to_string(),
            args: vec![Expr::Number(3.1)],
        };
        let result = evaluate(&expr, &empty_vars());
        assert_eq!(result, Ok(4.0));
    }

    #[test]
    fn test_function_ceil_negative() {
        let expr = Expr::FunctionCall {
            name: "ceil".to_string(),
            args: vec![Expr::Number(-3.9)],
        };
        let result = evaluate(&expr, &empty_vars());
        assert_eq!(result, Ok(-3.0));
    }

    #[test]
    fn test_function_round_down() {
        let expr = Expr::FunctionCall {
            name: "round".to_string(),
            args: vec![Expr::Number(3.4)],
        };
        let result = evaluate(&expr, &empty_vars());
        assert_eq!(result, Ok(3.0));
    }

    #[test]
    fn test_function_round_up() {
        let expr = Expr::FunctionCall {
            name: "round".to_string(),
            args: vec![Expr::Number(3.6)],
        };
        let result = evaluate(&expr, &empty_vars());
        assert_eq!(result, Ok(4.0));
    }

    // ============================================================
    // Tests for utility functions (3.2.6)
    // ============================================================

    #[test]
    fn test_function_min() {
        let expr = Expr::FunctionCall {
            name: "min".to_string(),
            args: vec![Expr::Number(3.0), Expr::Number(7.0)],
        };
        let result = evaluate(&expr, &empty_vars());
        assert_eq!(result, Ok(3.0));
    }

    #[test]
    fn test_function_min_negative() {
        let expr = Expr::FunctionCall {
            name: "min".to_string(),
            args: vec![Expr::Number(-5.0), Expr::Number(2.0)],
        };
        let result = evaluate(&expr, &empty_vars());
        assert_eq!(result, Ok(-5.0));
    }

    #[test]
    fn test_function_max() {
        let expr = Expr::FunctionCall {
            name: "max".to_string(),
            args: vec![Expr::Number(3.0), Expr::Number(7.0)],
        };
        let result = evaluate(&expr, &empty_vars());
        assert_eq!(result, Ok(7.0));
    }

    #[test]
    fn test_function_max_negative() {
        let expr = Expr::FunctionCall {
            name: "max".to_string(),
            args: vec![Expr::Number(-5.0), Expr::Number(-2.0)],
        };
        let result = evaluate(&expr, &empty_vars());
        assert_eq!(result, Ok(-2.0));
    }

    #[test]
    fn test_function_hypot() {
        let expr = Expr::FunctionCall {
            name: "hypot".to_string(),
            args: vec![Expr::Number(3.0), Expr::Number(4.0)],
        };
        let result = evaluate(&expr, &empty_vars());
        assert_eq!(result, Ok(5.0));
    }

    // ============================================================
    // Tests for error cases (3.3)
    // ============================================================

    #[test]
    fn test_error_unknown_function() {
        let expr = Expr::FunctionCall {
            name: "unknown_func".to_string(),
            args: vec![Expr::Number(1.0)],
        };
        let result = evaluate(&expr, &empty_vars());
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().message(),
            "unknown function 'unknown_func'"
        );
    }

    #[test]
    fn test_error_wrong_argument_count_too_few() {
        let expr = Expr::FunctionCall {
            name: "pow".to_string(),
            args: vec![Expr::Number(2.0)],
        };
        let result = evaluate(&expr, &empty_vars());
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().message(),
            "function 'pow' expects 2 argument(s), got 1"
        );
    }

    #[test]
    fn test_error_wrong_argument_count_too_many() {
        let expr = Expr::FunctionCall {
            name: "sqrt".to_string(),
            args: vec![Expr::Number(4.0), Expr::Number(2.0)],
        };
        let result = evaluate(&expr, &empty_vars());
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().message(),
            "function 'sqrt' expects 1 argument(s), got 2"
        );
    }

    #[test]
    fn test_error_undefined_variable_in_function() {
        let expr = Expr::FunctionCall {
            name: "sqrt".to_string(),
            args: vec![Expr::Variable("undefined".to_string())],
        };
        let result = evaluate(&expr, &empty_vars());
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().message(),
            "undefined variable 'undefined'"
        );
    }

    // ============================================================
    // Tests for complex expressions
    // ============================================================

    #[test]
    fn test_complex_expression_with_functions_and_operators() {
        // sqrt(16) + pow(2, 3) = 4 + 8 = 12
        let expr = Expr::BinaryOp {
            left: Box::new(Expr::FunctionCall {
                name: "sqrt".to_string(),
                args: vec![Expr::Number(16.0)],
            }),
            op: BinaryOp::Add,
            right: Box::new(Expr::FunctionCall {
                name: "pow".to_string(),
                args: vec![Expr::Number(2.0), Expr::Number(3.0)],
            }),
        };
        let result = evaluate(&expr, &empty_vars());
        assert_eq!(result, Ok(12.0));
    }

    #[test]
    fn test_nested_function_calls() {
        // sqrt(abs(-16)) = sqrt(16) = 4
        let expr = Expr::FunctionCall {
            name: "sqrt".to_string(),
            args: vec![Expr::FunctionCall {
                name: "abs".to_string(),
                args: vec![Expr::Number(-16.0)],
            }],
        };
        let result = evaluate(&expr, &empty_vars());
        assert_eq!(result, Ok(4.0));
    }

    #[test]
    fn test_expression_with_multiple_variables() {
        // a * b + c where a=2, b=3, c=4 => 10
        let expr = Expr::BinaryOp {
            left: Box::new(Expr::BinaryOp {
                left: Box::new(Expr::Variable("a".to_string())),
                op: BinaryOp::Mul,
                right: Box::new(Expr::Variable("b".to_string())),
            }),
            op: BinaryOp::Add,
            right: Box::new(Expr::Variable("c".to_string())),
        };
        let result = evaluate(&expr, &vars(&[("a", 2.0), ("b", 3.0), ("c", 4.0)]));
        assert_eq!(result, Ok(10.0));
    }

    // ============================================================
    // Tests for edge cases with special values
    // ============================================================

    #[test]
    fn test_sqrt_negative_returns_nan() {
        let expr = Expr::FunctionCall {
            name: "sqrt".to_string(),
            args: vec![Expr::Number(-1.0)],
        };
        let result = evaluate(&expr, &empty_vars());
        assert!(result.unwrap().is_nan());
    }

    #[test]
    fn test_log_zero_returns_neg_infinity() {
        let expr = Expr::FunctionCall {
            name: "ln".to_string(),
            args: vec![Expr::Number(0.0)],
        };
        let result = evaluate(&expr, &empty_vars());
        let val = result.unwrap();
        assert!(val.is_infinite() && val < 0.0);
    }

    #[test]
    fn test_acosh_less_than_one_returns_nan() {
        let expr = Expr::FunctionCall {
            name: "acosh".to_string(),
            args: vec![Expr::Number(0.5)],
        };
        let result = evaluate(&expr, &empty_vars());
        assert!(result.unwrap().is_nan());
    }
}
