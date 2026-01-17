//! Syntax highlighting for the expression editor.
//!
//! Provides tokenization and color styling for calculator expressions.
//! Tokens are categorized and styled as follows:
//! - Variables: cyan color
//! - Numbers: white/default color
//! - Operators: dimmed/grey color
//! - Parentheses: default color
//! - Functions: cyan color (like variables)
//! - Whitespace: default color

use ratatui::{
    style::{Color, Style},
    text::Span,
};

/// Token types for syntax highlighting.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    /// Variable names (e.g., x, foo, myVar)
    Variable,
    /// Numeric literals (e.g., 42, 3.14, -5)
    Number,
    /// Operators (+, -, *, /, %, ^, =)
    Operator,
    /// Parentheses ( and )
    Parenthesis,
    /// Whitespace characters
    Whitespace,
    /// Function names (sqrt, sin, cos, etc.)
    Function,
}

/// A token with its type and text content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    /// The type of the token.
    pub token_type: TokenType,
    /// The text content of the token.
    pub text: String,
}

impl Token {
    /// Creates a new token.
    #[must_use]
    pub fn new(token_type: TokenType, text: impl Into<String>) -> Self {
        Self {
            token_type,
            text: text.into(),
        }
    }
}

/// Known function names that should be highlighted as functions.
const KNOWN_FUNCTIONS: &[&str] = &[
    "sqrt", "sin", "cos", "tan", "asin", "acos", "atan", "sinh", "cosh", "tanh", "asinh", "acosh",
    "atanh", "ln", "log", "log2", "log10", "exp", "exp2", "floor", "ceil", "round", "abs", "min",
    "max", "pow",
];

/// Known constants that should be highlighted as numbers.
const KNOWN_CONSTANTS: &[&str] = &["pi", "e"];

/// Tokenizes an expression string into tokens for syntax highlighting.
///
/// # Arguments
/// * `input` - The expression string to tokenize
///
/// # Returns
/// A vector of tokens representing the expression.
#[must_use]
pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

        if c.is_whitespace() {
            // Collect consecutive whitespace
            let start = i;
            while i < chars.len() && chars[i].is_whitespace() {
                i += 1;
            }
            tokens.push(Token::new(
                TokenType::Whitespace,
                chars[start..i].iter().collect::<String>(),
            ));
        } else if c.is_ascii_digit()
            || (c == '.' && i + 1 < chars.len() && chars[i + 1].is_ascii_digit())
        {
            // Number: digits, optional decimal point, optional exponent
            let start = i;
            while i < chars.len()
                && (chars[i].is_ascii_digit()
                    || chars[i] == '.'
                    || chars[i] == 'e'
                    || chars[i] == 'E'
                    || ((chars[i] == '+' || chars[i] == '-')
                        && i > 0
                        && (chars[i - 1] == 'e' || chars[i - 1] == 'E')))
            {
                i += 1;
            }
            tokens.push(Token::new(
                TokenType::Number,
                chars[start..i].iter().collect::<String>(),
            ));
        } else if c.is_alphabetic() || c == '_' {
            // Identifier: variable or function name
            let start = i;
            while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let text: String = chars[start..i].iter().collect();
            let token_type = classify_identifier(&text);
            tokens.push(Token::new(token_type, text));
        } else if is_operator(c) {
            tokens.push(Token::new(TokenType::Operator, c.to_string()));
            i += 1;
        } else if c == '(' || c == ')' {
            tokens.push(Token::new(TokenType::Parenthesis, c.to_string()));
            i += 1;
        } else {
            // Unknown character - treat as operator
            tokens.push(Token::new(TokenType::Operator, c.to_string()));
            i += 1;
        }
    }

    tokens
}

/// Classifies an identifier as either a function, constant (number), or variable.
fn classify_identifier(text: &str) -> TokenType {
    let lower = text.to_lowercase();
    if KNOWN_FUNCTIONS.contains(&lower.as_str()) {
        TokenType::Function
    } else if KNOWN_CONSTANTS.contains(&lower.as_str()) {
        TokenType::Number
    } else {
        TokenType::Variable
    }
}

/// Checks if a character is an operator.
const fn is_operator(c: char) -> bool {
    matches!(c, '+' | '-' | '*' | '/' | '%' | '^' | '=')
}

