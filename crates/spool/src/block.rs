use crate::{
    env::Env,
    stmt::Stmt,
    utils::{self, extract_whitespace},
    val::Val,
    Eval, EvalError, Parse, ParseOutput,
};

const BLOCK_OPEN: &str = "{";
const BLOCK_CLOSE: &str = "}";

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub(crate) struct Block {
    pub(crate) stmts: Vec<Stmt>,
}

impl Eval for Block {
    fn eval(&self, env: &mut Env) -> Result<Val, EvalError> {
        if self.stmts.is_empty() {
            return Ok(Val::Unit);
        }

        let mut inner_env = Env::new(Some(env));

        for stmt in &self.stmts[..self.stmts.len() - 1] {
            stmt.eval(&mut inner_env)?;
        }

        match self.stmts.last().unwrap() {
            Stmt::Binding(_) => Ok(Val::Unit),
            Stmt::Expr(expr) => expr.eval(&mut inner_env),
        }
    }
}

impl Parse for Block {
    fn parse(input: &str) -> ParseOutput<Self> {
        let input = utils::tag(BLOCK_OPEN, input)?;
        let (_, input) = extract_whitespace(&input);

        let mut stmts = vec![];
        let mut input = input;

        while let Ok((new_input, stmt)) = Stmt::parse(&input) {
            stmts.push(stmt);
            input = extract_whitespace(&new_input).1;
        }

        let (_, input) = extract_whitespace(&input);

        let input = utils::tag(BLOCK_CLOSE, &input)?;

        Ok((input, Self { stmts }))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        block::Block,
        env::Env,
        expr::{Expr, Integer, Op},
        stmt::Stmt,
        val::Val,
        Eval, Parse,
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

    #[test]
    fn eval_block() {
        assert_eq!(
            Block {
                stmts: vec![
                    Stmt::Binding(crate::var::Binding {
                        name: "x".into(),
                        value: crate::expr::Expr::Simple(Integer(5))
                    }),
                    Stmt::Expr(crate::expr::Expr::BindingRef(crate::var::BindingRef {
                        name: "x".into()
                    }))
                ]
            }
            .eval(&mut Env::default()),
            Ok(Val::Integer(5))
        )
    }

    #[test]
    fn eval_empty_block() {
        assert_eq!(
            Block { stmts: vec![] }.eval(&mut Env::default()),
            Ok(Val::Unit)
        )
    }

    #[test]
    fn eval_block_with_multiple_bindings() {
        assert_eq!(
            Block {
                stmts: vec![
                    Stmt::Binding(crate::var::Binding {
                        name: "x".into(),
                        value: crate::expr::Expr::Simple(Integer(5))
                    }),
                    Stmt::Binding(crate::var::Binding {
                        name: "y".into(),
                        value: crate::expr::Expr::Simple(Integer(6))
                    }),
                    Stmt::Binding(crate::var::Binding {
                        name: "z".into(),
                        value: crate::expr::Expr::Simple(Integer(7))
                    })
                ]
            }
            .eval(&mut Env::default()),
            Ok(Val::Unit)
        )
    }

    #[test]
    fn eval_with_inheriting() {
        let mut outer_env = Env::default();
        outer_env.store_binding("x".into(), Val::Integer(5));

        assert_eq!(
            Block {
                stmts: vec![
                    Stmt::Binding(crate::var::Binding {
                        name: "y".into(),
                        value: Expr::BindingRef(crate::var::BindingRef { name: "x".into() })
                    }),
                    Stmt::Expr(Expr::BindingRef(crate::var::BindingRef {
                        name: "y".into()
                    }))
                ]
            }
            .eval(&mut outer_env),
            Ok(Val::Integer(5))
        )
    }
}
