//! Variable context management for expression evaluation.
//!
//! Wraps evalexpr's `HashMapContext` to provide variable storage and retrieval
//! that persists across line evaluations.

use evalexpr::{
    ContextWithMutableFunctions, Function, HashMapContext, IterateVariablesContext, Value,
};
use std::collections::HashMap;

/// Evaluation context that manages variable bindings.
///
/// Wraps evalexpr's `HashMapContext` and provides methods for storing and
/// retrieving variables during expression evaluation.
#[derive(Debug, Default)]
pub struct EvalContext {
    inner: HashMapContext,
}

impl EvalContext {
    /// Creates a new evaluation context with predefined mathematical constants.
    ///
    /// The following constants are pre-defined:
    /// - `pi`: 3.141592653589793 (mathematical constant pi)
    /// - `e`: 2.718281828459045 (Euler's number)
    #[must_use]
    pub fn new() -> Self {
        let mut ctx = Self::default();
        ctx.init_constants();
        ctx.init_math_functions();
        ctx
    }

    /// Initializes mathematical constants in the context.
    fn init_constants(&mut self) {
        self.set_variable("pi", Value::Float(std::f64::consts::PI));
        self.set_variable("e", Value::Float(std::f64::consts::E));
    }

    /// Registers mathematical function aliases without the `math::` prefix.
    ///
    /// This provides user-friendly short names for common mathematical functions
    /// that would otherwise require the `math::` namespace prefix.
    fn init_math_functions(&mut self) {
        // Basic math functions
        self.register_math_fn("sqrt", |x: f64| x.sqrt());
        self.register_math_fn("cbrt", |x: f64| x.cbrt());
        self.register_math_fn_preserving_int("abs", |x: f64| x.abs(), |x: i64| x.abs());
        self.register_math_fn2("pow", |base: f64, exp: f64| base.powf(exp));

        // Trigonometric functions
        self.register_math_fn("sin", |x: f64| x.sin());
        self.register_math_fn("cos", |x: f64| x.cos());
        self.register_math_fn("tan", |x: f64| x.tan());
        self.register_math_fn("asin", |x: f64| x.asin());
        self.register_math_fn("acos", |x: f64| x.acos());
        self.register_math_fn("atan", |x: f64| x.atan());
        self.register_math_fn2("atan2", |y: f64, x: f64| y.atan2(x));

        // Hyperbolic functions
        self.register_math_fn("sinh", |x: f64| x.sinh());
        self.register_math_fn("cosh", |x: f64| x.cosh());
        self.register_math_fn("tanh", |x: f64| x.tanh());
        self.register_math_fn("asinh", |x: f64| x.asinh());
        self.register_math_fn("acosh", |x: f64| x.acosh());
        self.register_math_fn("atanh", |x: f64| x.atanh());

        // Logarithmic and exponential functions
        self.register_math_fn("ln", |x: f64| x.ln());
        self.register_math_fn("log", |x: f64| x.log10());
        self.register_math_fn("log2", |x: f64| x.log2());
        self.register_math_fn("log10", |x: f64| x.log10());
        self.register_math_fn("exp", |x: f64| x.exp());
        self.register_math_fn("exp2", |x: f64| x.exp2());

        // Rounding functions (these may already exist without prefix, but register for consistency)
        // Note: evalexpr already provides floor, ceil without prefix - skip to avoid conflicts

        // Utility functions
        self.register_math_fn2("hypot", |a: f64, b: f64| a.hypot(b));
        // Note: min and max are already provided by evalexpr without prefix
    }

    /// Helper to register a single-argument math function that returns f64.
    fn register_math_fn<F>(&mut self, name: &str, f: F)
    where
        F: Fn(f64) -> f64 + Send + Sync + Clone + 'static,
    {
        let _ = self.inner.set_function(
            name.into(),
            Function::new(move |arg| {
                let x = arg.as_number()?;
                Ok(Value::Float(f(x)))
            }),
        );
    }

    /// Helper to register abs that preserves int type when given int input.
    fn register_math_fn_preserving_int<F, G>(&mut self, name: &str, float_fn: F, int_fn: G)
    where
        F: Fn(f64) -> f64 + Send + Sync + Clone + 'static,
        G: Fn(i64) -> i64 + Send + Sync + Clone + 'static,
    {
        let _ = self.inner.set_function(
            name.into(),
            Function::new(move |arg| {
                if let Ok(i) = arg.as_int() {
                    Ok(Value::Int(int_fn(i)))
                } else {
                    let x = arg.as_number()?;
                    Ok(Value::Float(float_fn(x)))
                }
            }),
        );
    }

