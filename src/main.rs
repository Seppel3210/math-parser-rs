#![feature(box_patterns)]
use expression::Expr;

mod expression;

fn main() {
    use Expr::*;
    let expr = Add(Box::new(Var("x".to_owned())), Box::new(Const(3.0)));
    println!("{}", expr);
    println!("{}", expr.reduce());
    println!("{}", expr.derive("x").reduce());
}
