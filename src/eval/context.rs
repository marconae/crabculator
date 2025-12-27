//! Variable context management for expression evaluation.
//!
//! Provides variable storage and retrieval that persists across line evaluations.

use std::collections::HashMap;

/// Evaluation context that manages variable bindings.
///
/// Stores variables as `f64` values in a `HashMap` and provides methods for
/// storing and retrieving variables during expression evaluation.
#[derive(Debug, Default)]
pub struct EvalContext {
    inner: HashMap<String, f64>,
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
        ctx
    }

    /// Initializes mathematical constants in the context.
    fn init_constants(&mut self) {
        self.inner.insert("pi".to_string(), std::f64::consts::PI);
        self.inner.insert("e".to_string(), std::f64::consts::E);
    }

    /// Stores a variable with the given name and value.
    pub fn set_variable(&mut self, name: &str, value: f64) {
        self.inner.insert(name.to_string(), value);
    }

    /// Retrieves a variable by name.
    ///
    /// Returns `None` if the variable is not defined.
    #[must_use]
    pub fn get_variable(&self, name: &str) -> Option<f64> {
        self.inner.get(name).copied()
    }

    /// Returns a reference to the inner variable map.
    ///
    /// Used when evaluating expressions with variable references.
    #[must_use]
    pub const fn variables(&self) -> &HashMap<String, f64> {
        &self.inner
    }

    /// Clears all variables from the context.
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /// Extracts all variables as a `HashMap<String, f64>`.
    #[must_use]
    pub fn extract_variables(&self) -> HashMap<String, f64> {
        self.inner.clone()
    }

    /// Loads variables from a `HashMap<String, f64>`.
    pub fn load_variables(&mut self, variables: &HashMap<String, f64>) {
        for (name, &value) in variables {
            self.inner.insert(name.clone(), value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_context_is_empty() {
        let context = EvalContext::new();
        // New context has predefined constants but arbitrary variable x should not exist
        assert!(context.get_variable("x").is_none());
    }

    #[test]
    fn test_set_and_get_integer_variable() {
        let mut context = EvalContext::new();
        context.set_variable("a", 42.0);

        let value = context.get_variable("a");
        assert_eq!(value, Some(42.0));
    }

    #[test]
    fn test_set_and_get_float_variable() {
        let mut context = EvalContext::new();
        context.set_variable("my_float", 1.5);

        let value = context.get_variable("my_float");
        assert_eq!(value, Some(1.5));
    }

    #[test]
    fn test_overwrite_variable() {
        let mut context = EvalContext::new();
        context.set_variable("x", 10.0);
        context.set_variable("x", 20.0);

        assert_eq!(context.get_variable("x"), Some(20.0));
    }

    #[test]
    fn test_multiple_variables() {
        let mut context = EvalContext::new();
        context.set_variable("a", 1.0);
        context.set_variable("b", 2.0);
        context.set_variable("c", 3.0);

        assert_eq!(context.get_variable("a"), Some(1.0));
        assert_eq!(context.get_variable("b"), Some(2.0));
        assert_eq!(context.get_variable("c"), Some(3.0));
    }

    #[test]
    fn test_get_undefined_variable() {
        let context = EvalContext::new();
        assert!(context.get_variable("undefined").is_none());
    }

    #[test]
    fn test_clear_context() {
        let mut context = EvalContext::new();
        context.set_variable("x", 10.0);
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
        context.set_variable("x", 42.0);
        context.set_variable("y", 100.0);

        let vars = context.extract_variables();
        assert_eq!(vars.get("x"), Some(&42.0));
        assert_eq!(vars.get("y"), Some(&100.0));
    }

    #[test]
    fn test_extract_variables_with_floats() {
        let mut context = EvalContext::new();
        context.set_variable("first", 1.234_56);
        context.set_variable("second", 9.876_54);

        let vars = context.extract_variables();
        assert!((vars.get("first").unwrap() - 1.234_56).abs() < 0.0001);
        assert!((vars.get("second").unwrap() - 9.876_54).abs() < 0.0001);
    }

    #[test]
    fn test_extract_variables_mixed_int_and_float() {
        let mut context = EvalContext::new();
        context.set_variable("int_val", 10.0);
        context.set_variable("float_val", 20.5);

        let vars = context.extract_variables();
        assert_eq!(vars.get("int_val"), Some(&10.0));
        assert!((vars.get("float_val").unwrap() - 20.5).abs() < 0.0001);
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

        assert_eq!(context.get_variable("x"), Some(42.0));
        assert_eq!(context.get_variable("y"), Some(3.125));
    }

    #[test]
    fn test_load_variables_roundtrip() {
        let mut context1 = EvalContext::new();
        context1.set_variable("a", 10.0);
        context1.set_variable("b", 20.5);

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
        let value = pi.unwrap();
        assert!(
            (value - std::f64::consts::PI).abs() < 1e-15,
            "pi should be 3.141592653589793"
        );
    }

    #[test]
    fn test_new_context_has_e_constant() {
        let context = EvalContext::new();
        let e = context.get_variable("e");
        assert!(e.is_some(), "e should be predefined");
        let value = e.unwrap();
        assert!(
            (value - std::f64::consts::E).abs() < 1e-15,
            "e should be 2.718281828459045"
        );
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
