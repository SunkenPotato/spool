use crate::{parse::Parse, stmt::Stmt};

#[derive(Debug, PartialEq, Default)]
pub struct Block {
    pub exprs: Vec<Stmt>,
}

impl Parse for Block {
    fn parse(input: &str) -> crate::parse::ParseOutput<Self> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        block::Block,
        expr::{Integer, Op},
        parse::Parse,
        stmt::Stmt,
    };

    #[test]
    fn parse_empty_block() {
        assert_eq!(Block::parse("{}").unwrap(), ("".into(), Block::default()))
    }

    #[test]
    fn parse_single_stmt_block() {
        assert_eq!(
            Block::parse("{ bind x = 5 }").unwrap(),
            (
                "".into(),
                Block {
                    exprs: vec![Stmt::Binding(crate::var::Binding {
                        name: "x".into(),
                        value: crate::expr::Expr::Simple(Integer(5))
                    })]
                }
            )
        )
    }

    #[test]
    fn parse_multiple_stmt_block() {
        assert_eq!(
            Block::parse("{ bind x = 5 bind y = x 5 + 5 }").unwrap(),
            (
                "".into(),
                Block {
                    exprs: vec![
                        Stmt::Binding(crate::var::Binding {
                            name: "x".into(),
                            value: crate::expr::Expr::Simple(Integer(5))
                        }),
                        #[allow(unreachable_code)]
                        Stmt::Binding(crate::var::Binding {
                            name: "y".into(),
                            value: todo!()
                        }),
                        Stmt::Expr(crate::expr::Expr::Complex {
                            lhs: Integer(5),
                            op: Op::Add,
                            rhs: Integer(5)
                        })
                    ]
                }
            )
        )
    }
}
