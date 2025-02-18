use crate::{
    env::Env,
    stmt::Stmt,
    utils::{extract_whitespace, tag},
    Eval, Parse,
};

const BLOCK_OPEN: &str = "{";
const BLOCK_CLOSE: &str = "}";

#[derive(Debug, Clone, PartialEq)]
pub struct Block<'b> {
    pub stmts: Vec<Stmt<'b>>,
}

impl Parse for Block<'_> {
    fn parse(s: &str) -> crate::ParseOutput<Self> {
        let (_, s) = extract_whitespace(s);
        let s = tag(BLOCK_OPEN, &s)?;

        let mut stmts = vec![];
        let mut s = s;

        while let Ok((new_s, stmt)) = Stmt::parse(&s) {
            s = extract_whitespace(&new_s).1;
            stmts.push(stmt);
        }

        let s = tag(BLOCK_CLOSE, &s)?;

        Ok((s, Self { stmts }))
    }
}

impl Eval for Block<'_> {
    fn eval(&self, env: &mut crate::env::Env) -> Result<crate::val::Val, crate::EvalError> {
        if self.stmts.is_empty() {
            return Ok(crate::val::Val::Unit);
        }

        let mut this_env = Env::from_parent(env);

        let all_but_last = &self.stmts[..self.stmts.len() - 1];

        for stmt in all_but_last {
            if let Stmt::Binding(b) = stmt {
                b.eval(&mut this_env)?;
            }
        }

        let last = self.stmts.last().unwrap(); // this is ok because we checked if it's empty at the beginning

        Ok(last.eval(&mut this_env)?)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        binding::Binding, block::Block, env::Env, expr::Expr, lit::Op, stmt::Stmt, val::Val, Eval,
        Parse,
    };

    #[test]
    fn parse_empty_block() {
        assert_eq!(Block::parse("{}").unwrap().1, Block { stmts: vec![] })
    }

    #[test]
    fn parse_single_stmt_block() {
        assert_eq!(
            Block::parse("{ bind x = 5 }"),
            Ok((
                "".into(),
                Block {
                    stmts: vec![Stmt::Binding(Binding::new(
                        "x".into(),
                        crate::expr::Expr::Simple(crate::lit::Literal::Real(crate::lit::LitReal(
                            5.
                        )))
                    ))]
                }
            ))
        )
    }

    #[test]
    fn parse_multiple_stmt_block() {
        assert_eq!(
            Block::parse("{ bind x = 5 bind y = x y }"),
            Ok((
                "".into(),
                Block {
                    stmts: vec![
                        Stmt::Binding(Binding::new(
                            "x".into(),
                            crate::expr::Expr::Simple(crate::lit::Literal::Real(
                                crate::lit::LitReal(5.)
                            ))
                        )),
                        Stmt::Binding(Binding::new(
                            "y".into(),
                            crate::expr::Expr::BindingRef(crate::binding::BindingRef {
                                id: "x".into()
                            })
                        )),
                        Stmt::Expr(crate::expr::Expr::BindingRef(crate::binding::BindingRef {
                            id: "y".into()
                        }))
                    ]
                }
            ))
        )
    }

    #[test]
    fn eval_empty_block() {
        assert_eq!(
            Block { stmts: vec![] }.eval(&mut Env::new()),
            Ok(crate::val::Val::Unit)
        )
    }

    #[test]
    fn eval_single_stmt_block() {
        assert_eq!(
            Block {
                stmts: vec![Stmt::Expr(crate::expr::Expr::MathExpr(
                    crate::expr::MathExpr {
                        lhs: crate::lit::LitReal(5.),
                        op: Op::Sub,
                        rhs: crate::lit::LitReal(4.)
                    }
                ))]
            }
            .eval(&mut Env::new()),
            Ok(crate::val::Val::Real(1.))
        )
    }

    #[test]
    fn eval_multiple_stmt_block() {
        assert_eq!(
            Block {
                stmts: vec![
                    Stmt::Binding(Binding::new(
                        "e".into(),
                        Expr::Simple(crate::lit::Literal::Real(crate::lit::LitReal(2.71828)))
                    )),
                    Stmt::Expr(Expr::MathExpr(crate::expr::MathExpr {
                        lhs: crate::lit::LitReal(1.20205),
                        op: Op::Add,
                        rhs: crate::lit::LitReal(3.14159)
                    }))
                ]
            }
            .eval(&mut Env::new()),
            Ok(Val::Real(4.3436403))
        )
    }

    #[test]
    fn eval_stmt_external_env() {
        let mut external_env = Env::new();
        external_env.store_binding("outer".into(), Val::Real(3.14159));

        assert_eq!(
            Block {
                stmts: vec![Stmt::Expr(Expr::BindingRef(crate::binding::BindingRef {
                    id: "outer".into()
                }))]
            }
            .eval(&mut Env::from_parent(&external_env)),
            Ok(Val::Real(3.14159))
        )
    }
}
