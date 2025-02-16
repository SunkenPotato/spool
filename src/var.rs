use std::{error::Error, fmt::Display};

use crate::{
    env::Env,
    expr::{EvalError, Expr},
    parse::{Parse, ParseOutput},
    utils::{extract_identifier, extract_whitespace},
    val::Val,
};

const BIND_TOKEN: &str = "bind";
const ASSIGN_TOKEN: &str = "=";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    pub name: String,
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl<'a> From<&'a str> for Identifier {
    fn from(value: &'a str) -> Self {
        Self { name: value.into() }
    }
}

impl From<String> for Identifier {
    fn from(name: String) -> Self {
        Self { name }
    }
}

impl Identifier {
    pub fn new(name: String) -> Self {
        Identifier { name }
    }
}

impl Parse for Identifier {
    fn parse(input: &str) -> ParseOutput<Self> {
        let (id, s) = extract_identifier(input)?;
        Ok((s, Identifier::new(id)))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

    pub fn eval(&self, env: &mut Env) -> Result<Val, EvalError> {
        let val = self.value.eval(&env)?;
        env.store_binding(self.name.clone(), val);
        Ok(val)
    }
}

impl Error for BindingError {}
impl Display for BindingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Parse for Binding {
    fn parse(input: &str) -> ParseOutput<Self> {
        let input = if input.starts_with(BIND_TOKEN) {
            &input[4..]
        } else {
            return Err(BindingError::ExpectedBind.into());
        };

        let (_, input) = extract_whitespace(input);

        let (id, input) = extract_identifier(&input)?;
        let (_, input) = extract_whitespace(&input);

        let input = if input.starts_with(ASSIGN_TOKEN) {
            &input[1..]
        } else {
            return Err(BindingError::ExpectedAssign.into());
        };

        let (_, input) = extract_whitespace(input);
        let (input, expr) = Expr::parse(&input)?;

        Ok((
            input,
            Self {
                name: Identifier::new(id),
                value: expr,
            },
        ))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct BindingRef {
    pub name: Identifier,
}

impl Parse for BindingRef {
    fn parse(input: &str) -> ParseOutput<Self> {
        let (input, name) = Identifier::parse(input)?;
        Ok((input, Self { name }))
    }
}

impl BindingRef {
    #[inline]
    pub fn eval(&self, env: &Env) -> Result<Val, EvalError> {
        env.get_binding_val(&self.name)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        expr::{Integer, Op},
        val::Val,
    };

    use super::*;

    #[test]
    fn parse_binding() {
        assert_eq!(
            Binding::parse("bind x = 123 + 456").unwrap(),
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
            Binding::parse("bind x = 123").unwrap(),
            (
                "".into(),
                Binding {
                    name: Identifier::new("x".into()),
                    value: Expr::Simple(Integer(123)),
                }
            )
        )
    }

    #[test]
    fn eval_binding_ref() {
        let mut env = Env::default();
        env.store_binding("x".into(), Val::Integer(12));

        assert_eq!(
            BindingRef { name: "x".into() }.eval(&env),
            Ok(Val::Integer(12))
        )
    }

    #[test]
    fn eval_non_existent_ref() {
        let env = Env::default();

        assert_ne!(BindingRef { name: "x".into() }.eval(&env), Ok(Val::Unit))
    }

    #[test]
    fn parse_binding_ref() {
        assert_eq!(
            BindingRef { name: "xyz".into() },
            BindingRef::parse("xyz").unwrap().1
        )
    }
}
