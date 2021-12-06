use std::io;
use std::{collections::HashMap, io::Write};

use math_parser::expression::Expr;

fn main() {
    let mut actions = HashMap::new();
    actions.insert(
        "derive",
        Box::new(|expr: Expr| {
            let var_name = input("differetiate with respect to? ");
            expr.derive(var_name.trim()).reduce()
        }) as Box<dyn Fn(_) -> _>,
    );
    actions.insert(
        "substitute",
        Box::new(|expr: Expr| {
            let var_name = input("which variable? ");
            let input = input("with which expression? ");
            let expr2 = loop {
                match math_parser::parse(&input) {
                    Ok(expr) => break expr,
                    Err(err) => {
                        println!("{:?}", err);
                        continue;
                    }
                };
            };
            expr.substitute(var_name.trim(), &expr2)
        }),
    );
    actions.insert("reduce", Box::new(|expr: Expr| expr.reduce()));
    actions.insert(
        "dbg",
        Box::new(|expr: Expr| {
            println!("{:?}", expr);
            expr
        }),
    );

    println!("math_parser CLI\n(c) Sebastian Widua 2021");
    loop {
        let input = input("type an expression:\n");
        let expr = match math_parser::parse(&input) {
            Ok(expr) => expr,
            Err(err) => {
                println!("{:?}", err);
                continue;
            }
        };
        expression_menu(expr, &actions);
    }
}

fn expression_menu(mut expr: Expr, actions: &HashMap<&str, Box<dyn Fn(Expr) -> Expr>>) {
    loop {
        println!("{}", expr);
        println!(
            r#"actions: {:?} or "exit" to type another expression"#,
            actions.keys()
        );
        let action_name = input("");
        if action_name.trim() == "exit" {
            return;
        }
        match actions.get(action_name.trim()) {
            Some(f) => expr = f(expr),
            None => {
                println!("unknown action");
                continue;
            }
        }
    }
}

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("error flushing stdout");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error reading stdin");
    input
}
