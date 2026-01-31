//! Tokenizer for mathematical expressions.
//!
//! This module provides lexical analysis for math expressions, converting
//! input strings into a sequence of tokens for the parser.

use std::fmt;

/// A span indicating the position of a token in the source string.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    /// Start position (byte offset, inclusive).
    pub start: usize,
    /// End position (byte offset, exclusive).
    pub end: usize,
}

impl Span {
    /// Creates a new span.
    #[must_use]
    pub const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

/// A value paired with its span in the source.
pub type Spanned<T> = (T, Span);

/// Token types for mathematical expressions.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// A numeric literal (integer or float).
    Number(f64),
    /// An identifier (variable or function name).
    Identifier(String),
    /// Addition operator `+`.
    Plus,
    /// Subtraction operator `-`.
    Minus,
    /// Multiplication operator `*`.
    Star,
    /// Division operator `/`.
    Slash,
    /// Modulo operator `%`.
    Percent,
    /// Power operator `^`.
    Caret,
    /// Left parenthesis `(`.
    LParen,
    /// Right parenthesis `)`.
    RParen,
    /// Comma `,`.
    Comma,
    /// Equals sign `=`.
    Equals,
    /// Factorial operator '!'
    Exclaim,
}

/// An error that occurred during tokenization.
#[derive(Debug, Clone)]
pub struct TokenError {
    /// Description of the error.
    pub message: String,
    /// Position in the input where the error occurred.
    pub position: usize,
}

impl TokenError {
    /// Creates a new token error.
    pub fn new(message: impl Into<String>, position: usize) -> Self {
        Self {
            message: message.into(),
            position,
        }
    }
}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "at position {}: {}", self.position, self.message)
    }
}

impl std::error::Error for TokenError {}

/// Tokenizer for mathematical expressions.
pub struct Tokenizer<'a> {
    /// The input string being tokenized.
    input: &'a str,
    /// Current position in the input (byte offset).
    position: usize,
}

impl<'a> Tokenizer<'a> {
    /// Creates a new tokenizer for the given input.
    #[must_use]
    pub const fn new(input: &'a str) -> Self {
        Self { input, position: 0 }
    }

    /// Tokenizes the entire input and returns a vector of spanned tokens.
    ///
    /// # Errors
    ///
    /// Returns a `TokenError` if the input contains invalid characters or
    /// malformed numbers (e.g., incomplete scientific notation).
    pub fn tokenize(&mut self) -> Result<Vec<Spanned<Token>>, TokenError> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            self.skip_whitespace();
            if self.is_at_end() {
                break;
            }

