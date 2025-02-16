use crate::{
    parse::Parse,
    stmt::Stmt,
    utils::{self, extract_whitespace},
};

const BLOCK_OPEN: &str = "{";
const BLOCK_CLOSE: &str = "}";

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

impl Parse for Block {
    fn parse(input: &str) -> crate::parse::ParseOutput<Self> {
        let input = utils::tag(BLOCK_OPEN, input)?;
        let (_, input) = extract_whitespace(&input);

        let mut stmts = vec![];
        let mut input = input;

        while let Ok((new_input, stmt)) = Stmt::parse(&input).inspect_err(|e| {
            dbg!(e);
        }) {
            stmts.push(stmt);
            input = extract_whitespace(&new_input).1;
        }

        let (_, input) = extract_whitespace(&input);
        dbg!(&input);
        let input = utils::tag(BLOCK_CLOSE, &input)?;

        Ok((input, Self { stmts }))
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
                    stmts: vec![Stmt::Binding(crate::var::Binding {
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
                    stmts: vec![
                        Stmt::Binding(crate::var::Binding {
                            name: "x".into(),
                            value: crate::expr::Expr::Simple(Integer(5))
                        }),
                        #[allow(unreachable_code)]
                        Stmt::Binding(crate::var::Binding {
                            name: "y".into(),
                            value: crate::expr::Expr::BindingRef(crate::var::BindingRef {
                                name: "x".into()
                            })
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
