use crate::{
    binding::BindingRef,
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
            Op::Add => self.lhs.0 + self.rhs.0,
            Op::Sub => self.lhs.0 - self.rhs.0,
            Op::Mul => self.lhs.0 * self.rhs.0,
            Op::Div => self.lhs.0 / self.rhs.0,
        }))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Simple(Literal),
    MathExpr(MathExpr),
    BindingRef(BindingRef),
}

impl Parse for Expr {
    fn parse(s: &str) -> crate::ParseOutput<Self> {
        MathExpr::parse(s)
            .map(|(s, p)| (s, Self::MathExpr(p)))
            .or_else(|_| Literal::parse(s).map(|(s, p)| (s, Self::Simple(p))))
            .or_else(|_| BindingRef::parse(s).map(|(s, p)| (s, Self::BindingRef(p))))
    }
}

impl Eval for Expr {
    fn eval(&self, env: &mut crate::env::Env) -> Result<crate::val::Val, crate::EvalError> {
        match self {
            Self::Simple(lit) => lit.eval(env),
            Self::MathExpr(expr) => expr.eval(env),
            Self::BindingRef(b_ref) => b_ref.eval(env),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        binding::{Binding, BindingRef},
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

    #[test]
    fn eval_ref_expr() {
        let mut env = Env::new();

        let _ = Binding::new(
            "x".into(),
            Expr::MathExpr(super::MathExpr {
                lhs: crate::lit::LitReal(5.),
                op: Op::Div,
                rhs: crate::lit::LitReal(5.),
            }),
        )
        .eval(&mut env);

        assert_eq!(
            Expr::BindingRef(BindingRef { id: "x".into() }).eval(&mut env),
            Ok(Val::Real(1.))
        )
    }
}
