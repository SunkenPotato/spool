use crate::{expr::Expr, stmt::Stmt};

#[derive(Debug, PartialEq, Default)]
pub struct Block {
    pub exprs: Vec<Stmt>,
}
