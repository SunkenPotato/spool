use crate::{
    utils::{extract_float, extract_op, extract_string, extract_whitespace},
    Parse, ParseError,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    String(String),
    Float(f32),
}

impl Literal {
    fn parse_str(s: &str) -> crate::ParseOutput<Self> {
        let (_, s) = extract_whitespace(s);
        let (string, rest) = extract_string(&s)?;

        Ok((rest, Literal::String(string)))
    }

    fn parse_float(s: &str) -> crate::ParseOutput<Self> {
        let (_, s) = extract_whitespace(s);
        let (unparsed, rest) = extract_float(&s);

        let float = unparsed
            .parse()
            .map_err(|e| ParseError::ParseFloatError(e))?;

        Ok((rest, Literal::Float(float)))
    }
}

impl Parse for Literal {
    fn parse(s: &str) -> crate::ParseOutput<Self> {
        Self::parse_float(s).or_else(|_| Self::parse_str(s))
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
            Ok(("".into(), Literal::String("Hello, world".into())))
        )
    }

    #[test]
    fn parse_float() {
        assert_eq!(
            Literal::parse("3.1414723"),
            Ok(("".into(), Literal::Float(3.1414723)))
        )
    }

    #[test]
    fn parse_op() {
        assert_eq!(Op::parse("+").unwrap().1, Op::Add)
    }
}
