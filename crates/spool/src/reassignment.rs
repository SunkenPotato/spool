use crate::{
    Eval, Parse,
    binding::{ASSIGN_TOKEN, Identifier},
    expr::Expr,
    utils::{extract_whitespace, tag},
};

#[derive(Debug, PartialEq, Clone)]
pub struct Reassignment {
    pub lhs: Identifier,
    pub rhs: Expr,
}

impl Parse for Reassignment {
    fn parse(s: &str) -> crate::ParseOutput<Self> {
        let (_, s) = extract_whitespace(s);

        let (s, lhs) = Identifier::parse(&s)?;

        let (_, s) = extract_whitespace(&s);
        let s = tag(ASSIGN_TOKEN, &s)?;

        let (s, rhs) = Expr::parse(&s)?;

        Ok((s, Self { lhs, rhs }))
    }
}

impl Eval for Reassignment {
    fn eval(&self, env: &mut crate::Env) -> Result<crate::val::Val, crate::EvalError> {
        env.get_stored_binding(&self.lhs)?;

        let rhs_val = self.rhs.eval(env)?;
        env.reassign_binding(self.lhs.clone(), rhs_val)?;

        Ok(crate::val::Val::Unit)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Env, Eval, Parse,
        expr::{Expr, MathExpr},
        lit::Op,
    };

    use super::Reassignment;

    #[test]
    fn parse_reassignment() {
        assert_eq!(
            Reassignment::parse("x = 5 + y"),
            Ok((
                "".into(),
                Reassignment {
                    lhs: "x".into(),
                    rhs: Expr::math_expr(
                        MathExpr {
                            lhs: Expr::simple(crate::lit::Literal::Real(crate::lit::LitReal(5.))),
                            op: Op::Add,
                            rhs: Expr::binding_ref(crate::binding::BindingRef { id: "y".into() })
                        }
                        .into()
                    )
                }
            ))
        )
    }

    #[test]
    fn eval_reassignment() {
        let mut env = Env::new();
        env.store_binding("x".into(), crate::val::Val::Bool(true), false);

        let _ = Reassignment {
            lhs: "x".into(),
            rhs: Expr::simple(crate::lit::Literal::Bool(crate::lit::LitBool(false))),
        }
        .eval(&mut env);

        assert_eq!(
            env.get_stored_binding(&"x".into()),
            Ok((crate::val::Val::Bool(true), false))
        )
    }
}
