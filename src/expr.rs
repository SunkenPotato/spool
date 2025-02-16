use std::ops::{Add, Div, Mul, Sub};

use crate::{
    block::Block,
    parse::{Parse, ParseError, ParseOutput},
    utils::{extract_digits, extract_operator, extract_whitespace},
    val::Val,
    var::BindingRef,
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
    fn parse(input: &str) -> ParseOutput<Self> {
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
    fn parse(input: &str) -> ParseOutput<Self> {
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expr {
    Simple(Integer),
    Complex { lhs: Integer, op: Op, rhs: Integer },
    BindingRef(BindingRef),
    Block(Block),
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
            _ => todo!(),
        }
    }

    fn parse_simple(input: &str) -> ParseOutput<Self> {
        Integer::parse(input).map(|(s, int)| (s, Expr::Simple(int)))
    }

    fn parse_complex(input: &str) -> ParseOutput<Self> {
        let (input, lhs) = Integer::parse(input)?;
        let (_, input) = extract_whitespace(&input);

        let (input, op) = Op::parse(&input)?;
        let (_, input) = extract_whitespace(&input);
        let (input, rhs) = Integer::parse(&input)?;

        Ok((input, Expr::Complex { lhs, op, rhs }))
    }
}

impl Parse for Expr {
    fn parse(input: &str) -> ParseOutput<Self> {
        Self::parse_simple(input)
            .or_else(|_| Self::parse_complex(input))
            .or_else(|_| {
                BindingRef::parse(input).map(|(s, bind_ref)| (s, Expr::BindingRef(bind_ref)))
            })
            .or_else(|_| Block::parse(input).map(|(s, block)| (s, Expr::Block(block))))
    }
}

#[cfg(test)]
mod tests {
    use crate::{stmt::Stmt, utils::extract_identifier};

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
    fn parse_ref_expr() {
        assert_eq!(
            Expr::parse("x").unwrap(),
            ("".into(), Expr::BindingRef(BindingRef { name: "x".into() }))
        )
    }

    #[test]
    fn parse_block_expr() {
        assert_eq!(
            Expr::parse("{ x }").unwrap(),
            (
                "".into(),
                Expr::Block(Block {
                    stmts: vec![Stmt::Expr(Expr::BindingRef(BindingRef {
                        name: "x".into()
                    }))]
                })
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
