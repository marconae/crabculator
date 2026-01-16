//! Abstract Syntax Tree and parser for mathematical expressions.
//!
//! This module provides the AST types and a recursive descent parser
//! for parsing tokenized math expressions.

use crate::eval::token::{Span, Spanned, Token};

/// Binary operators for arithmetic expressions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
}

/// AST node representing a mathematical expression.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// A numeric literal.
    Number(f64),
    /// A variable reference.
    Variable(String),
    /// A binary operation.
    BinaryOp {
        left: Box<Self>,
        op: BinaryOp,
        right: Box<Self>,
    },
    /// Unary negation.
    UnaryMinus(Box<Self>),
    /// A function call with arguments.
    FunctionCall { name: String, args: Vec<Self> },
}

/// Error from parsing.
#[derive(Debug, Clone)]
pub struct ParseError {
    /// Human-readable error message.
    pub message: String,
    /// Optional span indicating where the error occurred (start, end).
    pub span: Option<(usize, usize)>,
}

impl ParseError {
    /// Creates a new parse error with a message.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            span: None,
        }
    }

    /// Creates a new parse error with a message and span.
    pub fn with_span(message: impl Into<String>, span: Span) -> Self {
        Self {
            message: message.into(),
            span: Some((span.start, span.end)),
        }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ParseError {}

/// Recursive descent parser for mathematical expressions.
///
/// Grammar (precedence from low to high):
/// ```text
/// expr       -> term (('+' | '-') term)*
/// term       -> power (('*' | '/' | '%') power)*
/// power      -> unary ('^' power)?     // right-associative via recursion
/// unary      -> '-' unary | primary
/// primary    -> NUMBER | IDENTIFIER | IDENTIFIER '(' args ')' | '(' expr ')'
/// args       -> expr (',' expr)* | empty
/// ```
pub struct Parser {
    tokens: Vec<Spanned<Token>>,
    pos: usize,
}

impl Parser {
    /// Creates a new parser with the given tokens.
    #[must_use]
    pub const fn new(tokens: Vec<Spanned<Token>>) -> Self {
        Self { tokens, pos: 0 }
    }

    /// Parses the tokens into an expression AST.
    ///
    /// # Errors
    ///
    /// Returns a `ParseError` if the token stream is invalid or cannot be parsed
    /// into a valid expression. This includes syntax errors, unexpected tokens,
    /// and unclosed parentheses.
    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        let expr = self.parse_expr()?;

        // Ensure we consumed all tokens
        if !self.is_at_end() {
            let (token, span) = &self.tokens[self.pos];
            return Err(ParseError::with_span(
                format!("Unexpected token: {token:?}"),
                *span,
            ));
        }

        Ok(expr)
    }

    // Parse addition and subtraction (lowest precedence)
    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_term()?;

        while let Some(op) = self.match_additive_op() {
            let right = self.parse_term()?;
            left = Expr::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    // Parse multiplication, division, modulo
    fn parse_term(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_power()?;

        while let Some(op) = self.match_multiplicative_op() {
            let right = self.parse_power()?;
            left = Expr::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    // Parse exponentiation (right-associative)
    fn parse_power(&mut self) -> Result<Expr, ParseError> {
        let base = self.parse_unary()?;

        if self.match_token(&Token::Caret) {
            // Right-associative: recurse for the exponent
            let exponent = self.parse_power()?;
            Ok(Expr::BinaryOp {
                left: Box::new(base),
                op: BinaryOp::Pow,
                right: Box::new(exponent),
            })
        } else {
            Ok(base)
        }
    }

    // Parse unary minus
    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token(&Token::Minus) {
            let operand = self.parse_unary()?;
            Ok(Expr::UnaryMinus(Box::new(operand)))
        } else {
            self.parse_primary()
        }
    }

    // Parse primary expressions: numbers, variables, function calls, parentheses
    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        if self.is_at_end() {
            return Err(ParseError::new("Unexpected end of expression"));
        }

        let (token, span) = self.tokens[self.pos].clone();

        match &token {
            Token::Number(n) => {
                self.advance();
                Ok(Expr::Number(*n))
            }
            Token::Identifier(name) => {
                let name = name.clone();
                self.advance();

                // Check if it's a function call
                if self.match_token(&Token::LParen) {
                    let args = self.parse_args()?;
                    if !self.match_token(&Token::RParen) {
                        return Err(ParseError::new("Expected ')' after function arguments"));
                    }
                    Ok(Expr::FunctionCall { name, args })
                } else {
                    Ok(Expr::Variable(name))
                }
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_expr()?;
                if !self.match_token(&Token::RParen) {
                    return Err(ParseError::new("Expected ')' after expression"));
                }
                Ok(expr)
            }
            _ => Err(ParseError::with_span(
                format!("Unexpected token: {token:?}"),
                span,
            )),
        }
    }

    // Parse function arguments
    fn parse_args(&mut self) -> Result<Vec<Expr>, ParseError> {
        let mut args = Vec::new();

        // Check for empty argument list
        if self.check(&Token::RParen) {
            return Ok(args);
        }

        // Parse first argument
        args.push(self.parse_expr()?);

        // Parse remaining arguments
        while self.match_token(&Token::Comma) {
            args.push(self.parse_expr()?);
        }

        Ok(args)
    }

    // Helper methods for token manipulation

    const fn is_at_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    const fn advance(&mut self) {
        if !self.is_at_end() {
            self.pos += 1;
        }
    }

    fn check(&self, token: &Token) -> bool {
        if self.is_at_end() {
            return false;
        }
        let (current, _) = &self.tokens[self.pos];
        std::mem::discriminant(current) == std::mem::discriminant(token)
    }

    fn match_token(&mut self, token: &Token) -> bool {
        if self.check(token) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn match_additive_op(&mut self) -> Option<BinaryOp> {
        if self.is_at_end() {
            return None;
        }
        let (token, _) = &self.tokens[self.pos];
        match token {
            Token::Plus => {
                self.advance();
                Some(BinaryOp::Add)
            }
            Token::Minus => {
                self.advance();
                Some(BinaryOp::Sub)
            }
            _ => None,
        }
    }

    fn match_multiplicative_op(&mut self) -> Option<BinaryOp> {
        if self.is_at_end() {
            return None;
        }
        let (token, _) = &self.tokens[self.pos];
        match token {
            Token::Star => {
                self.advance();
                Some(BinaryOp::Mul)
            }
            Token::Slash => {
                self.advance();
                Some(BinaryOp::Div)
            }
            Token::Percent => {
                self.advance();
                Some(BinaryOp::Mod)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to create a spanned token at a position
    fn spanned(token: Token, start: usize, end: usize) -> Spanned<Token> {
        (token, Span { start, end })
    }

    // Helper to create tokens without worrying about spans
    fn tok(token: Token) -> Spanned<Token> {
        spanned(token, 0, 0)
    }

    // ============================================================
    // Tests for parsing simple numbers
    // ============================================================

    #[test]
    fn test_parse_integer() {
        let tokens = vec![tok(Token::Number(42.0))];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert_eq!(result.unwrap(), Expr::Number(42.0));
    }

    #[test]
    fn test_parse_float() {
        let tokens = vec![tok(Token::Number(3.25))];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert_eq!(result.unwrap(), Expr::Number(3.25));
    }

    #[test]
    fn test_parse_negative_number_literal() {
        // -5 is parsed as UnaryMinus(Number(5))
        let tokens = vec![tok(Token::Minus), tok(Token::Number(5.0))];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert_eq!(
            result.unwrap(),
            Expr::UnaryMinus(Box::new(Expr::Number(5.0)))
        );
    }

    // ============================================================
    // Tests for parsing binary operations with correct precedence
    // ============================================================

    #[test]
    fn test_parse_simple_addition() {
        // 1 + 2
        let tokens = vec![
            tok(Token::Number(1.0)),
            tok(Token::Plus),
            tok(Token::Number(2.0)),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert_eq!(
            result.unwrap(),
            Expr::BinaryOp {
                left: Box::new(Expr::Number(1.0)),
                op: BinaryOp::Add,
                right: Box::new(Expr::Number(2.0)),
            }
        );
    }

    #[test]
    fn test_parse_simple_subtraction() {
        // 5 - 3
        let tokens = vec![
            tok(Token::Number(5.0)),
            tok(Token::Minus),
            tok(Token::Number(3.0)),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert_eq!(
            result.unwrap(),
            Expr::BinaryOp {
                left: Box::new(Expr::Number(5.0)),
                op: BinaryOp::Sub,
                right: Box::new(Expr::Number(3.0)),
            }
        );
    }

    #[test]
    fn test_parse_simple_multiplication() {
        // 2 * 3
        let tokens = vec![
            tok(Token::Number(2.0)),
            tok(Token::Star),
            tok(Token::Number(3.0)),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert_eq!(
            result.unwrap(),
            Expr::BinaryOp {
                left: Box::new(Expr::Number(2.0)),
                op: BinaryOp::Mul,
                right: Box::new(Expr::Number(3.0)),
            }
        );
    }

    #[test]
    fn test_parse_simple_division() {
        // 10 / 2
        let tokens = vec![
            tok(Token::Number(10.0)),
            tok(Token::Slash),
            tok(Token::Number(2.0)),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert_eq!(
            result.unwrap(),
            Expr::BinaryOp {
                left: Box::new(Expr::Number(10.0)),
                op: BinaryOp::Div,
                right: Box::new(Expr::Number(2.0)),
            }
        );
    }

    #[test]
    fn test_parse_modulo() {
        // 7 % 3
        let tokens = vec![
            tok(Token::Number(7.0)),
            tok(Token::Percent),
            tok(Token::Number(3.0)),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert_eq!(
            result.unwrap(),
            Expr::BinaryOp {
                left: Box::new(Expr::Number(7.0)),
                op: BinaryOp::Mod,
                right: Box::new(Expr::Number(3.0)),
            }
        );
    }

    #[test]
    fn test_parse_precedence_mul_over_add() {
        // 1 + 2 * 3 = 1 + (2 * 3)
        let tokens = vec![
            tok(Token::Number(1.0)),
            tok(Token::Plus),
            tok(Token::Number(2.0)),
            tok(Token::Star),
            tok(Token::Number(3.0)),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert_eq!(
            result.unwrap(),
            Expr::BinaryOp {
                left: Box::new(Expr::Number(1.0)),
                op: BinaryOp::Add,
                right: Box::new(Expr::BinaryOp {
                    left: Box::new(Expr::Number(2.0)),
                    op: BinaryOp::Mul,
                    right: Box::new(Expr::Number(3.0)),
                }),
            }
        );
    }

    #[test]
    fn test_parse_precedence_div_over_sub() {
        // 10 - 6 / 2 = 10 - (6 / 2)
        let tokens = vec![
            tok(Token::Number(10.0)),
            tok(Token::Minus),
            tok(Token::Number(6.0)),
            tok(Token::Slash),
            tok(Token::Number(2.0)),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert_eq!(
            result.unwrap(),
            Expr::BinaryOp {
                left: Box::new(Expr::Number(10.0)),
                op: BinaryOp::Sub,
                right: Box::new(Expr::BinaryOp {
                    left: Box::new(Expr::Number(6.0)),
                    op: BinaryOp::Div,
                    right: Box::new(Expr::Number(2.0)),
                }),
            }
        );
    }

    #[test]
    fn test_parse_left_associativity_addition() {
        // 1 + 2 + 3 = (1 + 2) + 3
        let tokens = vec![
            tok(Token::Number(1.0)),
            tok(Token::Plus),
            tok(Token::Number(2.0)),
            tok(Token::Plus),
            tok(Token::Number(3.0)),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert_eq!(
            result.unwrap(),
            Expr::BinaryOp {
                left: Box::new(Expr::BinaryOp {
                    left: Box::new(Expr::Number(1.0)),
                    op: BinaryOp::Add,
                    right: Box::new(Expr::Number(2.0)),
                }),
                op: BinaryOp::Add,
                right: Box::new(Expr::Number(3.0)),
            }
        );
    }

    #[test]
    fn test_parse_left_associativity_multiplication() {
        // 2 * 3 * 4 = (2 * 3) * 4
        let tokens = vec![
            tok(Token::Number(2.0)),
            tok(Token::Star),
            tok(Token::Number(3.0)),
            tok(Token::Star),
            tok(Token::Number(4.0)),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert_eq!(
            result.unwrap(),
            Expr::BinaryOp {
                left: Box::new(Expr::BinaryOp {
                    left: Box::new(Expr::Number(2.0)),
                    op: BinaryOp::Mul,
                    right: Box::new(Expr::Number(3.0)),
                }),
                op: BinaryOp::Mul,
                right: Box::new(Expr::Number(4.0)),
            }
        );
    }

    // ============================================================
    // Tests for right-associative power operator
    // ============================================================

    #[test]
    fn test_parse_simple_power() {
        // 2 ^ 3
        let tokens = vec![
            tok(Token::Number(2.0)),
            tok(Token::Caret),
            tok(Token::Number(3.0)),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert_eq!(
            result.unwrap(),
            Expr::BinaryOp {
                left: Box::new(Expr::Number(2.0)),
                op: BinaryOp::Pow,
                right: Box::new(Expr::Number(3.0)),
            }
        );
    }

    #[test]
    fn test_parse_power_right_associative() {
        // 2 ^ 3 ^ 4 = 2 ^ (3 ^ 4) (RIGHT associative)
        let tokens = vec![
            tok(Token::Number(2.0)),
            tok(Token::Caret),
            tok(Token::Number(3.0)),
            tok(Token::Caret),
            tok(Token::Number(4.0)),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert_eq!(
            result.unwrap(),
            Expr::BinaryOp {
                left: Box::new(Expr::Number(2.0)),
                op: BinaryOp::Pow,
                right: Box::new(Expr::BinaryOp {
                    left: Box::new(Expr::Number(3.0)),
                    op: BinaryOp::Pow,
                    right: Box::new(Expr::Number(4.0)),
                }),
            }
        );
    }

    #[test]
    fn test_parse_power_precedence_over_multiplication() {
        // 2 * 3 ^ 2 = 2 * (3 ^ 2)
        let tokens = vec![
            tok(Token::Number(2.0)),
            tok(Token::Star),
            tok(Token::Number(3.0)),
            tok(Token::Caret),
            tok(Token::Number(2.0)),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert_eq!(
            result.unwrap(),
            Expr::BinaryOp {
                left: Box::new(Expr::Number(2.0)),
                op: BinaryOp::Mul,
                right: Box::new(Expr::BinaryOp {
                    left: Box::new(Expr::Number(3.0)),
                    op: BinaryOp::Pow,
                    right: Box::new(Expr::Number(2.0)),
                }),
            }
        );
    }

    // ============================================================
    // Tests for unary minus
    // ============================================================

    #[test]
    fn test_parse_unary_minus() {
        // -5
        let tokens = vec![tok(Token::Minus), tok(Token::Number(5.0))];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert_eq!(
            result.unwrap(),
            Expr::UnaryMinus(Box::new(Expr::Number(5.0)))
        );
    }

    #[test]
    fn test_parse_double_unary_minus() {
        // --5 = -(-5)
        let tokens = vec![
            tok(Token::Minus),
            tok(Token::Minus),
            tok(Token::Number(5.0)),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert_eq!(
            result.unwrap(),
            Expr::UnaryMinus(Box::new(Expr::UnaryMinus(Box::new(Expr::Number(5.0)))))
        );
    }

    #[test]
    fn test_parse_unary_minus_in_expression() {
        // 3 + -2 = 3 + (-2)
        let tokens = vec![
            tok(Token::Number(3.0)),
            tok(Token::Plus),
            tok(Token::Minus),
            tok(Token::Number(2.0)),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert_eq!(
            result.unwrap(),
            Expr::BinaryOp {
                left: Box::new(Expr::Number(3.0)),
                op: BinaryOp::Add,
                right: Box::new(Expr::UnaryMinus(Box::new(Expr::Number(2.0)))),
            }
        );
    }

    #[test]
    fn test_parse_unary_minus_with_power() {
        // -2^3 parses as (-2)^3 in our grammar
        // (power calls unary, so -2 is parsed first, then ^3)
        let tokens = vec![
            tok(Token::Minus),
            tok(Token::Number(2.0)),
            tok(Token::Caret),
            tok(Token::Number(3.0)),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        // Our grammar: power -> unary ('^' power)?
        // So -2^3 parses as (-2)^3
        assert_eq!(
            result.unwrap(),
            Expr::BinaryOp {
                left: Box::new(Expr::UnaryMinus(Box::new(Expr::Number(2.0)))),
                op: BinaryOp::Pow,
                right: Box::new(Expr::Number(3.0)),
            }
        );
    }

    // ============================================================
    // Tests for parentheses
    // ============================================================

    #[test]
    fn test_parse_parentheses_simple() {
        // (5)
        let tokens = vec![
            tok(Token::LParen),
            tok(Token::Number(5.0)),
            tok(Token::RParen),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert_eq!(result.unwrap(), Expr::Number(5.0));
    }

    #[test]
    fn test_parse_parentheses_override_precedence() {
        // (1 + 2) * 3
        let tokens = vec![
            tok(Token::LParen),
            tok(Token::Number(1.0)),
            tok(Token::Plus),
            tok(Token::Number(2.0)),
            tok(Token::RParen),
            tok(Token::Star),
            tok(Token::Number(3.0)),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert_eq!(
            result.unwrap(),
            Expr::BinaryOp {
                left: Box::new(Expr::BinaryOp {
                    left: Box::new(Expr::Number(1.0)),
                    op: BinaryOp::Add,
                    right: Box::new(Expr::Number(2.0)),
                }),
                op: BinaryOp::Mul,
                right: Box::new(Expr::Number(3.0)),
            }
        );
    }

    #[test]
    fn test_parse_nested_parentheses() {
        // ((1 + 2))
        let tokens = vec![
            tok(Token::LParen),
            tok(Token::LParen),
            tok(Token::Number(1.0)),
            tok(Token::Plus),
            tok(Token::Number(2.0)),
            tok(Token::RParen),
            tok(Token::RParen),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert_eq!(
            result.unwrap(),
            Expr::BinaryOp {
                left: Box::new(Expr::Number(1.0)),
                op: BinaryOp::Add,
                right: Box::new(Expr::Number(2.0)),
            }
        );
    }

    // ============================================================
    // Tests for variables
    // ============================================================

    #[test]
    fn test_parse_variable() {
        // x
        let tokens = vec![tok(Token::Identifier("x".to_string()))];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert_eq!(result.unwrap(), Expr::Variable("x".to_string()));
    }

    #[test]
    fn test_parse_variable_in_expression() {
        // x + 1
        let tokens = vec![
            tok(Token::Identifier("x".to_string())),
            tok(Token::Plus),
            tok(Token::Number(1.0)),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert_eq!(
            result.unwrap(),
            Expr::BinaryOp {
                left: Box::new(Expr::Variable("x".to_string())),
                op: BinaryOp::Add,
                right: Box::new(Expr::Number(1.0)),
            }
        );
    }

    // ============================================================
    // Tests for function calls
    // ============================================================

    #[test]
    fn test_parse_function_call_no_args() {
        // random()
        let tokens = vec![
            tok(Token::Identifier("random".to_string())),
            tok(Token::LParen),
            tok(Token::RParen),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert_eq!(
            result.unwrap(),
            Expr::FunctionCall {
                name: "random".to_string(),
                args: vec![],
            }
        );
    }

    #[test]
    fn test_parse_function_call_one_arg() {
        // sqrt(16)
        let tokens = vec![
            tok(Token::Identifier("sqrt".to_string())),
            tok(Token::LParen),
            tok(Token::Number(16.0)),
            tok(Token::RParen),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert_eq!(
            result.unwrap(),
            Expr::FunctionCall {
                name: "sqrt".to_string(),
                args: vec![Expr::Number(16.0)],
            }
        );
    }

    #[test]
    fn test_parse_function_call_multiple_args() {
        // pow(2, 8)
        let tokens = vec![
            tok(Token::Identifier("pow".to_string())),
            tok(Token::LParen),
            tok(Token::Number(2.0)),
            tok(Token::Comma),
            tok(Token::Number(8.0)),
            tok(Token::RParen),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert_eq!(
            result.unwrap(),
            Expr::FunctionCall {
                name: "pow".to_string(),
                args: vec![Expr::Number(2.0), Expr::Number(8.0)],
            }
        );
    }

    #[test]
    fn test_parse_function_call_three_args() {
        // clamp(x, 0, 100)
        let tokens = vec![
            tok(Token::Identifier("clamp".to_string())),
            tok(Token::LParen),
            tok(Token::Identifier("x".to_string())),
            tok(Token::Comma),
            tok(Token::Number(0.0)),
            tok(Token::Comma),
            tok(Token::Number(100.0)),
            tok(Token::RParen),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert_eq!(
            result.unwrap(),
            Expr::FunctionCall {
                name: "clamp".to_string(),
                args: vec![
                    Expr::Variable("x".to_string()),
                    Expr::Number(0.0),
                    Expr::Number(100.0),
                ],
            }
        );
    }

    #[test]
    fn test_parse_function_call_with_expression_arg() {
        // sqrt(1 + 3)
        let tokens = vec![
            tok(Token::Identifier("sqrt".to_string())),
            tok(Token::LParen),
            tok(Token::Number(1.0)),
            tok(Token::Plus),
            tok(Token::Number(3.0)),
            tok(Token::RParen),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert_eq!(
            result.unwrap(),
            Expr::FunctionCall {
                name: "sqrt".to_string(),
                args: vec![Expr::BinaryOp {
                    left: Box::new(Expr::Number(1.0)),
                    op: BinaryOp::Add,
                    right: Box::new(Expr::Number(3.0)),
                }],
            }
        );
    }

    #[test]
    fn test_parse_nested_function_calls() {
        // sqrt(abs(-16))
        let tokens = vec![
            tok(Token::Identifier("sqrt".to_string())),
            tok(Token::LParen),
            tok(Token::Identifier("abs".to_string())),
            tok(Token::LParen),
            tok(Token::Minus),
            tok(Token::Number(16.0)),
            tok(Token::RParen),
            tok(Token::RParen),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert_eq!(
            result.unwrap(),
            Expr::FunctionCall {
                name: "sqrt".to_string(),
                args: vec![Expr::FunctionCall {
                    name: "abs".to_string(),
                    args: vec![Expr::UnaryMinus(Box::new(Expr::Number(16.0)))],
                }],
            }
        );
    }

    // ============================================================
    // Tests for error cases
    // ============================================================

    #[test]
    fn test_parse_error_empty_input() {
        let tokens: Vec<Spanned<Token>> = vec![];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_err());
        assert!(result.unwrap_err().message.contains("end of expression"));
    }

    #[test]
    fn test_parse_error_unclosed_paren() {
        // (5 + 3
        let tokens = vec![
            tok(Token::LParen),
            tok(Token::Number(5.0)),
            tok(Token::Plus),
            tok(Token::Number(3.0)),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .message
                .contains("Expected ')' after expression")
        );
    }

    #[test]
    fn test_parse_error_unclosed_function_call() {
        // sqrt(16
        let tokens = vec![
            tok(Token::Identifier("sqrt".to_string())),
            tok(Token::LParen),
            tok(Token::Number(16.0)),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .message
                .contains("Expected ')' after function arguments")
        );
    }

    #[test]
    fn test_parse_error_unexpected_token() {
        // ) at start
        let tokens = vec![tok(Token::RParen)];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_err());
        assert!(result.unwrap_err().message.contains("Unexpected token"));
    }

    #[test]
    fn test_parse_error_trailing_operator() {
        // 5 +
        let tokens = vec![tok(Token::Number(5.0)), tok(Token::Plus)];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_consecutive_operators() {
        // 5 + + 3 - this actually parses as 5 + (+3), but we don't have unary plus
        // So this should be an error
        let tokens = vec![
            tok(Token::Number(5.0)),
            tok(Token::Plus),
            tok(Token::Plus),
            tok(Token::Number(3.0)),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        // Plus is not a valid start of unary, so this should fail
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_extra_closing_paren() {
        // 5 + 3)
        let tokens = vec![
            tok(Token::Number(5.0)),
            tok(Token::Plus),
            tok(Token::Number(3.0)),
            tok(Token::RParen),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_err());
        assert!(result.unwrap_err().message.contains("Unexpected token"));
    }

    // ============================================================
    // Tests for complex expressions
    // ============================================================

    #[test]
    fn test_parse_complex_expression() {
        // 2 ^ (a + 5) * 3
        let tokens = vec![
            tok(Token::Number(2.0)),
            tok(Token::Caret),
            tok(Token::LParen),
            tok(Token::Identifier("a".to_string())),
            tok(Token::Plus),
            tok(Token::Number(5.0)),
            tok(Token::RParen),
            tok(Token::Star),
            tok(Token::Number(3.0)),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        // Power has higher precedence than multiply, but we have parens
        // So this is: (2 ^ (a + 5)) * 3
        assert_eq!(
            result.unwrap(),
            Expr::BinaryOp {
                left: Box::new(Expr::BinaryOp {
                    left: Box::new(Expr::Number(2.0)),
                    op: BinaryOp::Pow,
                    right: Box::new(Expr::BinaryOp {
                        left: Box::new(Expr::Variable("a".to_string())),
                        op: BinaryOp::Add,
                        right: Box::new(Expr::Number(5.0)),
                    }),
                }),
                op: BinaryOp::Mul,
                right: Box::new(Expr::Number(3.0)),
            }
        );
    }
}