    /// Helper to register a two-argument math function that returns f64.
    fn register_math_fn2<F>(&mut self, name: &str, f: F)
    where
        F: Fn(f64, f64) -> f64 + Send + Sync + Clone + 'static,
    {
        let _ = self.inner.set_function(
            name.into(),
            Function::new(move |arg| {
                let args = arg.as_tuple()?;
                if args.len() != 2 {
                    return Err(evalexpr::EvalexprError::WrongFunctionArgumentAmount {
                        expected: 2..=2,
                        actual: args.len(),
                    });
                }
                let a = args[0].as_number()?;
                let b = args[1].as_number()?;
                Ok(Value::Float(f(a, b)))
            }),
        );
    }

    /// Stores a variable with the given name and value.
    pub fn set_variable(&mut self, name: &str, value: Value) {
        // evalexpr's set_value returns a Result, but we can ignore errors
        // for simple variable assignments
        let _ =
            evalexpr::ContextWithMutableVariables::set_value(&mut self.inner, name.into(), value);
    }

    /// Retrieves a variable by name.
    ///
    /// Returns `None` if the variable is not defined.
    #[must_use]
    pub fn get_variable(&self, name: &str) -> Option<&Value> {
        evalexpr::Context::get_value(&self.inner, name)
    }

    /// Returns a reference to the inner evalexpr context.
    ///
    /// Used when evaluating expressions with variable references.
    #[must_use]
    pub const fn inner(&self) -> &HashMapContext {
        &self.inner
    }

    /// Returns a mutable reference to the inner evalexpr context.
    ///
    /// Used when evaluating expressions that may modify variables.
    pub const fn inner_mut(&mut self) -> &mut HashMapContext {
        &mut self.inner
    }

    /// Clears all variables from the context.
    pub fn clear(&mut self) {
        self.inner = HashMapContext::new();
    }

    /// Extracts all numeric variables as a `HashMap<String, f64>`.
    ///
    /// Only `Int` and `Float` values are included. Non-numeric values
    /// (String, Boolean, Tuple, Empty) are ignored.
    ///
    /// Note: Converting `i64` to `f64` may lose precision for very large integers,
    /// but this is acceptable for calculator use cases.
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn extract_variables(&self) -> HashMap<String, f64> {
        self.inner
            .iter_variables()
            .filter_map(|(name, value)| {
                match value {
                    Value::Int(i) => Some((name, i as f64)),
                    Value::Float(f) => Some((name, f)),
                    _ => None, // Ignore non-numeric values
                }
            })
            .collect()
    }

