use crate::{Eval, Parse, binding::Identifier, expr::Expr};

#[derive(Debug, PartialEq, Clone)]
pub struct Reassignment {
    lhs: Identifier,
    rhs: Expr,
}

impl Parse for Reassignment {
    fn parse(s: &str) -> crate::ParseOutput<Self> {
        todo!()
    }
}

impl Eval for Reassignment {
    fn eval(&self, env: &mut crate::Env) -> Result<crate::val::Val, crate::EvalError> {
        todo!()
    }
}
