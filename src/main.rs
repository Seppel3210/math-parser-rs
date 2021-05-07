#![feature(box_patterns)]
use expression::Expr;

mod expression;

fn main() {
    use Expr::*;
    let expr = Pow(Box::new(Var("x".to_owned())), Box::new(Const(2.0))) + Var("x".to_owned());
    println!("{:?}", expr);
    println!("{:?}", expr.reduce());
    println!("{:?}", expr.derive("x").reduce());
}
