use crate::{
    Env, Eval, Parse,
    binding::Identifier,
    expr::Expr,
    utils::{extract_whitespace, tag},
};

#[derive(Debug, PartialEq, Clone)]
pub struct FuncCall {
    pub(crate) callee: Identifier,
    pub(crate) params: Vec<Expr>,
}

impl Parse for FuncCall {
    fn parse(s: &str) -> crate::ParseOutput<Self> {
        let (_, s) = extract_whitespace(s);
        let (s, id) = Identifier::parse(&s)?;

        let s = tag("(", &s)?;

        let mut params = vec![];
        let mut s = s;

        while let Ok((new_s, id)) = Expr::parse(&s) {
            params.push(id);
            s = match tag(",", &new_s) {
                Ok(v) => v,
                Err(_) => new_s,
            };
        }

        let s = tag(")", &s)?;

        Ok((s, Self { callee: id, params }))
    }
}

impl Eval for FuncCall {
    fn eval(&self, env: &mut crate::Env) -> Result<crate::val::Val, crate::EvalError> {
        let fn_id = &self.callee;
        let fn_def = env.get_stored_func(fn_id)?;
        let fn_params = fn_def.params;
        let call_params = &self.params;

        if fn_params.len() != call_params.len() {
            return Err(crate::EvalError::InvalidArgumentLen);
        }

        let mut fn_env = Env::new();

        for (idx, call_param) in call_params.iter().enumerate() {
            fn_env.store_binding(fn_params[idx].clone(), call_param.eval(env)?);
        }

        fn_def.body.eval(&mut fn_env)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Env, Eval, Parse,
        binding::BindingRef,
        block::Block,
        expr::{Expr, MathExpr},
        fn_call::FuncCall,
        lit::{LitReal, Literal},
        stmt::Stmt,
        val::Val,
    };

    #[test]
    fn parse_fn_call_no_params() {
        assert_eq!(
            FuncCall::parse("test()"),
            Ok((
                "".into(),
                FuncCall {
                    callee: "test".into(),
                    params: vec![]
                }
            ))
        )
    }

    #[test]
    fn parse_fn_call_one_param() {
        assert_eq!(
            FuncCall::parse("test(5)"),
            Ok((
                "".into(),
                FuncCall {
                    callee: "test".into(),
                    params: vec![Expr::simple(Literal::Real(crate::lit::LitReal(5.)))]
                }
            ))
        )
    }

    #[test]
    fn parse_fn_call_multiple_params() {
        assert_eq!(
            FuncCall::parse("test(hello, world)"),
            Ok((
                "".into(),
                FuncCall {
                    callee: "test".into(),
                    params: vec![
                        Expr::binding_ref(BindingRef { id: "hello".into() }),
                        Expr::binding_ref(BindingRef { id: "world".into() })
                    ]
                }
            ))
        )
    }

    #[test]
    fn eval_fn() {
        assert_eq!(
            Block {
                stmts: vec![
                    Stmt::Func(crate::func::FuncDef {
                        id: "testfn".into(),
                        params: vec!["a".into(), "b".into()],
                        body: crate::expr::Expr::math_expr(
                            MathExpr {
                                lhs: crate::expr::Expr::binding_ref(BindingRef { id: "a".into() }),
                                op: crate::lit::Op::Add,
                                rhs: crate::expr::Expr::binding_ref(BindingRef { id: "b".into() })
                            }
                            .into()
                        )
                    }),
                    Stmt::Expr(crate::expr::Expr::func_call(FuncCall {
                        callee: "testfn".into(),
                        params: vec![
                            Expr::simple(Literal::Real(LitReal(5.))),
                            Expr::simple(Literal::Real(LitReal(5.)))
                        ]
                    }))
                ]
            }
            .eval(&mut Env::new()),
            Ok(Val::Real(10.))
        )
    }
}
