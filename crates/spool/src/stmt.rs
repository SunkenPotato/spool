use crate::{Eval, Parse, binding::Binding, expr::Expr, func::FuncDef};

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    Binding(Binding),
    Expr(Expr),
    Func(FuncDef),
}

impl Parse for Stmt {
    fn parse(s: &str) -> crate::ParseOutput<Self> {
        Binding::parse(s)
            .map(|(s, p)| (s, Self::Binding(p)))
            .or_else(|_| Expr::parse(s).map(|(s, p)| (s, Self::Expr(p))))
            .or_else(|_| FuncDef::parse(s).map(|(s, p)| (s, Self::Func(p))))
    }
}

impl Eval for Stmt {
    fn eval(&self, env: &mut crate::env::Env) -> Result<crate::val::Val, crate::EvalError> {
        match self {
            Self::Binding(b) => b.eval(env),
            Self::Expr(e) => e.eval(env),
            Self::Func(f) => f.eval(env),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Eval, Parse, binding::Binding, env::Env, func::FuncDef, stmt::Stmt, val::Val};

    #[test]
    fn eval_binding_stmt() {
        assert_eq!(
            Stmt::Binding(Binding::new(
                "x".into(),
                crate::expr::Expr::simple(crate::lit::Literal::Real(crate::lit::LitReal(0.)))
            ))
            .eval(&mut Env::new()),
            Ok(Val::Unit)
        )
    }

    #[test]
    fn parse_func() {
        assert_eq!(
            Stmt::parse("func fn(x) => x"),
            Ok((
                "".into(),
                Stmt::Func(FuncDef {
                    id: "fn".into(),
                    params: vec!["x".into()],
                    body: crate::expr::Expr::binding_ref(crate::binding::BindingRef {
                        id: "x".into()
                    })
                })
            ))
        )
    }
}
