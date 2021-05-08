#![feature(box_patterns)]

mod expression;
mod lexer;
mod parser;

use lexer::Lexer;
use parser::Parser;

fn main() {
    let expr = Parser::new(&Lexer::new("22/7").tokens().unwrap())
        .expression()
        .unwrap();
    println!("{:?}", expr);
    println!("{:?}", expr.reduce());
    println!("{:?}", expr.derive("x").reduce());
}