/// Returns the style for a given token type.
///
/// Colors:
/// - Variables: Cyan
/// - Numbers: White (default)
/// - Operators: Gray (visible on both default and highlighted backgrounds)
/// - Parentheses: Default
/// - Functions: Cyan (like variables)
/// - Whitespace: Default
#[must_use]
pub fn token_style(token_type: &TokenType) -> Style {
    match token_type {
        TokenType::Variable | TokenType::Function => Style::default().fg(Color::Cyan),
        TokenType::Number => Style::default().fg(Color::White),
        TokenType::Operator => Style::default().fg(Color::Gray),
        TokenType::Parenthesis | TokenType::Whitespace => Style::default(),
    }
}

/// Converts a line of text into styled spans with syntax highlighting.
///
/// # Arguments
/// * `line` - The line of text to highlight
///
/// # Returns
/// A vector of styled spans representing the highlighted line.
#[must_use]
pub fn highlight_line(line: &str) -> Vec<Span<'_>> {
    let tokens = tokenize(line);

    // We need to return spans that reference the original line
    // So we track positions and create spans from slices
    let mut spans = Vec::new();
    let mut pos = 0;

    for token in tokens {
        let len = token.text.len();
        if pos + len <= line.len() {
            let style = token_style(&token.token_type);
            spans.push(Span::styled(&line[pos..pos + len], style));
        }
        pos += len;
    }

    spans
}

