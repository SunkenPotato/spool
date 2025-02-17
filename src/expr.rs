use crate::{
    lit::{LitReal, Literal, Op},
    Eval, Parse,
};

#[derive(Debug, PartialEq, Clone)]
pub struct MathExpr {
    pub lhs: LitReal,
    pub op: Op,
    pub rhs: LitReal,
}

impl Parse for MathExpr {
    fn parse(s: &str) -> crate::ParseOutput<Self> {
        let (s, lhs) = LitReal::parse(&s)?;
        let (s, op) = Op::parse(&s)?;
        let (s, rhs) = LitReal::parse(&s)?;

        Ok((s.into(), Self { lhs, op, rhs }))
    }
}

impl Eval for MathExpr {
    fn eval(&self, _env: &mut crate::env::Env) -> Result<crate::val::Val, crate::EvalError> {
        Ok(crate::val::Val::Real(match self.op {
            Op::Add => self.rhs.0 + self.lhs.0,
            Op::Sub => self.rhs.0 - self.lhs.0,
            Op::Mul => self.rhs.0 * self.lhs.0,
            Op::Div => self.rhs.0 / self.lhs.0,
        }))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Simple(Literal),
    MathExpr(MathExpr),
}

impl Parse for Expr {
    fn parse(s: &str) -> crate::ParseOutput<Self> {
        MathExpr::parse(s)
            .map(|(s, p)| (s, Self::MathExpr(p)))
            .or_else(|_| Literal::parse(s).map(|(s, p)| (s, Self::Simple(p))))
    }
}

impl Eval for Expr {
    fn eval(&self, env: &mut crate::env::Env) -> Result<crate::val::Val, crate::EvalError> {
        match self {
            Self::Simple(lit) => lit.eval(env),
            Self::MathExpr(expr) => expr.eval(env),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        env::Env,
        expr::Expr,
        lit::{LitStr, Op},
        val::Val,
        Eval, Parse,
    };

    #[test]
    fn parse_simple_expr() {
        assert_eq!(
            Expr::parse("\"Hello, world\""),
            Ok((
                "".into(),
                Expr::Simple(crate::lit::Literal::Str(LitStr("Hello, world".into())))
            ))
        )
    }

    #[test]
    fn parse_math_expr() {
        assert_eq!(
            Expr::parse("5 * 5"),
            Ok((
                "".into(),
                Expr::MathExpr(crate::expr::MathExpr {
                    lhs: crate::lit::LitReal(5.),
                    op: Op::Mul,
                    rhs: crate::lit::LitReal(5.)
                })
            ))
        )
    }

    #[test]
    fn eval_simple_expr() {
        assert_eq!(
            Expr::Simple(crate::lit::Literal::Real(crate::lit::LitReal(5.))).eval(&mut Env::new()),
            Ok(Val::Real(5.))
        )
    }

    #[test]
    fn eval_math_expr() {
        assert_eq!(
            Expr::MathExpr(crate::expr::MathExpr {
                lhs: crate::lit::LitReal(5.),
                op: Op::Mul,
                rhs: crate::lit::LitReal(6.)
            })
            .eval(&mut Env::new()),
            Ok(Val::Real(30.))
        )
    }
}
