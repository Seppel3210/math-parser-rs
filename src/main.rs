#![feature(box_patterns)]
use expression::Expr;

mod expression;
mod lexer;

fn main() {
    use Expr::*;
    let expr = Pow(Box::new(Var("x".to_owned())), Box::new(Const(2.0))) + Var("x".to_owned());
    println!("{:?}", expr);
    println!("{:?}", expr.reduce());
    println!("{:?}", expr.derive("x").reduce());
    let tokens = lexer::Lexer::new("(dctie+ - 03.32 ) *").tokens();
    println!("{:?}", tokens);
}
