use crate::{
    expr::Expr,
    utils::{extract_ident, extract_whitespace, tag},
    Eval, Parse, ParseError,
};

const BIND_TOKEN: &str = "bind";
const ASSIGN_TOKEN: &str = "=";

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Identifier(pub String);

impl Parse for Identifier {
    fn parse(s: &str) -> crate::ParseOutput<Self> {
        let (_, s) = extract_whitespace(s);
        let (id, s) = extract_ident(&s)?;

        if id.is_empty() {
            return Err(ParseError::SequenceNotFound {
                expected: "valid identifier".into(),
                received: id.into(),
            });
        }

        Ok((s, Self(id)))
    }
}

#[cfg(test)]
impl From<&'_ str> for Identifier {
    fn from(value: &'_ str) -> Self {
        Self(value.into())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Binding {
    pub ident: Identifier,
    pub expr: Expr,
}

impl Binding {
    #[cfg(test)]
    pub fn new(ident: Identifier, expr: Expr) -> Self {
        Self { ident, expr }
    }
}

impl Parse for Binding {
    fn parse(s: &str) -> crate::ParseOutput<Self> {
        let (_, s) = extract_whitespace(s);
        let s = tag(BIND_TOKEN, &s)?;

        let (s, ident) = Identifier::parse(&s)?;

        let (_, s) = extract_whitespace(&s);
        let s = tag(ASSIGN_TOKEN, &s)?;

        let (s, expr) = Expr::parse(&s)?;

        Ok((s, Binding { ident, expr }))
    }
}

impl Eval for Binding {
    fn eval(&self, env: &mut crate::env::Env) -> Result<crate::val::Val, crate::EvalError> {
        let val = self.expr.eval(env)?;

        env.store_binding(self.ident.clone(), val);
        Ok(crate::val::Val::Unit)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BindingRef {
    pub id: Identifier,
}

impl Parse for BindingRef {
    fn parse(s: &str) -> crate::ParseOutput<Self> {
        let (s, id) = Identifier::parse(s)?;
        Ok((s, Self { id }))
    }
}

impl Eval for BindingRef {
    #[inline]
    fn eval(&self, env: &mut crate::env::Env) -> Result<crate::val::Val, crate::EvalError> {
        env.get_stored_binding(&self.id)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        binding::{Binding, BindingRef},
        env::Env,
        val::Val,
        Eval, Parse,
    };

    #[test]
    fn parse_binding() {
        assert_eq!(
            Binding::parse("bind x = \"Hello, world\""),
            Ok((
                "".into(),
                Binding::new(
                    crate::binding::Identifier("x".into()),
                    crate::expr::Expr::Simple(crate::lit::Literal::Str("Hello, world".into()))
                )
            ))
        )
    }

    #[test]
    fn eval_binding() {
        assert_eq!(
            Binding::new(
                "x".into(),
                crate::expr::Expr::Simple(crate::lit::Literal::Real(crate::lit::LitReal(5.)))
            )
            .eval(&mut Env::new())
            .unwrap(),
            Val::Unit
        )
    }

    #[test]
    fn eval_binding_ref() {
        let mut env = Env::new();

        env.store_binding("x".into(), Val::Real(5.));

        assert_eq!(
            BindingRef { id: "x".into() }.eval(&mut env),
            Ok(Val::Real(5.))
        )
    }
}
