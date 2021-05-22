#![feature(box_patterns)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]

pub mod expression;
mod lexer;
mod parser;

use std::error;
use std::fmt;

use expression::Expr;
use lexer::{LexError, Lexer};
use parser::ParseError;
use parser::Parser;

/// Parse the math expression passed in as a string
/// # Errors
/// this function errors if there is an error while parsing the string
/// i.e. a lex error or parse error
pub fn parse(source: &str) -> Result<Expr, Error> {
    let mut lexer = Lexer::new(source);
    Ok(Parser::new(lexer.tokens()?.as_slice()).expression()?)
}

#[derive(Debug)]
pub enum Error {
    LexError(LexError),
    ParseError(ParseError),
}

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Self {
        Error::ParseError(err)
    }
}

impl From<LexError> for Error {
    fn from(err: LexError) -> Self {
        Error::LexError(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LexError(err) => write!(f, "error while lexing: {}", err),
            Self::ParseError(err) => write!(f, "error while parsing: {}", err),
        }
    }
}

impl error::Error for Error {}
