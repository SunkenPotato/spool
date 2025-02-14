use std::error::Error;

use crate::utils::{extract_digits, extract_operator, extract_whitespace};

pub type ParseError = Box<dyn Error>;

pub trait Parse: Sized {
    fn parse(input: &str) -> Result<(String, Self), ParseError>;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Integer(pub i32);

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
pub struct Expr {
    pub lhs: Integer,
    pub op: Op,
    pub rhs: Integer,
}

impl Parse for Expr {
    fn parse(s: &str) -> Result<(String, Self), ParseError> {
        let (s, lhs) = Integer::parse(s)?;
        let (_, s) = extract_whitespace(&s);

        let (s, op) = Op::parse(&s)?;
        let (_, s) = extract_whitespace(&s);

        let (s, rhs) = Integer::parse(&s)?;

        Ok((s, Self { lhs, op, rhs }))
    }
}
