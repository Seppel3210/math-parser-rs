#![feature(box_patterns)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]

pub mod expression;
mod parser;

use chumsky::Error;
use expression::Expr;

use chumsky::prelude::*;

/// Parse the math expression passed in as a string
/// # Errors
/// this function errors if there is an error while parsing the string
pub fn parse(source: &str) -> Result<Expr, Vec<Simple<char>>> {
    let expr = recursive(|expr| {
        let num = text::digits(10)
            .chain::<char, _, _>(just('.').chain(text::digits(10)).or_not().flatten())
            .collect::<String>()
            .map(|s| Expr::Const(s.parse().unwrap()));

        let ident = text::ident().padded();

        let var = ident.map(Expr::Var);

        let atom = num.or(var).or(expr.delimited_by('(', ')'));

        let call = ident
            .then(atom.clone())
            .try_map(|(name, arg), span| {
                if name == "ln" {
                    Ok(Expr::Ln(Box::new(arg)))
                } else {
                    Err(Error::expected_input_found(
                        span,
                        "ln".chars(),
                        name.chars().next(),
                    ))
                }
            })
            .or(atom);

        let op = |c| just(c).padded();

        let pow = recursive(|pow| {
            call.clone()
                .then(op('^').ignore_then(pow).or_not())
                .map(|(lhs, rhs)| {
                    if let Some(rhs) = rhs {
                        Expr::Pow(Box::new(lhs), Box::new(rhs))
                    } else {
                        lhs
                    }
                })
        });

        let unary = op('-')
            .repeated()
            .then(pow)
            .foldr(|_, rhs| Expr::Neg(Box::new(rhs)));

        let product = unary
            .clone()
            .then(
                op('*')
                    .to(Expr::Mul as fn(_, _) -> _)
                    .or(op('/').to(Expr::Mul as fn(_, _) -> _))
                    .then(unary)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

        let sum = product
            .clone()
            .then(
                op('+')
                    .to(Expr::Add as fn(_, _) -> _)
                    .or(op('-').to(Expr::Sub as fn(_, _) -> _))
                    .then(product)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

        sum.padded()
    });
    expr.then_ignore(end()).parse(source)
}
