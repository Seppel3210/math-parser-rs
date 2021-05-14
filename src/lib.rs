#![feature(box_patterns)]

pub mod expression;
mod lexer;
mod parser;

use expression::Expr;
use lexer::{LexError, Lexer};
use parser::ParseError;
use parser::Parser;

pub fn parse(source: &str) -> Result<Expr, Error> {
    let mut lexer = Lexer::new(source);
    Ok(Parser::new(lexer.tokens()?.as_slice()).expression()?)
}

#[derive(Debug)]
pub enum Error {
    ParseError(ParseError),
    LexError(LexError),
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
