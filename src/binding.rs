use std::marker::PhantomData;

use crate::{
    expr::Expr,
    utils::{extract_ident, extract_whitespace, tag},
    Parse,
};

const BIND_TOKEN: &str = "bind";
const ASSIGN_TOKEN: &str = "=";

#[derive(Debug, PartialEq, Clone)]
pub struct Identifier(pub String);

impl Parse for Identifier {
    fn parse(s: &str) -> crate::ParseOutput<Self> {
        let (_, s) = extract_whitespace(s);
        let (id, s) = extract_ident(&s)?;

        Ok((s, Self(id)))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Binding<'b> {
    pub ident: Identifier,
    pub expr: Expr,
    _p: PhantomData<&'b ()>,
}

impl Binding<'_> {
    #[cfg(test)]
    pub(crate) fn new(ident: Identifier, expr: Expr) -> Self {
        Self {
            ident,
            expr,
            _p: PhantomData,
        }
    }
}

impl<'b> Parse for Binding<'b> {
    fn parse(s: &str) -> crate::ParseOutput<Self> {
        let (_, s) = extract_whitespace(s);
        let s = tag(BIND_TOKEN, &s)?;

        let (s, ident) = Identifier::parse(&s)?;

        let (_, s) = extract_whitespace(&s);
        let s = tag(ASSIGN_TOKEN, &s)?;

        let (s, expr) = Expr::parse(&s)?;

        Ok((
            s,
            Binding {
                ident,
                expr,
                _p: PhantomData,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::{binding::Binding, Parse};

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
}
