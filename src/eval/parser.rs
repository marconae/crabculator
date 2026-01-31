//! Expression parsing for identifying assignments, expressions, and empty lines.
//!
//! This module parses input lines to determine their type before evaluation.

/// Represents a parsed line of input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParsedLine {
    /// An assignment expression: `name = expression`
    Assignment {
        /// The variable name being assigned.
        name: String,
        /// The expression to evaluate and assign.
        expression: String,
    },
    /// A standalone expression to evaluate.
    Expression(String),
    /// An empty or whitespace-only line.
    Empty,
}

/// Parses a line of input into its structural form.
///
/// # Arguments
/// * `line` - The input line to parse
///
/// # Returns
/// A `ParsedLine` indicating whether this is an assignment, expression, or empty line.
#[must_use]
pub fn parse_line(line: &str) -> ParsedLine {
    let trimmed = line.trim();

    if trimmed.is_empty() {
        return ParsedLine::Empty;
    }

    if let Some(assignment) = try_parse_assignment(trimmed) {
        return assignment;
    }

    ParsedLine::Expression(trimmed.to_string())
}

/// Attempts to parse an assignment expression.
///
/// Returns `None` if the line is not a valid assignment.
fn try_parse_assignment(line: &str) -> Option<ParsedLine> {
    let mut chars = line.char_indices().peekable();
    let mut equals_pos = None;

    while let Some((i, c)) = chars.next() {
        if c == '=' {
            let prev_char = if i > 0 { line.chars().nth(i - 1) } else { None };
            let next_char = chars.peek().map(|(_, c)| *c);

            let is_comparison =
                matches!(prev_char, Some('!' | '<' | '>' | '=')) || matches!(next_char, Some('='));

            if !is_comparison {
                equals_pos = Some(i);
                break;
            }
        }
    }

    let equals_pos = equals_pos?;

    let name_part = line[..equals_pos].trim();
    let expr_part = line[equals_pos + 1..].trim();

    if !is_valid_identifier(name_part) {
        return None;
    }

    if expr_part.is_empty() {
        return None;
    }

    Some(ParsedLine::Assignment {
        name: name_part.to_string(),
        expression: expr_part.to_string(),
    })
}

/// Checks if a string is a valid identifier.
///
/// Valid identifiers start with a letter or underscore, followed by
/// letters, digits, or underscores.
fn is_valid_identifier(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    let mut chars = s.chars();
    let first = chars.next().unwrap();

    if !first.is_alphabetic() && first != '_' {
        return false;
    }

    chars.all(|c| c.is_alphanumeric() || c == '_')
}

#[cfg(test)]
mod tests {
    use super::*;

    // Empty line tests
    #[test]
    fn test_parse_empty_line() {
        assert_eq!(parse_line(""), ParsedLine::Empty);
    }

    #[test]
    fn test_parse_whitespace_line() {
        assert_eq!(parse_line("   "), ParsedLine::Empty);
        assert_eq!(parse_line("\t"), ParsedLine::Empty);
        assert_eq!(parse_line("  \t  "), ParsedLine::Empty);
    }

    // Assignment tests
    #[test]
    fn test_parse_simple_assignment() {
        assert_eq!(
            parse_line("a = 5"),
            ParsedLine::Assignment {
                name: "a".to_string(),
                expression: "5".to_string(),
            }
        );
    }

    #[test]
    fn test_parse_assignment_with_expression() {
        assert_eq!(
            parse_line("result = 5 + 3 * 2"),
            ParsedLine::Assignment {
                name: "result".to_string(),
                expression: "5 + 3 * 2".to_string(),
            }
        );
    }

    #[test]
    fn test_parse_assignment_no_spaces() {
        assert_eq!(
            parse_line("x=10"),
            ParsedLine::Assignment {
                name: "x".to_string(),
                expression: "10".to_string(),
            }
        );
    }

    #[test]
    fn test_parse_assignment_underscore_name() {
        assert_eq!(
            parse_line("_private = 42"),
            ParsedLine::Assignment {
                name: "_private".to_string(),
                expression: "42".to_string(),
            }
        );
    }

    #[test]
    fn test_parse_assignment_complex_name() {
        assert_eq!(
            parse_line("my_var_123 = 100"),
            ParsedLine::Assignment {
                name: "my_var_123".to_string(),
                expression: "100".to_string(),
            }
        );
    }

    // Expression tests
    #[test]
    fn test_parse_simple_expression() {
        assert_eq!(
            parse_line("5 + 3"),
            ParsedLine::Expression("5 + 3".to_string())
        );
    }

    #[test]
    fn test_parse_expression_with_parentheses() {
        assert_eq!(
            parse_line("(5 + 3) * 2"),
            ParsedLine::Expression("(5 + 3) * 2".to_string())
        );
    }

    #[test]
    fn test_parse_expression_with_function() {
        assert_eq!(
            parse_line("sqrt(16)"),
            ParsedLine::Expression("sqrt(16)".to_string())
        );
    }

    // Comparison operator tests (should NOT be parsed as assignments)
    #[test]
    fn test_parse_equality_comparison() {
        assert_eq!(
            parse_line("a == b"),
            ParsedLine::Expression("a == b".to_string())
        );
    }

    #[test]
    fn test_parse_not_equal_comparison() {
        assert_eq!(
            parse_line("a != b"),
            ParsedLine::Expression("a != b".to_string())
        );
    }

    #[test]
    fn test_parse_less_equal_comparison() {
        assert_eq!(
            parse_line("a <= b"),
            ParsedLine::Expression("a <= b".to_string())
        );
    }

    #[test]
    fn test_parse_greater_equal_comparison() {
        assert_eq!(
            parse_line("a >= b"),
            ParsedLine::Expression("a >= b".to_string())
        );
    }

    // Edge cases
    #[test]
    fn test_parse_assignment_with_comparison_in_expression() {
        // a = b == c should be assignment where expression is "b == c"
        assert_eq!(
            parse_line("a = b == c"),
            ParsedLine::Assignment {
                name: "a".to_string(),
                expression: "b == c".to_string(),
            }
        );
    }

    #[test]
    fn test_parse_number_not_valid_identifier() {
        // "5 = 10" should not be an assignment (5 is not a valid identifier)
        assert_eq!(
            parse_line("5 = 10"),
            ParsedLine::Expression("5 = 10".to_string())
        );
    }

    #[test]
    fn test_parse_trims_whitespace() {
        assert_eq!(
            parse_line("  5 + 3  "),
            ParsedLine::Expression("5 + 3".to_string())
        );
    }

    #[test]
    fn test_parse_single_number() {
        assert_eq!(parse_line("42"), ParsedLine::Expression("42".to_string()));
    }

    #[test]
    fn test_parse_variable_reference() {
        assert_eq!(parse_line("x"), ParsedLine::Expression("x".to_string()));
    }

    // Valid identifier tests
    #[test]
    fn test_is_valid_identifier() {
        assert!(is_valid_identifier("x"));
        assert!(is_valid_identifier("_x"));
        assert!(is_valid_identifier("x1"));
        assert!(is_valid_identifier("my_variable"));
        assert!(is_valid_identifier("CamelCase"));

        assert!(!is_valid_identifier(""));
        assert!(!is_valid_identifier("123"));
        assert!(!is_valid_identifier("1x"));
        assert!(!is_valid_identifier("x-y"));
        assert!(!is_valid_identifier("x y"));
    }
}
