use core::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone)]
pub enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
    Ln(Box<Expr>),
    Var(String),
    Const(f32),
}

impl Expr {
    pub fn reduce(&self) -> Expr {
        use Expr::*;

        match self {
            Add(box Const(left), box Const(right)) => Const(left + right),
            Sub(box Const(left), box Const(right)) => Const(left - right),
            Mul(box Const(left), box Const(right)) => Const(left * right),
            Div(box Const(left), box Const(right)) => Const(left / right),
            Pow(box Const(left), box Const(right)) => Const(left.powf(*right)),
            Neg(box Const(arg)) => Const(-arg),
            _ => self.clone(),
        }
    }

    pub fn derive(&self, var_name: &str) -> Expr {
        use Expr::*;

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

    pub fn substitute(&self, var_name: &str, expr: &Expr) -> Expr {
        use Expr::*;

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
}

impl Add for Expr {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Expr::Add(Box::new(self), Box::new(rhs))
    }
}

impl Add for &Expr {
    type Output = Expr;
    fn add(self, rhs: Self) -> Self::Output {
        Expr::Add(Box::new(self.clone()), Box::new(rhs.clone()))
    }
}

impl Sub for Expr {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Expr::Sub(Box::new(self), Box::new(rhs))
    }
}

impl Sub for &Expr {
    type Output = Expr;
    fn sub(self, rhs: Self) -> Self::Output {
        Expr::Sub(Box::new(self.clone()), Box::new(rhs.clone()))
    }
}

impl Mul for Expr {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Expr::Mul(Box::new(self), Box::new(rhs))
    }
}

impl Mul for &Expr {
    type Output = Expr;
    fn mul(self, rhs: Self) -> Self::Output {
        Expr::Mul(Box::new(self.clone()), Box::new(rhs.clone()))
    }
}

impl Div for Expr {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Expr::Div(Box::new(self), Box::new(rhs))
    }
}

impl Div for &Expr {
    type Output = Expr;
    fn div(self, rhs: Self) -> Self::Output {
        Expr::Div(Box::new(self.clone()), Box::new(rhs.clone()))
    }
}

impl Neg for Expr {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Expr::Neg(Box::new(self))
    }
}

impl Neg for &Expr {
    type Output = Expr;
    fn neg(self) -> Self::Output {
        Expr::Neg(Box::new(self.clone()))
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Expr::*;

        match self {
            Add(left, right) => write!(f, "({} + {})", left, right),
            Sub(left, right) => write!(f, "({} - {})", left, right),
            Mul(left, right) => write!(f, "({} * {})", left, right),
            Div(left, right) => write!(f, "({} / {})", left, right),
            Pow(left, right) => write!(f, "({} ^ {})", left, right),
            Neg(arg) => write!(f, "(-{})", arg),
            Ln(arg) => write!(f, "ln({})", arg),
            Var(name) => write!(f, "{}", name),
            Const(val) => write!(f, "{}", val),
        }
    }
}
