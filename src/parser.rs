use std::{error::Error, fmt, iter::Peekable, slice::Iter};

use crate::expression::Expr::*;
use crate::{expression::Expr, lexer::Token, lexer::TokenType};
use ParseError::*;
use TokenType::*;

pub struct Parser<'a> {
    tokens: Peekable<Iter<'a, Token>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser {
            tokens: tokens.iter().peekable(),
        }
    }

    pub fn expression(&mut self) -> Result<Expr, ParseError> {
        self.term()
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;
        while let Some(Plus | Minus) = self.peek_type() {
            let operator = self.next_type().unwrap();
            let right = self.factor()?;
            expr = match operator {
                Plus => expr + right,
                Minus => expr - right,
                _ => unreachable!(),
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;

        while let Some(Star | Slash) = self.peek_type() {
            let operator = self.next_type().unwrap();
            let right = self.unary()?;
            expr = match operator {
                Star => expr * right,
                Slash => expr / right,
                _ => unreachable!(),
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if let Some(Minus) = self.peek_type() {
            self.next();
            let right = self.unary()?;
            Ok(-right)
        } else {
            self.power()
        }
    }

    fn power(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.function()?;

        if let Some(Caret) = self.peek_type() {
            self.next();
            let right = self.power()?;
            expr = Pow(Box::new(expr), Box::new(right));
        }

        Ok(expr)
    }

    fn function(&mut self) -> Result<Expr, ParseError> {
        if let Some(FnLn) = self.peek_type() {
            self.next();
            let right = self.primary()?;
            Ok(Ln(Box::new(right)))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        let token = self.next().ok_or(UnexpectedEof)?;
        match token.kind {
            Number => Ok(Const(token.lexeme.parse().unwrap())),
            Ident => Ok(Var(token.lexeme.clone())),
            LeftParen => {
                let expr = self.expression()?;
                let paren = self.next().ok_or(UnexpectedEof)?;
                if paren.kind != RightParen {
                    Err(Other(format!(
                        "expected ')' after expression at {}:{}",
                        paren.position.0, paren.position.1
                    )))?
                }
                Ok(expr)
            }
            _ => Err(UnexpectedToken(token.clone())),
        }
    }

    fn next(&mut self) -> Option<&Token> {
        self.tokens.next()
    }

    fn next_type(&mut self) -> Option<TokenType> {
        self.tokens.next().map(|t| t.kind)
    }

    fn peek_type(&mut self) -> Option<TokenType> {
        self.tokens.peek().map(|t| t.kind)
    }
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(Token),
    UnexpectedEof,
    Other(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnexpectedToken(t) => write!(
                f,
                "unexpected token: {:?} at {}:{}",
                t.kind, t.position.0, t.position.1
            ),
            UnexpectedEof => write!(f, "unexpected EOF"),
            Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for ParseError {}
