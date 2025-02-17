use crate::{lit::Literal, Parse};

pub enum Expr {
    Simple(Literal),
    MathExpr(),
}

impl Parse for Expr {
    fn parse(s: &str) -> crate::ParseOutput<Self> {
        todo!()
    }
}
