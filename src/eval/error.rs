//! Error types for expression evaluation.
//!
//! Provides error formatting with optional span information for highlighting
//! error locations in the source expression.

use std::fmt;

/// Span indicating the position of an error (0-indexed, end is exclusive).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ErrorSpan {
    pub start: usize,
    pub end: usize,
}

impl ErrorSpan {
    /// Creates a new error span.
    #[must_use]
    pub const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

/// Error returned from expression evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvalError {
    /// Human-readable error message.
    message: String,
    /// Optional span indicating where the error occurred.
    span: Option<ErrorSpan>,
}

impl EvalError {
    /// Creates a new evaluation error with a message.
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            span: None,
        }
    }

    /// Creates a new evaluation error with a message and span.
    #[must_use]
    pub fn with_span(message: impl Into<String>, span: ErrorSpan) -> Self {
        Self {
            message: message.into(),
            span: Some(span),
        }
    }

    /// Creates a new evaluation error with a message and span from start/end positions.
    #[must_use]
    pub fn with_span_range(message: impl Into<String>, start: usize, end: usize) -> Self {
        Self {
            message: message.into(),
            span: Some(ErrorSpan::new(start, end)),
        }
    }

    /// Creates an error for an undefined variable.
    #[must_use]
    pub fn undefined_variable(name: &str) -> Self {
        Self::new(format!("undefined variable '{name}'"))
    }

    /// Creates an error for an unknown function.
    #[must_use]
    pub fn unknown_function(name: &str) -> Self {
        Self::new(format!("unknown function '{name}'"))
    }

    /// Creates an error for invalid argument count.
    #[must_use]
    pub fn invalid_argument_count(name: &str, expected: usize, got: usize) -> Self {
        Self::new(format!(
            "function '{name}' expects {expected} argument(s), got {got}"
        ))
    }

    /// Returns the error message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the error span, if available.
    #[must_use]
    pub const fn span(&self) -> Option<ErrorSpan> {
        self.span
    }
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for EvalError {}

impl From<crate::eval::token::TokenError> for EvalError {
    fn from(err: crate::eval::token::TokenError) -> Self {
        // Create a span of length 1 at the error position
        Self::with_span_range(err.message, err.position, err.position + 1)
    }
}

impl From<crate::eval::ast::ParseError> for EvalError {
    fn from(err: crate::eval::ast::ParseError) -> Self {
        match err.span {
            Some((start, end)) => Self::with_span_range(err.message, start, end),
            None => Self::new(err.message),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_span_new() {
        let span = ErrorSpan::new(5, 10);
        assert_eq!(span.start, 5);
        assert_eq!(span.end, 10);
    }

    #[test]
    fn test_eval_error_new() {
        let error = EvalError::new("test error");
        assert_eq!(error.message(), "test error");
        assert!(error.span().is_none());
    }

    #[test]
    fn test_eval_error_with_span() {
        let span = ErrorSpan::new(3, 7);
        let error = EvalError::with_span("syntax error", span);
        assert_eq!(error.message(), "syntax error");
        assert_eq!(error.span(), Some(span));
    }

    #[test]
    fn test_eval_error_with_span_from_positions() {
        let error = EvalError::with_span_range("syntax error", 3, 7);
        assert_eq!(error.message(), "syntax error");
        assert_eq!(error.span(), Some(ErrorSpan::new(3, 7)));
    }

    #[test]
    fn test_eval_error_display() {
        let error = EvalError::new("division by zero");
        assert_eq!(format!("{error}"), "division by zero");
    }

    #[test]
    fn test_undefined_variable_error() {
        let error = EvalError::undefined_variable("foo");
        assert_eq!(error.message(), "undefined variable 'foo'");
        assert!(error.span().is_none());
    }

    #[test]
    fn test_unknown_function_error() {
        let error = EvalError::unknown_function("bar");
        assert_eq!(error.message(), "unknown function 'bar'");
        assert!(error.span().is_none());
    }

    #[test]
    fn test_invalid_argument_count_error() {
        let error = EvalError::invalid_argument_count("sin", 1, 2);
        assert_eq!(
            error.message(),
            "function 'sin' expects 1 argument(s), got 2"
        );
        assert!(error.span().is_none());
    }

    #[test]
    fn test_from_token_error() {
        use crate::eval::token::TokenError;
        let token_error = TokenError::new("invalid character", 5);
        let eval_error: EvalError = token_error.into();
        assert_eq!(eval_error.message(), "invalid character");
        assert_eq!(eval_error.span(), Some(ErrorSpan::new(5, 6)));
    }

    #[test]
    fn test_from_parse_error() {
        use crate::eval::ast::ParseError;
        let parse_error = ParseError::new("unexpected token");
        let eval_error: EvalError = parse_error.into();
        assert_eq!(eval_error.message(), "unexpected token");
        assert!(eval_error.span().is_none());
    }

    #[test]
    fn test_from_parse_error_with_span() {
        use crate::eval::ast::ParseError;
        use crate::eval::token::Span;
        let parse_error = ParseError::with_span("unexpected token", Span { start: 3, end: 7 });
        let eval_error: EvalError = parse_error.into();
        assert_eq!(eval_error.message(), "unexpected token");
        assert_eq!(eval_error.span(), Some(ErrorSpan::new(3, 7)));
    }
}
