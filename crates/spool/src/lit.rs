use crate::{
    Eval, Parse, ParseError,
    utils::{extract_float, extract_op, extract_string, extract_whitespace},
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

#[cfg(test)]
impl<'a> From<&'a str> for LitStr {
    fn from(value: &'a str) -> Self {
        Self(value.into())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LitReal(pub f32);

#[cfg(test)]
impl From<f32> for LitReal {
    fn from(value: f32) -> Self {
        Self(value)
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
pub struct LitBool(pub bool);

#[cfg(test)]
impl From<bool> for LitBool {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl Parse for LitBool {
    fn parse(s: &str) -> crate::ParseOutput<Self> {
        let (_, s) = extract_whitespace(s);
        if s.len() < 4 {
            return Err(ParseError::SequenceNotFound {
                expected: "boolean sequence".into(),
                received: s.into(),
            });
        }

        let (unparsed, rest) = (&s[..4], &s[4..]);
        let inner = match unparsed {
            "true" => true,
            "false" => false,
            s => {
                return Err(ParseError::InvalidSequence {
                    expected: "true|false".into(),
                    received: s.into(),
                });
            }
        };

        Ok((rest.into(), Self(inner)))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Str(LitStr),
    Real(LitReal),
    Bool(LitBool),
}

impl Parse for Literal {
    fn parse(s: &str) -> crate::ParseOutput<Self> {
        LitReal::parse(s)
            .map(|(s, p)| (s, Self::Real(p)))
            .or_else(|_| LitStr::parse(s).map(|(s, p)| (s, Self::Str(p))))
            .or_else(|_| LitBool::parse(s).map(|(s, p)| (s, Self::Bool(p))))
    }
}

impl Eval for Literal {
    fn eval(&self, _env: &mut crate::env::Env) -> Result<crate::val::Val, crate::EvalError> {
        Ok(match self {
            Self::Str(s) => crate::val::Val::Str(s.0.clone()),
            Self::Real(r) => crate::val::Val::Real(r.0),
            Self::Bool(b) => crate::val::Val::Bool(b.0),
        })
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
        Eval, Parse,
        env::Env,
        lit::{Literal, Op},
        val::Val,
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

    #[test]
    fn eval_literal() {
        let mut env = Env::new();

        assert_eq!(
            Literal::Str("Hello, world!".into()).eval(&mut env),
            Ok(crate::val::Val::Str("Hello, world!".into()))
        );
        assert_eq!(
            Literal::Real(3.1414723.into()).eval(&mut env),
            Ok(Val::Real(3.1414723))
        );
        assert_eq!(
            Literal::Bool(true.into()).eval(&mut env),
            Ok(Val::Bool(true))
        )
    }

    #[test]
    fn parse_bool() {
        assert_eq!(
            Literal::parse("true"),
            Ok(("".into(), Literal::Bool(crate::lit::LitBool(true))))
        )
    }
}