            let token = self.next_token()?;
            tokens.push(token);
        }

        Ok(tokens)
    }

    /// Returns true if we've reached the end of the input.
    const fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }

    /// Returns the current character without consuming it.
    fn peek(&self) -> Option<char> {
        self.input[self.position..].chars().next()
    }

    /// Returns the next character without consuming it (lookahead by 1).
    fn peek_next(&self) -> Option<char> {
        let mut chars = self.input[self.position..].chars();
        chars.next();
        chars.next()
    }

    /// Consumes and returns the current character.
    fn advance(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.position += c.len_utf8();
        Some(c)
    }

    /// Skips whitespace characters.
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Parses the next token from the input.
    fn next_token(&mut self) -> Result<Spanned<Token>, TokenError> {
        let start = self.position;
        let c = self
            .peek()
            .ok_or_else(|| TokenError::new("unexpected end of input", self.position))?;

        // Single-character tokens
        let token = match c {
            '+' => {
                self.advance();
                Token::Plus
            }
            '-' => {
                self.advance();
                Token::Minus
            }
            '*' => {
                self.advance();
                Token::Star
            }
            '/' => {
                self.advance();
                Token::Slash
            }
            '%' => {
                self.advance();
                Token::Percent
            }
            '^' => {
                self.advance();
                Token::Caret
            }
            '(' => {
                self.advance();
                Token::LParen
            }
            ')' => {
                self.advance();
                Token::RParen
            }
            ',' => {
                self.advance();
                Token::Comma
            }
            '=' => {
                self.advance();
                Token::Equals
            }
            '!' => {
                self.advance();
                Token::Exclaim
            }
            c if c.is_ascii_digit() || c == '.' => self.parse_number()?,
            c if c.is_alphabetic() || c == '_' => self.parse_identifier(),
            _ => {
                return Err(TokenError::new(
                    format!("unexpected character '{c}'"),
                    self.position,
                ));
            }
        };

        let end = self.position;
        Ok((token, Span::new(start, end)))
    }

    /// Parses a number (integer, float, scientific notation, or base-prefix literal).
    fn parse_number(&mut self) -> Result<Token, TokenError> {
        let start = self.position;
        let mut has_dot = false;

        // Handle leading dot (e.g., .5)
        if self.peek() == Some('.') {
            has_dot = true;
            self.advance();
            // Must have at least one digit after the dot
            if !self.peek().is_some_and(|c| c.is_ascii_digit()) {
                return Err(TokenError::new(
                    "expected digit after decimal point",
                    self.position,
                ));
            }
        }

        // Check for base-prefix literals (0x, 0b, 0o)
        if !has_dot
            && self.peek() == Some('0')
            && let Some(prefix) = self.peek_next()
        {
            match prefix {
                'x' | 'X' => {
                    self.advance(); // consume '0'
                    self.advance(); // consume 'x'/'X'
                    return self.parse_base_digits(16, "hex", start);
                }
                'b' | 'B' => {
                    self.advance(); // consume '0'
                    self.advance(); // consume 'b'/'B'
                    return self.parse_base_digits(2, "binary", start);
                }
                'o' | 'O' => {
                    self.advance(); // consume '0'
                    self.advance(); // consume 'o'/'O'
                    return self.parse_base_digits(8, "octal", start);
                }
                _ => {}
            }
        }

        // Parse integer part
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }

        // Parse decimal part
        if !has_dot && self.peek() == Some('.') {
            // Check that the next character is a digit (to avoid "1." being valid)
            if self.peek_next().is_some_and(|c| c.is_ascii_digit()) {
                self.advance(); // consume the dot
                while let Some(c) = self.peek() {
                    if c.is_ascii_digit() {
                        self.advance();
                    } else {
                        break;
                    }
                }
            }
        }

        // Parse exponent part (e.g., 1e10, 1E-5)
        if let Some(c) = self.peek()
            && (c == 'e' || c == 'E')
        {
            self.advance();

            // Optional sign
            if let Some(sign) = self.peek()
                && (sign == '+' || sign == '-')
            {
                self.advance();
            }

            // Must have at least one digit in exponent
            if !self.peek().is_some_and(|c| c.is_ascii_digit()) {
                return Err(TokenError::new("expected digit in exponent", self.position));
            }

            while let Some(c) = self.peek() {
                if c.is_ascii_digit() {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        let number_str = &self.input[start..self.position];
        let value: f64 = number_str
            .parse()
            .map_err(|_| TokenError::new(format!("invalid number '{number_str}'"), start))?;

        Ok(Token::Number(value))
    }

    /// Parses digits in a given base after a prefix (e.g., after `0x`).
    #[allow(clippy::cast_precision_loss)]
    fn parse_base_digits(
        &mut self,
        base: u32,
        base_name: &str,
        start: usize,
    ) -> Result<Token, TokenError> {
        let digits_start = self.position;

        // Must have at least one valid digit
        if !self.peek().is_some_and(|c| c.is_digit(base)) {
            return Err(TokenError::new(
                format!(
                    "expected {base_name} digit after '0{}'",
                    self.input[start + 1..start + 2].chars().next().unwrap()
                ),
                self.position,
            ));
        }

        // Consume valid digits for the base
        while let Some(c) = self.peek() {
            if c.is_digit(base) {
                self.advance();
            } else if c.is_alphanumeric() {
                // Invalid digit for this base (e.g., '2' in 0b12)
                return Err(TokenError::new(
                    format!("invalid digit '{c}' in {base_name} literal"),
                    self.position,
                ));
            } else {
                break;
            }
        }

        let digit_str = &self.input[digits_start..self.position];
        let value = i64::from_str_radix(digit_str, base)
            .map_err(|_| TokenError::new(format!("invalid {base_name} literal"), start))?;

        Ok(Token::Number(value as f64))
    }

    /// Parses an identifier (variable or function name).
    fn parse_identifier(&mut self) -> Token {
        let start = self.position;

        // First character must be alphabetic or underscore
        self.advance();

        // Subsequent characters can be alphanumeric or underscore
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let name = self.input[start..self.position].to_string();
        Token::Identifier(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tokenize(input: &str) -> Result<Vec<Token>, TokenError> {
        let mut tokenizer = Tokenizer::new(input);
        tokenizer
            .tokenize()
            .map(|v| v.into_iter().map(|(t, _)| t).collect())
    }

    #[test]
    fn test_tokenize_integer() {
        let tokens = tokenize("42").unwrap();
        assert_eq!(tokens, vec![Token::Number(42.0)]);
    }

    #[test]
    fn test_tokenize_zero() {
        let tokens = tokenize("0").unwrap();
        assert_eq!(tokens, vec![Token::Number(0.0)]);
    }

    #[test]
    fn test_tokenize_large_integer() {
        let tokens = tokenize("1234567890").unwrap();
        assert_eq!(tokens, vec![Token::Number(1_234_567_890.0)]);
    }

    #[test]
    fn test_tokenize_float() {
        let tokens = tokenize("3.25").unwrap();
        assert_eq!(tokens, vec![Token::Number(3.25)]);
    }

    #[test]
    fn test_tokenize_float_with_trailing_zeros() {
        let tokens = tokenize("2.50").unwrap();
        assert_eq!(tokens, vec![Token::Number(2.5)]);
    }

    #[test]
    fn test_tokenize_float_leading_dot() {
        let tokens = tokenize(".5").unwrap();
        assert_eq!(tokens, vec![Token::Number(0.5)]);
    }

    #[test]
    fn test_tokenize_scientific_notation_positive() {
        let tokens = tokenize("1e10").unwrap();
        assert_eq!(tokens, vec![Token::Number(1e10)]);
    }

    #[test]
    fn test_tokenize_scientific_notation_negative_exponent() {
        let tokens = tokenize("1e-5").unwrap();
        assert_eq!(tokens, vec![Token::Number(1e-5)]);
    }

    #[test]
    fn test_tokenize_scientific_notation_positive_exponent() {
        let tokens = tokenize("2.5e+3").unwrap();
        assert_eq!(tokens, vec![Token::Number(2500.0)]);
    }

    #[test]
    fn test_tokenize_scientific_notation_uppercase() {
        let tokens = tokenize("1E10").unwrap();
        assert_eq!(tokens, vec![Token::Number(1e10)]);
    }

    #[test]
    fn test_tokenize_scientific_notation_float() {
        let tokens = tokenize("6.022e23").unwrap();
        assert_eq!(tokens, vec![Token::Number(6.022e23)]);
    }

    #[test]
    fn test_tokenize_simple_identifier() {
        let tokens = tokenize("x").unwrap();
        assert_eq!(tokens, vec![Token::Identifier("x".to_string())]);
    }

    #[test]
    fn test_tokenize_longer_identifier() {
        let tokens = tokenize("variable").unwrap();
        assert_eq!(tokens, vec![Token::Identifier("variable".to_string())]);
    }

    #[test]
    fn test_tokenize_identifier_with_numbers() {
        let tokens = tokenize("x1").unwrap();
        assert_eq!(tokens, vec![Token::Identifier("x1".to_string())]);
    }

    #[test]
    fn test_tokenize_identifier_with_underscore() {
        let tokens = tokenize("my_var").unwrap();
        assert_eq!(tokens, vec![Token::Identifier("my_var".to_string())]);
    }

    #[test]
    fn test_tokenize_identifier_starting_with_underscore() {
        let tokens = tokenize("_private").unwrap();
        assert_eq!(tokens, vec![Token::Identifier("_private".to_string())]);
    }

    #[test]
    fn test_tokenize_identifier_uppercase() {
        let tokens = tokenize("PI").unwrap();
        assert_eq!(tokens, vec![Token::Identifier("PI".to_string())]);
    }

    #[test]
    fn test_tokenize_identifier_mixed_case() {
        let tokens = tokenize("camelCase").unwrap();
        assert_eq!(tokens, vec![Token::Identifier("camelCase".to_string())]);
    }

    #[test]
    fn test_tokenize_plus() {
        let tokens = tokenize("+").unwrap();
        assert_eq!(tokens, vec![Token::Plus]);
    }

    #[test]
    fn test_tokenize_minus() {
        let tokens = tokenize("-").unwrap();
        assert_eq!(tokens, vec![Token::Minus]);
    }

    #[test]
    fn test_tokenize_star() {
        let tokens = tokenize("*").unwrap();
        assert_eq!(tokens, vec![Token::Star]);
    }

    #[test]
    fn test_tokenize_slash() {
        let tokens = tokenize("/").unwrap();
        assert_eq!(tokens, vec![Token::Slash]);
    }

    #[test]
    fn test_tokenize_percent() {
        let tokens = tokenize("%").unwrap();
        assert_eq!(tokens, vec![Token::Percent]);
    }

    #[test]
    fn test_tokenize_caret() {
        let tokens = tokenize("^").unwrap();
        assert_eq!(tokens, vec![Token::Caret]);
    }

    #[test]
    fn test_tokenize_lparen() {
        let tokens = tokenize("(").unwrap();
        assert_eq!(tokens, vec![Token::LParen]);
    }

    #[test]
    fn test_tokenize_rparen() {
        let tokens = tokenize(")").unwrap();
        assert_eq!(tokens, vec![Token::RParen]);
    }

    #[test]
    fn test_tokenize_comma() {
        let tokens = tokenize(",").unwrap();
        assert_eq!(tokens, vec![Token::Comma]);
    }

    #[test]
    fn test_tokenize_equals() {
        let tokens = tokenize("=").unwrap();
        assert_eq!(tokens, vec![Token::Equals]);
    }

    #[test]
    fn test_tokenize_simple_addition() {
        let tokens = tokenize("2 + 3").unwrap();
        assert_eq!(
            tokens,
            vec![Token::Number(2.0), Token::Plus, Token::Number(3.0),]
        );
    }

    #[test]
    fn test_tokenize_expression_with_precedence() {
        let tokens = tokenize("2 + 3 * 4").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Number(2.0),
                Token::Plus,
                Token::Number(3.0),
                Token::Star,
                Token::Number(4.0),
            ]
        );
    }

    #[test]
    fn test_tokenize_expression_with_parentheses() {
        let tokens = tokenize("(2 + 3) * 4").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::Number(2.0),
                Token::Plus,
                Token::Number(3.0),
                Token::RParen,
                Token::Star,
                Token::Number(4.0),
            ]
        );
    }

    #[test]
    fn test_tokenize_expression_with_power() {
        let tokens = tokenize("2^3").unwrap();
        assert_eq!(
            tokens,
            vec![Token::Number(2.0), Token::Caret, Token::Number(3.0),]
        );
    }

    #[test]
    fn test_tokenize_function_call() {
        let tokens = tokenize("sqrt(16)").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Identifier("sqrt".to_string()),
                Token::LParen,
                Token::Number(16.0),
                Token::RParen,
            ]
        );
    }

    #[test]
    fn test_tokenize_function_call_with_multiple_args() {
        let tokens = tokenize("pow(2, 10)").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Identifier("pow".to_string()),
                Token::LParen,
                Token::Number(2.0),
                Token::Comma,
                Token::Number(10.0),
                Token::RParen,
            ]
        );
    }

    #[test]
    fn test_tokenize_nested_function_call() {
        let tokens = tokenize("sqrt(abs(-16))").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Identifier("sqrt".to_string()),
                Token::LParen,
                Token::Identifier("abs".to_string()),
                Token::LParen,
                Token::Minus,
                Token::Number(16.0),
                Token::RParen,
                Token::RParen,
            ]
        );
    }

    #[test]
    fn test_tokenize_assignment() {
        let tokens = tokenize("x = 42").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Identifier("x".to_string()),
                Token::Equals,
                Token::Number(42.0),
            ]
        );
    }

    #[test]
    fn test_tokenize_complex_expression() {
        let tokens = tokenize("a + b * c - d / e").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Identifier("a".to_string()),
                Token::Plus,
                Token::Identifier("b".to_string()),
                Token::Star,
                Token::Identifier("c".to_string()),
                Token::Minus,
                Token::Identifier("d".to_string()),
                Token::Slash,
                Token::Identifier("e".to_string()),
            ]
        );
    }

    #[test]
    fn test_tokenize_expression_no_spaces() {
        let tokens = tokenize("2+3*4").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Number(2.0),
                Token::Plus,
                Token::Number(3.0),
                Token::Star,
                Token::Number(4.0),
            ]
        );
    }

    #[test]
    fn test_tokenize_modulo_expression() {
        let tokens = tokenize("17 % 5").unwrap();
        assert_eq!(
            tokens,
            vec![Token::Number(17.0), Token::Percent, Token::Number(5.0),]
        );
    }

    #[test]
    fn test_tokenize_negative_number_expression() {
        let tokens = tokenize("-5").unwrap();
        assert_eq!(tokens, vec![Token::Minus, Token::Number(5.0),]);
    }

    #[test]
    fn test_tokenize_unary_minus_in_expression() {
        let tokens = tokenize("3 + -2").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Number(3.0),
                Token::Plus,
                Token::Minus,
                Token::Number(2.0),
            ]
        );
    }

    #[test]
    fn test_tokenize_empty_string() {
        let tokens = tokenize("").unwrap();
        assert_eq!(tokens, vec![]);
    }

    #[test]
    fn test_tokenize_only_whitespace() {
        let tokens = tokenize("   ").unwrap();
        assert_eq!(tokens, vec![]);
    }

    #[test]
    fn test_tokenize_with_tabs() {
        let tokens = tokenize("2\t+\t3").unwrap();
        assert_eq!(
            tokens,
            vec![Token::Number(2.0), Token::Plus, Token::Number(3.0),]
        );
    }

    #[test]
    fn test_tokenize_with_newlines() {
        let tokens = tokenize("2\n+\n3").unwrap();
        assert_eq!(
            tokens,
            vec![Token::Number(2.0), Token::Plus, Token::Number(3.0),]
        );
    }

    #[test]
    fn test_tokenize_leading_whitespace() {
        let tokens = tokenize("   42").unwrap();
        assert_eq!(tokens, vec![Token::Number(42.0)]);
    }

    #[test]
    fn test_tokenize_trailing_whitespace() {
        let tokens = tokenize("42   ").unwrap();
        assert_eq!(tokens, vec![Token::Number(42.0)]);
    }

    #[test]
    fn test_span_single_digit() {
        let mut tokenizer = Tokenizer::new("5");
        let tokens = tokenizer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].1, Span::new(0, 1));
    }

    #[test]
    fn test_span_multi_digit() {
        let mut tokenizer = Tokenizer::new("123");
        let tokens = tokenizer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].1, Span::new(0, 3));
    }

    #[test]
    fn test_span_with_whitespace() {
        let mut tokenizer = Tokenizer::new("  42  ");
        let tokens = tokenizer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].1, Span::new(2, 4));
    }

    #[test]
    fn test_span_expression() {
        let mut tokenizer = Tokenizer::new("2 + 3");
        let tokens = tokenizer.tokenize().unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].1, Span::new(0, 1)); // "2"
        assert_eq!(tokens[1].1, Span::new(2, 3)); // "+"
        assert_eq!(tokens[2].1, Span::new(4, 5)); // "3"
    }

    #[test]
    fn test_span_identifier() {
        let mut tokenizer = Tokenizer::new("sqrt");
        let tokens = tokenizer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].1, Span::new(0, 4));
    }

    #[test]
    fn test_error_invalid_character() {
        let result = tokenize("2 @ 3");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("unexpected character"));
        assert_eq!(err.position, 2);
    }

    #[test]
    fn test_error_invalid_character_hash() {
        let result = tokenize("x # y");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("unexpected character"));
    }

    #[test]
    fn test_error_dot_without_digit() {
        let result = tokenize(".");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("digit"));
    }

    #[test]
    fn test_error_incomplete_exponent() {
        let result = tokenize("1e");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("exponent"));
    }

    #[test]
    fn test_error_incomplete_exponent_with_sign() {
        let result = tokenize("1e+");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("exponent"));
    }

    #[test]
    fn test_tokenize_multiple_operators() {
        let tokens = tokenize("+-*/^%").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Plus,
                Token::Minus,
                Token::Star,
                Token::Slash,
                Token::Caret,
                Token::Percent,
            ]
        );
    }

    #[test]
    fn test_tokenize_parentheses_sequence() {
        let tokens = tokenize("(())").unwrap();
        assert_eq!(
            tokens,
            vec![Token::LParen, Token::LParen, Token::RParen, Token::RParen,]
        );
    }

    #[test]
    fn test_tokenize_scientific_with_identifier_like_suffix() {
        // "1e2" should be parsed as scientific notation, not "1" followed by "e2"
        let tokens = tokenize("1e2").unwrap();
        assert_eq!(tokens, vec![Token::Number(100.0)]);
    }

    #[test]
    fn test_tokenize_identifier_followed_by_number() {
        let tokens = tokenize("x 1").unwrap();
        assert_eq!(
            tokens,
            vec![Token::Identifier("x".to_string()), Token::Number(1.0),]
        );
    }

    #[test]
    fn test_tokenize_pi_constant() {
        let tokens = tokenize("pi").unwrap();
        assert_eq!(tokens, vec![Token::Identifier("pi".to_string())]);
    }

    #[test]
    fn test_tokenize_e_constant() {
        // "e" alone should be an identifier, not scientific notation
        let tokens = tokenize("e").unwrap();
        assert_eq!(tokens, vec![Token::Identifier("e".to_string())]);
    }

    #[test]
    fn test_tokenize_hex_literal() {
        let tokens = tokenize("0xff").unwrap();
        assert_eq!(tokens, vec![Token::Number(255.0)]);
    }

    #[test]
    fn test_tokenize_hex_literal_uppercase_prefix() {
        let tokens = tokenize("0Xff").unwrap();
        assert_eq!(tokens, vec![Token::Number(255.0)]);
    }

    #[test]
    fn test_tokenize_hex_literal_uppercase_digits() {
        let tokens = tokenize("0xABCD").unwrap();
        assert_eq!(tokens, vec![Token::Number(0xABCD as f64)]);
    }

    #[test]
    fn test_tokenize_binary_literal() {
        let tokens = tokenize("0b1010").unwrap();
        assert_eq!(tokens, vec![Token::Number(10.0)]);
    }

    #[test]
    fn test_tokenize_binary_literal_uppercase_prefix() {
        let tokens = tokenize("0B1100").unwrap();
        assert_eq!(tokens, vec![Token::Number(12.0)]);
    }

    #[test]
    fn test_tokenize_octal_literal() {
        let tokens = tokenize("0o77").unwrap();
        assert_eq!(tokens, vec![Token::Number(63.0)]);
    }

    #[test]
    fn test_tokenize_octal_literal_uppercase_prefix() {
        let tokens = tokenize("0O10").unwrap();
        assert_eq!(tokens, vec![Token::Number(8.0)]);
    }

    #[test]
    fn test_tokenize_hex_no_digits_error() {
        let result = tokenize("0x");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("expected hex digit"));
    }

    #[test]
    fn test_tokenize_binary_no_digits_error() {
        let result = tokenize("0b");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("expected binary digit"));
    }

    #[test]
    fn test_tokenize_octal_no_digits_error() {
        let result = tokenize("0o");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("expected octal digit"));
    }

    #[test]
    fn test_tokenize_binary_invalid_digit_error() {
        let result = tokenize("0b123");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("invalid digit"));
    }

    #[test]
    fn test_tokenize_octal_invalid_digit_error() {
        // 8 is not a valid octal digit, so 0o8 fails immediately
        let result = tokenize("0o89");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("expected octal digit"));
    }

    #[test]
    fn test_tokenize_octal_invalid_digit_after_valid() {
        // 7 is valid octal, but 8 is not
        let result = tokenize("0o78");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("invalid digit"));
    }

    #[test]
    fn test_tokenize_hex_zero() {
        let tokens = tokenize("0x0").unwrap();
        assert_eq!(tokens, vec![Token::Number(0.0)]);
    }

    #[test]
    fn test_tokenize_hex_in_expression() {
        let tokens = tokenize("0xff + 1").unwrap();
        assert_eq!(
            tokens,
            vec![Token::Number(255.0), Token::Plus, Token::Number(1.0)]
        );
    }

    #[test]
    fn test_tokenize_exclaim() {
        let tokens = tokenize("5!").unwrap();
        assert_eq!(tokens, vec![Token::Number(5.0), Token::Exclaim]);
    }

    #[test]
    fn test_tokenize_exclaim_in_expression() {
        let tokens = tokenize("3! + 1").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Number(3.0),
                Token::Exclaim,
                Token::Plus,
                Token::Number(1.0)
            ]
        );
    }
}
