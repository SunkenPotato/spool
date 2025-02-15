use crate::expr::Expr;

#[derive(Debug, PartialEq, Default)]
pub struct Block {
    pub exprs: Vec<Expr>,
}