/// Converts a visible portion of a line into styled spans with syntax highlighting.
///
/// This function handles horizontal scrolling by only returning spans for the
/// visible portion of the line.
///
/// # Arguments
/// * `line` - The full line of text to highlight
/// * `horizontal_offset` - The first visible column index (0-based)
/// * `visible_width` - The number of visible columns
///
/// # Returns
/// A vector of styled spans representing the visible portion of the highlighted line.
#[must_use]
pub fn highlight_line_with_offset(
    line: &str,
    horizontal_offset: usize,
    visible_width: usize,
) -> Vec<Span<'_>> {
    if horizontal_offset >= line.len() {
        return vec![];
    }

    let tokens = tokenize(line);

    let mut spans = Vec::new();
    let mut pos = 0;
    let visible_end = (horizontal_offset + visible_width).min(line.len());

    for token in tokens {
        let token_start = pos;
        let token_end = pos + token.text.len();

        // Skip tokens entirely before visible area
        if token_end <= horizontal_offset {
            pos = token_end;
            continue;
        }

        // Stop if token starts after visible area
        if token_start >= visible_end {
            break;
        }

        // Calculate visible portion of this token
        let visible_start = token_start.max(horizontal_offset);
        let visible_token_end = token_end.min(visible_end);

        if visible_start < visible_token_end && visible_token_end <= line.len() {
            let style = token_style(&token.token_type);
            spans.push(Span::styled(&line[visible_start..visible_token_end], style));
        }

        pos = token_end;
    }

    spans
}

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================
    // Tokenizer tests - RED phase
    // ============================================================

    #[test]
    fn test_tokenize_simple_number() {
        let tokens = tokenize("42");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].text, "42");
    }

    #[test]
    fn test_tokenize_decimal_number() {
        let tokens = tokenize("3.14");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].text, "3.14");
    }

    #[test]
    fn test_tokenize_scientific_notation() {
        let tokens = tokenize("1.5e10");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].text, "1.5e10");
    }

    #[test]
    fn test_tokenize_scientific_notation_with_sign() {
        let tokens = tokenize("2.5e-3");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].text, "2.5e-3");
    }

    #[test]
    fn test_tokenize_variable() {
        let tokens = tokenize("x");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Variable);
        assert_eq!(tokens[0].text, "x");
    }

    #[test]
    fn test_tokenize_variable_multichar() {
        let tokens = tokenize("myVar");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Variable);
        assert_eq!(tokens[0].text, "myVar");
    }

    #[test]
    fn test_tokenize_variable_with_underscore() {
        let tokens = tokenize("my_var");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Variable);
        assert_eq!(tokens[0].text, "my_var");
    }

    #[test]
    fn test_tokenize_function_sqrt() {
        let tokens = tokenize("sqrt");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Function);
        assert_eq!(tokens[0].text, "sqrt");
    }

    #[test]
    fn test_tokenize_function_sin() {
        let tokens = tokenize("sin");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Function);
        assert_eq!(tokens[0].text, "sin");
    }

    #[test]
    fn test_tokenize_constant_pi() {
        let tokens = tokenize("pi");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].text, "pi");
    }

    #[test]
    fn test_tokenize_constant_e() {
        let tokens = tokenize("e");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].text, "e");
    }

    #[test]
    fn test_tokenize_operators() {
        for op in ['+', '-', '*', '/', '%', '^', '='] {
            let tokens = tokenize(&op.to_string());
            assert_eq!(tokens.len(), 1, "Operator {op}");
            assert_eq!(tokens[0].token_type, TokenType::Operator, "Operator {op}");
            assert_eq!(tokens[0].text, op.to_string(), "Operator {op}");
        }
    }

    #[test]
    fn test_tokenize_parentheses() {
        let tokens = tokenize("()");
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type, TokenType::Parenthesis);
        assert_eq!(tokens[0].text, "(");
        assert_eq!(tokens[1].token_type, TokenType::Parenthesis);
        assert_eq!(tokens[1].text, ")");
    }

    #[test]
    fn test_tokenize_whitespace() {
        let tokens = tokenize("  ");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Whitespace);
        assert_eq!(tokens[0].text, "  ");
    }

    #[test]
    fn test_tokenize_simple_expression() {
        let tokens = tokenize("5 + 3");
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].text, "5");
        assert_eq!(tokens[1].token_type, TokenType::Whitespace);
        assert_eq!(tokens[2].token_type, TokenType::Operator);
        assert_eq!(tokens[2].text, "+");
        assert_eq!(tokens[3].token_type, TokenType::Whitespace);
        assert_eq!(tokens[4].token_type, TokenType::Number);
        assert_eq!(tokens[4].text, "3");
    }

    #[test]
    fn test_tokenize_assignment() {
        let tokens = tokenize("x = 10");
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0].token_type, TokenType::Variable);
        assert_eq!(tokens[0].text, "x");
        assert_eq!(tokens[1].token_type, TokenType::Whitespace);
        assert_eq!(tokens[2].token_type, TokenType::Operator);
        assert_eq!(tokens[2].text, "=");
        assert_eq!(tokens[3].token_type, TokenType::Whitespace);
        assert_eq!(tokens[4].token_type, TokenType::Number);
        assert_eq!(tokens[4].text, "10");
    }

    #[test]
    fn test_tokenize_function_call() {
        let tokens = tokenize("sqrt(16)");
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].token_type, TokenType::Function);
        assert_eq!(tokens[0].text, "sqrt");
        assert_eq!(tokens[1].token_type, TokenType::Parenthesis);
        assert_eq!(tokens[1].text, "(");
        assert_eq!(tokens[2].token_type, TokenType::Number);
        assert_eq!(tokens[2].text, "16");
        assert_eq!(tokens[3].token_type, TokenType::Parenthesis);
        assert_eq!(tokens[3].text, ")");
    }

    #[test]
    fn test_tokenize_complex_expression() {
        let tokens = tokenize("(x + 2) * sin(pi / 4)");
        // Breakdown: (, x, ' ', +, ' ', 2, ), ' ', *, ' ', sin, (, pi, ' ', /, ' ', 4, )
        // Token indices: 0  1   2   3   4   5  6   7   8   9   10   11  12  13  14 15  16 17
        assert_eq!(tokens.len(), 18);

        assert_eq!(tokens[0].token_type, TokenType::Parenthesis);
        assert_eq!(tokens[0].text, "(");
        assert_eq!(tokens[1].token_type, TokenType::Variable);
        assert_eq!(tokens[1].text, "x");
        assert_eq!(tokens[2].token_type, TokenType::Whitespace);
        assert_eq!(tokens[3].token_type, TokenType::Operator);
        assert_eq!(tokens[3].text, "+");
        assert_eq!(tokens[4].token_type, TokenType::Whitespace);
        assert_eq!(tokens[5].token_type, TokenType::Number);
        assert_eq!(tokens[5].text, "2");
        assert_eq!(tokens[6].token_type, TokenType::Parenthesis);
        assert_eq!(tokens[6].text, ")");
        assert_eq!(tokens[7].token_type, TokenType::Whitespace);
        assert_eq!(tokens[8].token_type, TokenType::Operator);
        assert_eq!(tokens[8].text, "*");
        assert_eq!(tokens[9].token_type, TokenType::Whitespace);
        assert_eq!(tokens[10].token_type, TokenType::Function);
        assert_eq!(tokens[10].text, "sin");
        assert_eq!(tokens[11].token_type, TokenType::Parenthesis);
        assert_eq!(tokens[11].text, "(");
        assert_eq!(tokens[12].token_type, TokenType::Number); // pi is a constant
        assert_eq!(tokens[12].text, "pi");
        assert_eq!(tokens[13].token_type, TokenType::Whitespace);
        assert_eq!(tokens[14].token_type, TokenType::Operator);
        assert_eq!(tokens[14].text, "/");
        assert_eq!(tokens[15].token_type, TokenType::Whitespace);
        assert_eq!(tokens[16].token_type, TokenType::Number);
        assert_eq!(tokens[16].text, "4");
        assert_eq!(tokens[17].token_type, TokenType::Parenthesis);
        assert_eq!(tokens[17].text, ")");
    }

    #[test]
    fn test_tokenize_empty_string() {
        let tokens = tokenize("");
        assert!(tokens.is_empty());
    }

    #[test]
    fn test_tokenize_negative_number_context() {
        // In an expression like "5 + -3", the - is an operator followed by a number
        let tokens = tokenize("-3");
        // -3 starts with -, which is an operator
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type, TokenType::Operator);
        assert_eq!(tokens[0].text, "-");
        assert_eq!(tokens[1].token_type, TokenType::Number);
        assert_eq!(tokens[1].text, "3");
    }

    // ============================================================
    // Token style tests
    // ============================================================

    #[test]
    fn test_token_style_variable_is_cyan() {
        let style = token_style(&TokenType::Variable);
        assert_eq!(style.fg, Some(Color::Cyan));
    }

    #[test]
    fn test_token_style_function_is_cyan() {
        let style = token_style(&TokenType::Function);
        assert_eq!(style.fg, Some(Color::Cyan));
    }

    #[test]
    fn test_token_style_number_is_white() {
        let style = token_style(&TokenType::Number);
        assert_eq!(style.fg, Some(Color::White));
    }

    #[test]
    fn test_token_style_operator_is_dark_gray() {
        // Operators use Gray (not DarkGray) to remain visible on DarkGray highlight
        let style = token_style(&TokenType::Operator);
        assert_eq!(style.fg, Some(Color::Gray));
    }

    #[test]
    fn test_token_style_parenthesis_is_default() {
        let style = token_style(&TokenType::Parenthesis);
        assert_eq!(style.fg, None);
    }

    #[test]
    fn test_token_style_whitespace_is_default() {
        let style = token_style(&TokenType::Whitespace);
        assert_eq!(style.fg, None);
    }

    // ============================================================
    // Highlight line tests
    // ============================================================

    #[test]
    fn test_highlight_line_simple_expression() {
        let line = "5 + 3";
        let spans = highlight_line(line);

        assert_eq!(spans.len(), 5);
        // "5" - number (white)
        assert_eq!(spans[0].style.fg, Some(Color::White));
        // " " - whitespace (default)
        assert_eq!(spans[1].style.fg, None);
        // "+" - operator (gray)
        assert_eq!(spans[2].style.fg, Some(Color::Gray));
        // " " - whitespace (default)
        assert_eq!(spans[3].style.fg, None);
        // "3" - number (white)
        assert_eq!(spans[4].style.fg, Some(Color::White));
    }

    #[test]
    fn test_highlight_line_with_variable() {
        let line = "x = 10";
        let spans = highlight_line(line);

        assert_eq!(spans.len(), 5);
        // "x" - variable (cyan)
        assert_eq!(spans[0].style.fg, Some(Color::Cyan));
        // " " - whitespace (default)
        assert_eq!(spans[1].style.fg, None);
        // "=" - operator (gray)
        assert_eq!(spans[2].style.fg, Some(Color::Gray));
        // " " - whitespace (default)
        assert_eq!(spans[3].style.fg, None);
        // "10" - number (white)
        assert_eq!(spans[4].style.fg, Some(Color::White));
    }

    #[test]
    fn test_highlight_line_with_function() {
        let line = "sqrt(16)";
        let spans = highlight_line(line);

        assert_eq!(spans.len(), 4);
        // "sqrt" - function (cyan)
        assert_eq!(spans[0].style.fg, Some(Color::Cyan));
        // "(" - parenthesis (default)
        assert_eq!(spans[1].style.fg, None);
        // "16" - number (white)
        assert_eq!(spans[2].style.fg, Some(Color::White));
        // ")" - parenthesis (default)
        assert_eq!(spans[3].style.fg, None);
    }

    #[test]
    fn test_highlight_line_empty() {
        let line = "";
        let spans = highlight_line(line);
        assert!(spans.is_empty());
    }

    #[test]
    fn test_highlight_line_preserves_text() {
        let line = "x + y";
        let spans = highlight_line(line);

        let reconstructed: String = spans.iter().map(|s| s.content.as_ref()).collect();
        assert_eq!(reconstructed, line);
    }

    // ============================================================
    // Highlight line with offset tests
    // ============================================================

    #[test]
    fn test_highlight_line_with_offset_returns_visible_portion() {
        let line = "0123456789abcdef";
        let spans = highlight_line_with_offset(line, 5, 5);

        // Should return spans for positions 5-9 ("56789")
        let reconstructed: String = spans.iter().map(|s| s.content.as_ref()).collect();
        assert_eq!(reconstructed, "56789");
    }

    #[test]
    fn test_highlight_line_with_offset_zero_starts_from_beginning() {
        let line = "abc";
        let spans = highlight_line_with_offset(line, 0, 10);

        let reconstructed: String = spans.iter().map(|s| s.content.as_ref()).collect();
        assert_eq!(reconstructed, "abc");
    }

    #[test]
    fn test_highlight_line_with_offset_beyond_line_length() {
        let line = "abc";
        let spans = highlight_line_with_offset(line, 10, 5);

        // Offset beyond line length should return empty
        assert!(spans.is_empty());
    }

    #[test]
    fn test_highlight_line_with_offset_partial_token_at_start() {
        // Line: "x = 10" (positions: x=0, space=1, ==2, space=3, 1=4, 0=5)
        // Offset 2 should start from "= 10"
        let line = "x = 10";
        let spans = highlight_line_with_offset(line, 2, 10);

        let reconstructed: String = spans.iter().map(|s| s.content.as_ref()).collect();
        assert_eq!(reconstructed, "= 10");
    }

    #[test]
    fn test_highlight_line_with_offset_partial_token_at_end() {
        // Line: "x = 10" - visible width cuts off part of the line
        let line = "x = 10";
        let spans = highlight_line_with_offset(line, 0, 4);

        // Should show "x = " (positions 0-3)
        let reconstructed: String = spans.iter().map(|s| s.content.as_ref()).collect();
        assert_eq!(reconstructed, "x = ");
    }

    #[test]
    fn test_highlight_line_with_offset_preserves_syntax_highlighting() {
        // Line: "sqrt(16)" - check that function is still cyan colored
        let line = "sqrt(16)";
        let spans = highlight_line_with_offset(line, 0, 8);

        // First span should be "sqrt" with cyan color (function)
        assert_eq!(spans[0].content.as_ref(), "sqrt");
        assert_eq!(spans[0].style.fg, Some(Color::Cyan));
    }

    #[test]
    fn test_highlight_line_with_offset_empty_line() {
        let line = "";
        let spans = highlight_line_with_offset(line, 0, 10);
        assert!(spans.is_empty());
    }

    #[test]
    fn test_highlight_line_with_offset_offset_at_token_boundary() {
        // Line: "5 + 3" - offset at position 2 (the '+')
        let line = "5 + 3";
        let spans = highlight_line_with_offset(line, 2, 10);

        let reconstructed: String = spans.iter().map(|s| s.content.as_ref()).collect();
        assert_eq!(reconstructed, "+ 3");
    }
}
