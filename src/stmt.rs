use crate::{
    env::Env,
    expr::{EvalError, Expr},
    val::Val,
    var::Binding,
    Parse, ParseOutput,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
    Binding(Binding),
    Expr(Expr),
}

impl Stmt {
    pub fn eval(&self, env: &mut Env) -> Result<Val, EvalError> {
        match self {
            Stmt::Binding(binding) => binding.eval(env),
            Stmt::Expr(expr) => expr.eval(env),
        }
    }
}

impl Parse for Stmt {
    fn parse(input: &str) -> ParseOutput<Self> {
        Binding::parse(input)
            .map(|(s, binding_def)| (s, Self::Binding(binding_def)))
            .or_else(|_| Expr::parse(input).map(|(s, expr)| (s, Self::Expr(expr))))
    }
}

#[cfg(test)]
mod tests {
    use crate::{expr::Integer, stmt::Stmt, Parse};

    #[test]
    fn parse_binding_stmt() {
        assert_eq!(
            Stmt::parse("bind x = 5").unwrap(),
            (
                "".into(),
                Stmt::Binding(crate::var::Binding {
                    name: "x".into(),
                    value: crate::expr::Expr::Simple(Integer(5))
                })
            )
        )
    }

    #[test]
    fn parse_expr_stmt() {
        assert_eq!(
            Stmt::parse("5").unwrap(),
            ("".into(), Stmt::Expr(crate::expr::Expr::Simple(Integer(5))))
        )
    }
}