    /// Loads variables from a `HashMap<String, f64>`.
    ///
    /// All values are stored as `Float` values in the context.
    pub fn load_variables(&mut self, variables: &HashMap<String, f64>) {
        for (name, &value) in variables {
            self.set_variable(name, Value::Float(value));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_context_is_empty() {
        let context = EvalContext::new();
        assert!(context.get_variable("x").is_none());
    }

    #[test]
    fn test_set_and_get_integer_variable() {
        let mut context = EvalContext::new();
        context.set_variable("a", Value::Int(42));

        let value = context.get_variable("a");
        assert_eq!(value, Some(&Value::Int(42)));
    }

    #[test]
    fn test_set_and_get_float_variable() {
        let mut context = EvalContext::new();
        context.set_variable("my_float", Value::Float(1.5));

        let value = context.get_variable("my_float");
        assert_eq!(value, Some(&Value::Float(1.5)));
    }

    #[test]
    fn test_overwrite_variable() {
        let mut context = EvalContext::new();
        context.set_variable("x", Value::Int(10));
        context.set_variable("x", Value::Int(20));

        assert_eq!(context.get_variable("x"), Some(&Value::Int(20)));
    }

    #[test]
    fn test_multiple_variables() {
        let mut context = EvalContext::new();
        context.set_variable("a", Value::Int(1));
        context.set_variable("b", Value::Int(2));
        context.set_variable("c", Value::Int(3));

        assert_eq!(context.get_variable("a"), Some(&Value::Int(1)));
        assert_eq!(context.get_variable("b"), Some(&Value::Int(2)));
        assert_eq!(context.get_variable("c"), Some(&Value::Int(3)));
    }

    #[test]
    fn test_get_undefined_variable() {
        let context = EvalContext::new();
        assert!(context.get_variable("undefined").is_none());
    }

    #[test]
    fn test_clear_context() {
        let mut context = EvalContext::new();
        context.set_variable("x", Value::Int(10));
        context.clear();

        assert!(context.get_variable("x").is_none());
    }

    #[test]
    fn test_default_trait() {
        let context = EvalContext::default();
        assert!(context.get_variable("x").is_none());
    }

    // === extract_variables Tests ===

    #[test]
    fn test_extract_variables_new_context_contains_constants() {
        let context = EvalContext::new();
        let vars = context.extract_variables();
        // New context contains predefined constants pi and e
        assert_eq!(vars.len(), 2);
        assert!(vars.contains_key("pi"));
        assert!(vars.contains_key("e"));
    }

    #[test]
    fn test_extract_variables_with_integers() {
        let mut context = EvalContext::new();
        context.set_variable("x", Value::Int(42));
        context.set_variable("y", Value::Int(100));

        let vars = context.extract_variables();
        assert_eq!(vars.get("x"), Some(&42.0));
        assert_eq!(vars.get("y"), Some(&100.0));
    }

    #[test]
    fn test_extract_variables_with_floats() {
        let mut context = EvalContext::new();
        context.set_variable("first", Value::Float(1.234_56));
        context.set_variable("second", Value::Float(9.876_54));

        let vars = context.extract_variables();
        assert!((vars.get("first").unwrap() - 1.234_56).abs() < 0.0001);
        assert!((vars.get("second").unwrap() - 9.876_54).abs() < 0.0001);
    }

    #[test]
    fn test_extract_variables_mixed_int_and_float() {
        let mut context = EvalContext::new();
        context.set_variable("int_val", Value::Int(10));
        context.set_variable("float_val", Value::Float(20.5));

        let vars = context.extract_variables();
        assert_eq!(vars.get("int_val"), Some(&10.0));
        assert!((vars.get("float_val").unwrap() - 20.5).abs() < 0.0001);
    }

    #[test]
    fn test_extract_variables_ignores_non_numeric() {
        let mut context = EvalContext::new();
        context.set_variable("num", Value::Int(42));
        context.set_variable("text", Value::String("hello".to_string()));
        context.set_variable("flag", Value::Boolean(true));

        let vars = context.extract_variables();
        // Only numeric values should be extracted (includes predefined pi and e)
        assert_eq!(vars.len(), 3); // num + pi + e
        assert_eq!(vars.get("num"), Some(&42.0));
        assert!(!vars.contains_key("text"));
        assert!(!vars.contains_key("flag"));
    }

    // === load_variables Tests ===

    #[test]
    fn test_load_variables_empty_map_preserves_constants() {
        let mut context = EvalContext::new();
        context.load_variables(&HashMap::new());
        // Loading empty map preserves predefined constants
        let vars = context.extract_variables();
        assert_eq!(vars.len(), 2); // pi and e
        assert!(vars.contains_key("pi"));
        assert!(vars.contains_key("e"));
    }

    #[test]
    fn test_load_variables_with_values() {
        let mut context = EvalContext::new();
        let mut vars = HashMap::new();
        vars.insert("x".to_string(), 42.0);
        vars.insert("y".to_string(), 3.125);

        context.load_variables(&vars);

        // Values should be stored as Float
        assert_eq!(context.get_variable("x"), Some(&Value::Float(42.0)));
        assert_eq!(context.get_variable("y"), Some(&Value::Float(3.125)));
    }

    #[test]
    fn test_load_variables_roundtrip() {
        let mut context1 = EvalContext::new();
        context1.set_variable("a", Value::Int(10));
        context1.set_variable("b", Value::Float(20.5));

        let extracted = context1.extract_variables();

        let mut context2 = EvalContext::new();
        context2.load_variables(&extracted);

        let extracted2 = context2.extract_variables();
        assert_eq!(extracted, extracted2);
    }

    // === Mathematical Constants Tests ===

    #[test]
    fn test_new_context_has_pi_constant() {
        let context = EvalContext::new();
        let pi = context.get_variable("pi");
        assert!(pi.is_some(), "pi should be predefined");
        if let Some(Value::Float(value)) = pi {
            assert!(
                (*value - std::f64::consts::PI).abs() < 1e-15,
                "pi should be 3.141592653589793"
            );
        } else {
            panic!("pi should be a Float value");
        }
    }

    #[test]
    fn test_new_context_has_e_constant() {
        let context = EvalContext::new();
        let e = context.get_variable("e");
        assert!(e.is_some(), "e should be predefined");
        if let Some(Value::Float(value)) = e {
            assert!(
                (*value - std::f64::consts::E).abs() < 1e-15,
                "e should be 2.718281828459045"
            );
        } else {
            panic!("e should be a Float value");
        }
    }

    #[test]
    fn test_clear_removes_constants_but_new_restores_them() {
        let mut context = EvalContext::new();
        assert!(context.get_variable("pi").is_some());
        context.clear();
        // After clear, constants are gone (clear is a full reset)
        assert!(context.get_variable("pi").is_none());
        // But a new context has them
        let fresh_context = EvalContext::new();
        assert!(fresh_context.get_variable("pi").is_some());
    }
}
