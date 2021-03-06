use core::fmt;
use std::ops;

#[derive(Clone)]
pub enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
    Ln(Box<Expr>),
    Var(String),
    Const(f64),
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
pub enum Precedence {
    Lowest,
    Sum,
    Product,
    Power,
    PowerLeft,
    Highest,
}

use Expr::{Add, Const, Div, Ln, Mul, Neg, Pow, Sub, Var};
use Precedence::{Highest, Lowest, Power, PowerLeft, Product, Sum};

impl Expr {
    #[must_use]
    pub fn reduce(&self) -> Expr {
        match self {
            Add(box left, box right) => Expr::reduce_add(left, right),
            Sub(box left, box right) => Expr::reduce_sub(left, right),
            Mul(box left, box right) => Expr::reduce_mul(left, right),
            Div(box left, box right) => Expr::reduce_div(left, right),
            Pow(box left, box right) => Expr::reduce_pow(left, right),
            Ln(box arg) => Expr::reduce_ln(arg),
            Neg(box arg) => Expr::reduce_neg(arg),
            Var(_) | Const(_) => self.clone(),
        }
    }

    fn reduce_add(left: &Expr, right: &Expr) -> Expr {
        match (left.reduce(), right.reduce()) {
            (Const(z), other) if (z - 0.0).abs() < f64::EPSILON => other,
            (other, Const(z)) if (z - 0.0).abs() < f64::EPSILON => other,
            (Const(c1), Add(box Const(c2), box right) | Add(box right, box Const(c2))) => {
                Const(c1 + c2) + right
            }
            (Add(box Const(c1), box left) | Add(box left, box Const(c1)), Const(c2)) => {
                left + Const(c1 + c2)
            }
            (Const(left), Const(right)) => Const(left + right),
            (left, right) => left + right,
        }
    }

    fn reduce_sub(left: &Expr, right: &Expr) -> Expr {
        match (left.reduce(), right.reduce()) {
            (Const(left), Const(right)) => Const(left - right),
            (left, right) => left - right,
        }
    }

    fn reduce_mul(left: &Expr, right: &Expr) -> Expr {
        match (left.reduce(), right.reduce()) {
            (Const(z), _) if (z - 0.0).abs() < f64::EPSILON => Const(0.0),
            (_, Const(z)) if (z - 0.0).abs() < f64::EPSILON => Const(0.0),
            (Const(one), other) if (one - 1.0).abs() < f64::EPSILON => other,
            (other, Const(one)) if (one - 1.0).abs() < f64::EPSILON => other,
            (Const(c1), Mul(box Const(c2), box right) | Mul(box right, box Const(c2))) => {
                Const(c1 * c2) * right
            }
            (Mul(box Const(c1), box left) | Mul(box left, box Const(c1)), Const(c2)) => {
                left * Const(c1 * c2)
            }
            (Const(left), Const(right)) => Const(left * right),
            (left, right) => left * right,
        }
    }

    fn reduce_div(left: &Expr, right: &Expr) -> Expr {
        match (left.reduce(), right.reduce()) {
            (Const(left), Const(right)) => Const(left / right),
            (left, right) => left / right,
        }
    }

    fn reduce_pow(left: &Expr, right: &Expr) -> Expr {
        match (left.reduce(), right.reduce()) {
            (_, Const(z)) if (z - 0.0).abs() < f64::EPSILON => Const(1.0),
            (Const(z), _) if (z - 0.0).abs() < f64::EPSILON => Const(0.0),
            (left, Const(x)) if (x - 1.0).abs() < f64::EPSILON => left,
            (Const(left), Const(right)) => Const(left.powf(right)),
            (left, right) => Pow(Box::new(left), Box::new(right)),
        }
    }

    fn reduce_ln(arg: &Expr) -> Expr {
        match arg.reduce() {
            Const(arg) => Const(arg.ln()),
            Var(s) if s == "e" => Const(1.0),
            arg => Ln(Box::new(arg)),
        }
    }

    fn reduce_neg(arg: &Expr) -> Expr {
        match arg.reduce() {
            Const(arg) => Const(-arg),
            arg => -arg,
        }
    }

    #[must_use]
    pub fn derive(&self, var_name: &str) -> Expr {
        match self {
            Add(left, right) => left.derive(var_name) + right.derive(var_name),
            Sub(left, right) => left.derive(var_name) - right.derive(var_name),
            Mul(box left, box right) => {
                &left.derive(var_name) * right + &right.derive(var_name) * left
            }
            Div(box left, box right) => {
                (&left.derive(var_name) * right - &right.derive(var_name) * left)
                    / Pow(Box::new(right.clone()), Box::new(Const(2.0)))
            }
            Pow(box left, box right) => {
                (right.derive(var_name) * Ln(Box::new(left.clone())) * self.clone())
                    + (right
                        * &left.derive(var_name)
                        * Pow(Box::new(left.clone()), Box::new(right - &Const(1.0))))
            }
            Neg(box arg) => -arg.derive(var_name),
            Var(name) if name == var_name => Const(1.0),
            Ln(box arg) => arg.derive(var_name) / arg.clone(),
            Const(_) | Var(_) => Const(0.0),
        }
    }

