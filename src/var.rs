use std::{error::Error, fmt::Display};

use crate::{
    env::Env,
    expr::{Expr, Parse, ParseError},
    utils::{extract_end, extract_identifier, extract_whitespace},
};

const BIND_TOKEN: &str = "bind";
const ASSIGN_TOKEN: &str = "=";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    pub name: String,
}

impl Identifier {
    pub fn new(name: String) -> Self {
        Identifier { name }
    }
}

impl Parse for Identifier {
    fn parse(input: &str) -> Result<(String, Self), crate::expr::ParseError> {
        let (s, id) = extract_identifier(input);
        Ok((s, Identifier::new(id)))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Binding {
    pub name: Identifier,
    pub value: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BindingError {
    ExpectedBind,
    ExpectedIdentifier,
    ExpectedAssign,
    ExpectedExpr,
}

impl Binding {
    pub fn new(name: Identifier, value: Expr) -> Self {
        Binding { name, value }
    }

    pub fn eval(&self, env: &mut Env) {
        env.store_binding(self.name.name.clone(), self.value.eval());
    }
}

impl Error for BindingError {}
impl Display for BindingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Parse for Binding {
    fn parse(input: &str) -> Result<(String, Self), crate::expr::ParseError> {
        let input = if input.starts_with(BIND_TOKEN) {
            &input[4..]
        } else {
            return Err(BindingError::ExpectedBind.into());
        };

        let (_, input) = extract_whitespace(input);

        let (id, input) = extract_identifier(&input);
        let (_, input) = extract_whitespace(&input);

        let input = if input.starts_with(ASSIGN_TOKEN) {
            &input[1..]
        } else {
            return Err(BindingError::ExpectedAssign.into());
        };

        let (_, input) = extract_whitespace(input);
        let (input, expr) = Expr::parse(&input)?;

        let (end, input) = extract_end(&input);
        if end.chars().next() == None {
            return Err(ParseError::from("Unexpected end of input"));
        };

        Ok((
            input,
            Self {
                name: Identifier::new(id),
                value: expr,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::expr::{Integer, Op};

    use super::*;

    #[test]
    fn parse_binding() {
        assert_eq!(
            Binding::parse("bind x = 123 + 456;").unwrap(),
            (
                "".into(),
                Binding {
                    name: Identifier::new("x".into()),
                    value: Expr::Complex {
                        lhs: Integer(123),
                        op: Op::Add,
                        rhs: Integer(456),
                    }
                }
            )
        )
    }

    #[test]
    fn parse_simple_binding() {
        assert_eq!(
            Binding::parse("bind x = 123;").unwrap(),
            (
                "".into(),
                Binding {
                    name: Identifier::new("x".into()),
                    value: Expr::Simple(Integer(123)),
                }
            )
        )
    }
}
