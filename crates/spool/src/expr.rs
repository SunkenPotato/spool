use crate::{
    binding::BindingRef,
    block::Block,
    lit::{LitReal, Literal, Op},
    val::Val,
    Eval, EvalError, Parse,
};

#[derive(Debug, PartialEq, Clone)]
pub struct MathExpr {
    pub lhs: Expr,
    pub op: Op,
    pub rhs: Expr,
}

impl Parse for MathExpr {
    fn parse(s: &str) -> crate::ParseOutput<Self> {
        let (s, lhs) = BindingRef::parse(s)
            .map(|(s, p)| (s, Expr::BindingRef(p)))
            .or_else(|_| LitReal::parse(s).map(|(s, p)| (s, Expr::Simple(Literal::Real(p)))))?;

        let (s, op) = Op::parse(&s)?;
        let (s, rhs) = Expr::parse(&s)?;

        Ok((s.into(), Self { lhs, op, rhs }))
    }
}

impl Eval for MathExpr {
    fn eval(&self, env: &mut crate::env::Env) -> Result<crate::val::Val, crate::EvalError> {
        let lhs = match self.lhs.eval(env)? {
            Val::Real(lhs) => lhs,
            v => {
                return Err(EvalError::InvalidType {
                    expected: "a real number".into(),
                    received: v.get_type().into(),
                })
            }
        };

        let rhs = match self.rhs.eval(env)? {
            Val::Real(lhs) => lhs,
            v => {
                return Err(EvalError::InvalidType {
                    expected: "a real number".into(),
                    received: v.get_type().into(),
                })
            }
        };

        Ok(crate::val::Val::Real(match self.op {
            Op::Add => lhs + rhs,
            Op::Sub => lhs - rhs,
            Op::Mul => lhs * rhs,
            Op::Div => lhs / rhs,
        }))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Simple(Literal),
    MathExpr(Box<MathExpr>),
    BindingRef(BindingRef),
    Block(Block),
}

impl Parse for Expr {
    fn parse(s: &str) -> crate::ParseOutput<Self> {
        MathExpr::parse(s)
            .map(|(s, p)| (s, Self::MathExpr(p.into())))
            .or_else(|_| Literal::parse(s).map(|(s, p)| (s, Self::Simple(p))))
            .or_else(|_| BindingRef::parse(s).map(|(s, p)| (s, Self::BindingRef(p))))
            .or_else(|_| Block::parse(s).map(|(s, p)| (s, Self::Block(p))))
    }
}

impl Eval for Expr {
    fn eval(&self, env: &mut crate::env::Env) -> Result<crate::val::Val, crate::EvalError> {
        match self {
            Self::Simple(lit) => lit.eval(env),
            Self::MathExpr(expr) => expr.eval(env),
            Self::BindingRef(b_ref) => b_ref.eval(env),
            Self::Block(block) => block.eval(env),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        binding::{Binding, BindingRef},
        env::Env,
        expr::{Expr, MathExpr},
        lit::{LitReal, LitStr, Op},
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
                Expr::MathExpr(
                    crate::expr::MathExpr {
                        lhs: Expr::Simple(crate::lit::Literal::Real(crate::lit::LitReal(5.))),
                        op: Op::Mul,
                        rhs: Expr::Simple(crate::lit::Literal::Real(crate::lit::LitReal(5.)))
                    }
                    .into()
                )
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
            crate::expr::MathExpr {
                lhs: Expr::Simple(crate::lit::Literal::Real(crate::lit::LitReal(5.))),
                op: Op::Mul,
                rhs: Expr::Simple(crate::lit::Literal::Real(crate::lit::LitReal(6.)))
            }
            .eval(&mut Env::new()),
            Ok(Val::Real(30.))
        )
    }

    #[test]
    fn eval_ref_expr() {
        let mut env = Env::new();

        let _ = Binding::new(
            "x".into(),
            Expr::MathExpr(
                crate::expr::MathExpr {
                    lhs: Expr::Simple(crate::lit::Literal::Real(crate::lit::LitReal(5.))),
                    op: Op::Mul,
                    rhs: Expr::Simple(crate::lit::Literal::Real(crate::lit::LitReal(5.))),
                }
                .into(),
            ),
        )
        .eval(&mut env);

        assert_eq!(
            Expr::BindingRef(BindingRef { id: "x".into() }).eval(&mut env),
            Ok(Val::Real(25.))
        )
    }

    #[test]
    fn eval_ref_math_expr() {
        let mut env = Env::new();
        let _ = Binding::new(
            "x".into(),
            Expr::MathExpr(
                crate::expr::MathExpr {
                    lhs: Expr::Simple(crate::lit::Literal::Real(crate::lit::LitReal(5.))),
                    op: Op::Mul,
                    rhs: Expr::Simple(crate::lit::Literal::Real(crate::lit::LitReal(5.))),
                }
                .into(),
            ),
        )
        .eval(&mut env);

        assert_eq!(
            Expr::MathExpr(
                MathExpr {
                    lhs: Expr::BindingRef(BindingRef { id: "x".into() }),
                    op: Op::Add,
                    rhs: Expr::Simple(crate::lit::Literal::Real(LitReal(4.)))
                }
                .into()
            )
            .eval(&mut env),
            Ok(Val::Real(29.))
        )
    }
}
