//! Error types for expression evaluation.
//!
//! Provides error formatting with optional span information for highlighting
//! error locations in the source expression.

use std::fmt;

/// Span indicating the position of an error in the source expression.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ErrorSpan {
    /// Starting column (0-indexed).
    pub start: usize,
    /// Ending column (exclusive, 0-indexed).
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
    fn test_eval_error_display() {
        let error = EvalError::new("division by zero");
        assert_eq!(format!("{error}"), "division by zero");
    }
}
