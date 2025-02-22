use crate::{
    Eval, KEYWORDS, Parse, ParseError,
    expr::Expr,
    utils::{extract_ident, extract_whitespace, tag},
};

const BIND_TOKEN: &str = "bind";
pub const ASSIGN_TOKEN: &str = "=";
const IMMUTABLE_TOKEN: &str = "final";

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Identifier(pub String);

impl Parse for Identifier {
    fn parse(s: &str) -> crate::ParseOutput<Self> {
        let (_, s) = extract_whitespace(s);
        let (id, s) = extract_ident(&s)?;

        if id.is_empty() || KEYWORDS.contains(&id.as_str()) {
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

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Immutable;

impl Immutable {
    fn parse(s: &str) -> crate::ParseOutput<Option<Self>> {
        match tag(IMMUTABLE_TOKEN, s) {
            Ok(v) => Ok((v, Some(Self))),
            Err(_) => Ok((s.into(), None)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Binding {
    pub immutable: Option<Immutable>,
    pub ident: Identifier,
    pub expr: Expr,
}

#[cfg(test)]
impl Binding {
    pub fn new(immutable: Option<Immutable>, ident: Identifier, expr: Expr) -> Self {
        Self {
            immutable,
            ident,
            expr,
        }
    }
}

impl Parse for Binding {
    fn parse(s: &str) -> crate::ParseOutput<Self> {
        let (_, s) = extract_whitespace(s);
        let s = tag(BIND_TOKEN, &s)?;

        let (_, s) = extract_whitespace(&s);
        let (s, immutable) = dbg!(Immutable::parse(&s).unwrap());

        let (s, ident) = Identifier::parse(&s)?;

        let (_, s) = extract_whitespace(&s);
        let s = tag(ASSIGN_TOKEN, &s)?;

        let (s, expr) = Expr::parse(&s)?;

        Ok((
            s,
            Binding {
                immutable,
                ident,
                expr,
            },
        ))
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
        Eval, Parse,
        binding::{Binding, BindingRef},
        env::Env,
        expr::Expr,
        val::Val,
    };

    #[test]
    fn parse_binding() {
        assert_eq!(
            Binding::parse("bind x = \"Hello, world\""),
            Ok((
                "".into(),
                Binding::new(
                    None,
                    crate::binding::Identifier("x".into()),
                    crate::expr::Expr::simple(crate::lit::Literal::Str("Hello, world".into()))
                )
            ))
        )
    }

    #[test]
    fn eval_binding() {
        assert_eq!(
            Binding::new(
                None,
                "x".into(),
                crate::expr::Expr::simple(crate::lit::Literal::Real(crate::lit::LitReal(5.)))
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

    #[test]
    fn parse_immutable_binding() {
        assert_eq!(
            Binding::parse("bind final x = 5"),
            Ok((
                "".into(),
                Binding {
                    immutable: Some(crate::binding::Immutable),
                    ident: crate::binding::Identifier("x".into()),
                    expr: Expr::simple(crate::lit::Literal::Real(crate::lit::LitReal(5.)))
                }
            ))
        )
    }
}
