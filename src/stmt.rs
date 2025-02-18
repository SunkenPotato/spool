use crate::{binding::Binding, expr::Expr, Eval, Parse};

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    Binding(Binding),
    Expr(Expr),
}

impl Parse for Stmt {
    fn parse(s: &str) -> crate::ParseOutput<Self> {
        Binding::parse(s)
            .map(|(s, p)| (s, Self::Binding(p)))
            .or_else(|_| Expr::parse(s).map(|(s, p)| (s, Self::Expr(p))))
    }
}

impl Eval for Stmt {
    fn eval(&self, env: &mut crate::env::Env) -> Result<crate::val::Val, crate::EvalError> {
        match self {
            Self::Binding(b) => b.eval(env),
            Self::Expr(e) => e.eval(env),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{binding::Binding, env::Env, stmt::Stmt, val::Val, Eval};

    #[test]
    fn eval_binding_stmt() {
        assert_eq!(
            Stmt::Binding(Binding::new(
                "x".into(),
                crate::expr::Expr::Simple(crate::lit::Literal::Real(crate::lit::LitReal(0.)))
            ))
            .eval(&mut Env::new()),
            Ok(Val::Unit)
        )
    }
}
