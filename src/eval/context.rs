//! Variable context management for expression evaluation.
//!
//! Wraps evalexpr's `HashMapContext` to provide variable storage and retrieval
//! that persists across line evaluations.

use evalexpr::{HashMapContext, IterateVariablesContext, Value};
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
    /// Creates a new empty evaluation context.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
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
    fn test_extract_variables_empty_context() {
        let context = EvalContext::new();
        let vars = context.extract_variables();
        assert!(vars.is_empty());
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
        // Only numeric values should be extracted
        assert_eq!(vars.len(), 1);
        assert_eq!(vars.get("num"), Some(&42.0));
        assert!(!vars.contains_key("text"));
        assert!(!vars.contains_key("flag"));
    }

    // === load_variables Tests ===

    #[test]
    fn test_load_variables_empty_map() {
        let mut context = EvalContext::new();
        context.load_variables(&HashMap::new());
        assert!(context.extract_variables().is_empty());
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
}