    #[must_use]
    pub fn substitute(&self, var_name: &str, expr: &Expr) -> Expr {
        match self {
            Add(left, right) => left.substitute(var_name, expr) + right.substitute(var_name, expr),
            Sub(left, right) => left.substitute(var_name, expr) - right.substitute(var_name, expr),
            Mul(left, right) => left.substitute(var_name, expr) * right.substitute(var_name, expr),
            Div(left, right) => left.substitute(var_name, expr) / right.substitute(var_name, expr),
            Pow(left, right) => Pow(
                Box::new(left.substitute(var_name, expr)),
                Box::new(right.substitute(var_name, expr)),
            ),
            Neg(arg) => -arg.substitute(var_name, expr),
            Ln(arg) => Ln(Box::new(arg.substitute(var_name, expr))),
            Var(_) | Const(_) => self.clone(),
        }
    }

    fn pretty_print(
        &self,
        outer_precedence: Precedence,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let this_precedence = self.precedence();
        if outer_precedence > this_precedence {
            write!(f, "(")?;
        }

        match self {
            Add(left, right) => {
                left.pretty_print(this_precedence, f)?;
                write!(f, " + ")?;
                right.pretty_print(this_precedence, f)?;
            }
            Sub(left, right) => {
                left.pretty_print(this_precedence, f)?;
                write!(f, " - ")?;
                right.pretty_print(this_precedence, f)?;
            }
            Mul(left, right) => {
                left.pretty_print(this_precedence, f)?;
                write!(f, " * ")?;
                right.pretty_print(this_precedence, f)?;
            }
            Div(left, right) => {
                left.pretty_print(this_precedence, f)?;
                write!(f, " / ")?;
                right.pretty_print(this_precedence, f)?;
            }
            Pow(left, right) => {
                left.pretty_print(PowerLeft, f)?;
                write!(f, " ^ ")?;
                right.pretty_print(Power, f)?;
            }
            Neg(arg) => arg.pretty_print(this_precedence, f)?,
            Ln(arg) => {
                if self.precedence() > arg.precedence() {
                    write!(f, "ln")?;
                } else {
                    write!(f, "ln ")?;
                }
                arg.pretty_print(self.precedence(), f)?;
            }
            Var(name) => write!(f, "{}", name)?,
            Const(val) => write!(f, "{}", val)?,
        }
        if outer_precedence > this_precedence {
            write!(f, ")")?;
        }
        Ok(())
    }

    fn precedence(&self) -> Precedence {
        match self {
            Add(_, _) | Sub(_, _) => Sum,
            Mul(_, _) | Div(_, _) => Product,
            Pow(_, _) => Power,
            Neg(_) | Ln(_) | Var(_) | Const(_) => Highest,
        }
    }
}

impl ops::Add for Expr {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Expr::Add(Box::new(self), Box::new(rhs))
    }
}

impl ops::Add for &Expr {
    type Output = Expr;
    fn add(self, rhs: Self) -> Self::Output {
        Expr::Add(Box::new(self.clone()), Box::new(rhs.clone()))
    }
}

impl ops::Sub for Expr {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Expr::Sub(Box::new(self), Box::new(rhs))
    }
}

impl ops::Sub for &Expr {
    type Output = Expr;
    fn sub(self, rhs: Self) -> Self::Output {
        Expr::Sub(Box::new(self.clone()), Box::new(rhs.clone()))
    }
}

impl ops::Mul for Expr {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Expr::Mul(Box::new(self), Box::new(rhs))
    }
}

impl ops::Mul for &Expr {
    type Output = Expr;
    fn mul(self, rhs: Self) -> Self::Output {
        Expr::Mul(Box::new(self.clone()), Box::new(rhs.clone()))
    }
}

impl ops::Div for Expr {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Expr::Div(Box::new(self), Box::new(rhs))
    }
}

impl ops::Div for &Expr {
    type Output = Expr;
    fn div(self, rhs: Self) -> Self::Output {
        Expr::Div(Box::new(self.clone()), Box::new(rhs.clone()))
    }
}

impl ops::Neg for Expr {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Expr::Neg(Box::new(self))
    }
}

impl ops::Neg for &Expr {
    type Output = Expr;
    fn neg(self) -> Self::Output {
        Expr::Neg(Box::new(self.clone()))
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.pretty_print(Lowest, f)?;
        Ok(())
    }
}

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Add(left, right) => write!(f, "({:?} + {:?})", left, right),
            Sub(left, right) => write!(f, "({:?} - {:?})", left, right),
            Mul(left, right) => write!(f, "({:?} * {:?})", left, right),
            Div(left, right) => write!(f, "({:?} / {:?})", left, right),
            Pow(left, right) => write!(f, "({:?} ^ {:?})", left, right),
            Neg(arg) => write!(f, "(-{:?})", arg),
            Ln(arg) => write!(f, "ln({:?})", arg),
            Var(name) => write!(f, "{}", name),
            Const(val) => write!(f, "{}", val),
        }
    }
}
