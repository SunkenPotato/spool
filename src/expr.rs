use std::error::Error;

pub type ParseError = Box<dyn Error>;

pub trait Parse: Sized {
    fn parse(input: &str) -> Result<(&str, Self), ParseError>;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Integer(pub i32);

impl Parse for Integer {
    fn parse(input: &str) -> Result<(&str, Self), ParseError> {
        let (num, s) = extract_digits(input);
        Ok((s, Integer(num.parse()?)))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Parse for Op {
    fn parse(input: &str) -> Result<(&str, Self), ParseError> {
        let (s, op) = extract_operator(input);

        let op = match op {
            "+" => Op::Add,
            "-" => Op::Sub,
            "*" => Op::Mul,
            "/" => Op::Div,
            o => return Err(ParseError::from(format!("Invalid operator: {o}"))),
        };

        Ok((s, op))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Expr {
    pub lhs: Integer,
    pub op: Op,
    pub rhs: Integer,
}

impl Parse for Expr {
    fn parse(s: &str) -> Result<(&str, Self), ParseError> {
        let (s, lhs) = Integer::parse(s)?;
        let (s, op) = Op::parse(s)?;
        let (s, rhs) = Integer::parse(s)?;

        Ok((s, Self { lhs, op, rhs }))
    }
}

// seq: 123+456
// parse into: Integer(123), Op::Add, Integer(456)
pub fn extract_digits(s: &str) -> (&str, &str) {
    let end = s
        .char_indices()
        .find_map(|(i, c)| if c.is_ascii_digit() { None } else { Some(i) })
        .unwrap_or_else(|| s.len());

    (&s[..end], &s[end..])
}

pub fn extract_operator(s: &str) -> (&str, &str) {
    match &s[0..1] {
        "+" | "-" | "*" | "/" => {}
        _ => panic!(),
    };

    (&s[1..], &s[0..1])
}
