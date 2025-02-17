use crate::{binding::Binding, expr::Expr, Eval, Parse};

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt<'s> {
    Binding(Binding<'s>),
    Expr(Expr),
}

impl Parse for Stmt<'_> {
    fn parse(s: &str) -> crate::ParseOutput<Self> {
        Binding::parse(s)
            .map(|(s, p)| (s, Self::Binding(p)))
            .or_else(|_| Expr::parse(s).map(|(s, p)| (s, Self::Expr(p))))
    }
}

impl Eval for Stmt<'_> {
    fn eval(&self, env: &mut crate::env::Env) -> Result<crate::val::Val, crate::EvalError> {
        match self {
            Self::Binding(b) => b.eval(env),
            Self::Expr(e) => e.eval(env),
        }
    }
}
