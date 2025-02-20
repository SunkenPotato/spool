use crate::{
    binding::Identifier,
    expr::Expr,
    utils::{extract_whitespace, tag},
    Eval, Parse,
};

const FUNC_KW: &str = "func";

#[derive(Debug, Clone, PartialEq)]
pub struct FuncDef {
    pub(crate) id: Identifier,
    pub(crate) params: Vec<Identifier>,
    pub(crate) body: Expr,
}

impl Parse for FuncDef {
    fn parse(s: &str) -> crate::ParseOutput<Self> {
        let (_, s) = extract_whitespace(s);
        let s = tag(FUNC_KW, &s)?;

        let (_, s) = extract_whitespace(&s);
        let (s, id) = Identifier::parse(&s)?;

        let s = tag("(", &s)?;

        let mut params = vec![];
        let mut s = s;

        while let Ok((new_s, id)) = Identifier::parse(&s) {
            params.push(id);
            s = match tag(",", &new_s) {
                Ok(v) => v,
                Err(_) => new_s,
            };
        }

        let s = tag(")", &s)?;
        let (_, s) = extract_whitespace(&s);
        let s = tag("=>", &s)?;

        let (s, body) = Expr::parse(&s)?;

        Ok((s, Self { id, params, body }))
    }
}

impl Eval for FuncDef {
    #[allow(unused)]
    fn eval(&self, env: &mut crate::Env) -> Result<crate::val::Val, crate::EvalError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{func::FuncDef, Parse};

    #[test]
    fn parse() {
        assert_eq!(
            FuncDef::parse("func fn(x) => x"),
            Ok((
                "".into(),
                FuncDef {
                    id: "fn".into(),
                    params: vec!["x".into()],
                    body: crate::expr::Expr::BindingRef(crate::binding::BindingRef {
                        id: "x".into()
                    })
                }
            ))
        )
    }
}
