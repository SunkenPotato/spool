use crate::stmt::Stmt;

pub struct Block<'b> {
    pub stmts: Vec<Stmt<'b>>,
}
