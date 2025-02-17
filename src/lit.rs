use crate::{
    utils::{extract_float, extract_op, extract_string, extract_whitespace},
    Parse, ParseError,
};

#[derive(Debug, PartialEq, Clone)]
pub struct LitStr(pub String);

impl Parse for LitStr {
    fn parse(s: &str) -> crate::ParseOutput<Self> {
        let (_, s) = extract_whitespace(s);
        let (string, rest) = extract_string(&s)?;

        Ok((rest, LitStr(string)))
    }
}

impl<'a> From<&'a str> for LitStr {
    fn from(value: &'a str) -> Self {
        Self(value.into())
    }
}

impl Into<Literal> for LitStr {
    fn into(self) -> Literal {
        Literal::Str(self)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LitReal(pub f32);

impl From<f32> for LitReal {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

impl Into<Literal> for LitReal {
    fn into(self) -> Literal {
        Literal::Real(self)
    }
}

impl Parse for LitReal {
    fn parse(s: &str) -> crate::ParseOutput<Self> {
        let (_, s) = extract_whitespace(s);
        let (unparsed, rest) = extract_float(&s);

        let float = unparsed
            .parse()
            .map_err(|e| ParseError::ParseFloatError(e))?;

        Ok((rest, LitReal(float)))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Str(LitStr),
    Real(LitReal),
}

impl Parse for Literal {
    fn parse(s: &str) -> crate::ParseOutput<Self> {
        LitReal::parse(s)
            .map(|(s, p)| (s, Self::Real(p)))
            .or_else(|_| LitStr::parse(s).map(|(s, p)| (s, Self::Str(p))))
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
    fn parse(s: &str) -> crate::ParseOutput<Self> {
        let (_, s) = extract_whitespace(s);
        let (op, rest) = extract_op(&s)?;

        Ok((
            rest,
            match op.as_str() {
                "+" => Op::Add,
                "-" => Op::Sub,
                "*" => Op::Mul,
                "/" => Op::Div,
                o => panic!("parser should have returned at this point. received character {o}"),
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        lit::{Literal, Op},
        Parse,
    };

    #[test]
    fn parse_string() {
        assert_eq!(
            Literal::parse(r#""Hello, world""#),
            Ok(("".into(), Literal::Str("Hello, world".into())))
        )
    }

    #[test]
    fn parse_float() {
        assert_eq!(
            Literal::parse("3.1414723"),
            Ok(("".into(), Literal::Real(3.1414723.into())))
        )
    }

    #[test]
    fn parse_op() {
        assert_eq!(Op::parse("+").unwrap().1, Op::Add)
    }
}
