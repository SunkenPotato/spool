use std::{
    error::Error,
    ops::{Add, Div, Mul, Sub},
};

use crate::{
    utils::{extract_digits, extract_operator, extract_whitespace},
    val::Val,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Integer(pub i32);

impl Add for Integer {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Integer(self.0 + other.0)
    }
}

impl Sub for Integer {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Integer(self.0 - other.0)
    }
}

impl Mul for Integer {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Integer(self.0 * other.0)
    }
}

impl Div for Integer {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Integer(self.0 / other.0)
    }
}

impl Parse for Integer {
    fn parse(input: &str) -> Result<(String, Self), ParseError> {
        let (num, s) = extract_digits(input);
        Ok((s, Integer(num.parse()?)))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Parse for Op {
    fn parse(input: &str) -> Result<(String, Self), ParseError> {
        let (s, op) = extract_operator(input);

        let op = match op.as_str() {
            "+" => Op::Add,
            "-" => Op::Sub,
            "*" => Op::Mul,
            "/" => Op::Div,
            o => return Err(ParseError::from(format!("Invalid operator: {o}"))),
        };

        Ok((s, op))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Expr {
    Simple(Integer),
    Complex { lhs: Integer, op: Op, rhs: Integer },
}

impl Expr {
    pub fn eval(&self) -> Val {
        match self {
            Expr::Simple(i) => Val::Integer(i.0),
            Expr::Complex { lhs, op, rhs } => match op {
                Op::Add => Val::Integer((*lhs + *rhs).0),
                Op::Sub => Val::Integer((*lhs - *rhs).0),
                Op::Mul => Val::Integer((*lhs * *rhs).0),
                Op::Div => Val::Integer((*lhs / *rhs).0),
            },
        }
    }
}

impl Parse for Expr {
    fn parse(s: &str) -> Result<(String, Self), ParseError> {
        let (s, lhs) = Integer::parse(s)?;
        let (_, s) = extract_whitespace(&s);

        let (s, op) = match Op::parse(&s) {
            Ok(v) => v,
            Err(_) => {
                return Ok((s, Expr::Simple(lhs)));
            }
        };

        let (_, s) = extract_whitespace(&s);

        let (s, rhs) = Integer::parse(&s)?;

        Ok((s, Self::Complex { lhs, op, rhs }))
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::extract_identifier;

    use super::*;

    #[test]
    fn parse_op() {
        assert_eq!(Op::parse("+").unwrap(), ("".into(), Op::Add));
        assert_eq!(Op::parse("-").unwrap(), ("".into(), Op::Sub));
        assert_eq!(Op::parse("*").unwrap(), ("".into(), Op::Mul));
        assert_eq!(Op::parse("/").unwrap(), ("".into(), Op::Div));
    }

    #[test]
    fn parse_integer() {
        assert_eq!(Integer::parse("123").unwrap(), ("".into(), Integer(123)));
        assert_eq!(Integer::parse("456").unwrap(), ("".into(), Integer(456)));
    }

    #[test]
    fn extract_number() {
        assert_eq!(extract_digits("123+456"), ("123".into(), "+456".into()));
    }

    #[test]
    fn extract_op() {
        assert_eq!(extract_operator("+456"), ("456".into(), "+".into()));
    }

    #[test]
    fn test_extract_whitespace() {
        assert_eq!(extract_whitespace("  123").1, "123");
    }

    #[test]
    fn parse_expr() {
        assert_eq!(
            Expr::parse("123 + 456").unwrap(),
            (
                "".into(),
                Expr::Complex {
                    lhs: Integer(123),
                    op: Op::Add,
                    rhs: Integer(456),
                }
            )
        )
    }

    #[test]
    fn extract_ident() {
        assert_eq!(
            extract_identifier("x 456").unwrap(),
            ("x".into(), " 456".into())
        );
    }

    #[test]
    fn eval_expr() {
        assert_eq!(Expr::parse("1 + 2").unwrap().1.eval(), Val::Integer(3))
    }
}
