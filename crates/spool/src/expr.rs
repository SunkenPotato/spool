use crate::{
    Eval, EvalError, Parse, ParseError,
    binding::BindingRef,
    block::Block,
    fn_call::FuncCall,
    lit::{LitReal, Literal, Op},
    utils::extract_whitespace,
    val::Val,
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
            .map(|(s, p)| (s, Expr::binding_ref(p)))
            .or_else(|_| LitReal::parse(s).map(|(s, p)| (s, Expr::simple(Literal::Real(p)))))?;

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
                });
            }
        };

        let rhs = match self.rhs.eval(env)? {
            Val::Real(lhs) => lhs,
            v => {
                return Err(EvalError::InvalidType {
                    expected: "a real number".into(),
                    received: v.get_type().into(),
                });
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
enum InnerExpr {
    Simple(Literal),
    MathExpr(Box<MathExpr>),
    BindingRef(BindingRef),
    FuncCall(FuncCall),
    Block(Block),
}

impl Parse for InnerExpr {
    fn parse(s: &str) -> crate::ParseOutput<Self> {
        FuncCall::parse(s)
            .map(|(s, p)| (s, Self::FuncCall(p)))
            .or_else(|_| MathExpr::parse(s).map(|(s, p)| (s, Self::MathExpr(p.into()))))
            .or_else(|_| Literal::parse(s).map(|(s, p)| (s, Self::Simple(p))))
            .or_else(|_| BindingRef::parse(s).map(|(s, p)| (s, Self::BindingRef(p))))
            .or_else(|_| Block::parse(s).map(|(s, p)| (s, Self::Block(p))))
    }
}

impl Eval for InnerExpr {
    fn eval(&self, env: &mut crate::env::Env) -> Result<crate::val::Val, crate::EvalError> {
        match self {
            Self::Simple(lit) => lit.eval(env),
            Self::MathExpr(expr) => expr.eval(env),
            Self::BindingRef(b_ref) => b_ref.eval(env),
            Self::Block(block) => block.eval(env),
            Self::FuncCall(fnc) => fnc.eval(env),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Expr {
    negate: Option<Negate>,
    inner: InnerExpr,
}

impl Expr {
    pub fn simple(lit: Literal) -> Self {
        Self {
            negate: None,
            inner: InnerExpr::Simple(lit),
        }
    }

    pub fn math_expr(me: Box<MathExpr>) -> Self {
        Self {
            negate: None,
            inner: InnerExpr::MathExpr(me),
        }
    }

    pub fn binding_ref(b_ref: BindingRef) -> Self {
        Self {
            negate: None,
            inner: InnerExpr::BindingRef(b_ref),
        }
    }

    pub fn func_call(call: FuncCall) -> Self {
        Self {
            negate: None,
            inner: InnerExpr::FuncCall(call),
        }
    }

    pub fn block(block: Block) -> Self {
        Self {
            negate: None,
            inner: InnerExpr::Block(block),
        }
    }

    pub fn negate(&mut self) {
        self.negate = Some(Negate);
    }
}

impl Parse for Expr {
    fn parse(s: &str) -> crate::ParseOutput<Self> {
        let negate_out = Negate::parse(s);
        let (s, negate) = (negate_out.0, negate_out.1.ok());
        let (s, inner) = InnerExpr::parse(&s)?;

        Ok((s, Self { inner, negate }))
    }
}

impl Eval for Expr {
    fn eval(&self, env: &mut crate::Env) -> Result<Val, EvalError> {
        let expr_val = self.inner.eval(env)?;
        Ok(match expr_val {
            Val::Bool(b) => Val::Bool(!b),
            v => v,
        })
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Negate;

impl Negate {
    fn parse(s: &str) -> (String, Result<Self, ParseError>) {
        let (_, s) = extract_whitespace(s);
        match s.chars().next() {
            Some(v) => match v {
                '!' => (s[1..].into(), Ok(Self)),
                e => (
                    s,
                    Err(crate::ParseError::InvalidSequence {
                        expected: '!'.into(),
                        received: e.into(),
                    }),
                ),
            },
            None => (
                s,
                Err(crate::ParseError::SequenceNotFound {
                    expected: '!'.into(),
                    received: "".into(),
                }),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Eval, Parse,
        binding::{Binding, BindingRef},
        env::Env,
        expr::{Expr, MathExpr},
        fn_call::FuncCall,
        lit::{LitReal, LitStr, Op},
        val::Val,
    };

    #[test]
    fn parse_simple_expr() {
        assert_eq!(
            Expr::parse("\"Hello, world\""),
            Ok((
                "".into(),
                Expr::simple(crate::lit::Literal::Str(LitStr("Hello, world".into())))
            ))
        )
    }

    #[test]
    fn parse_math_expr() {
        assert_eq!(
            Expr::parse("5 * 5"),
            Ok((
                "".into(),
                Expr::math_expr(
                    crate::expr::MathExpr {
                        lhs: Expr::simple(crate::lit::Literal::Real(crate::lit::LitReal(5.))),
                        op: Op::Mul,
                        rhs: Expr::simple(crate::lit::Literal::Real(crate::lit::LitReal(5.)))
                    }
                    .into()
                )
            ))
        )
    }

    #[test]
    fn eval_simple_expr() {
        assert_eq!(
            Expr::simple(crate::lit::Literal::Real(crate::lit::LitReal(5.))).eval(&mut Env::new()),
            Ok(Val::Real(5.))
        )
    }

    #[test]
    fn eval_math_expr() {
        assert_eq!(
            crate::expr::MathExpr {
                lhs: Expr::simple(crate::lit::Literal::Real(crate::lit::LitReal(5.))),
                op: Op::Mul,
                rhs: Expr::simple(crate::lit::Literal::Real(crate::lit::LitReal(6.)))
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
            Expr::math_expr(
                crate::expr::MathExpr {
                    lhs: Expr::simple(crate::lit::Literal::Real(crate::lit::LitReal(5.))),
                    op: Op::Mul,
                    rhs: Expr::simple(crate::lit::Literal::Real(crate::lit::LitReal(5.))),
                }
                .into(),
            ),
        )
        .eval(&mut env);

        assert_eq!(
            Expr::binding_ref(BindingRef { id: "x".into() }).eval(&mut env),
            Ok(Val::Real(25.))
        )
    }

    #[test]
    fn eval_ref_math_expr() {
        let mut env = Env::new();
        let _ = Binding::new(
            "x".into(),
            Expr::math_expr(
                crate::expr::MathExpr {
                    lhs: Expr::simple(crate::lit::Literal::Real(crate::lit::LitReal(5.))),
                    op: Op::Mul,
                    rhs: Expr::simple(crate::lit::Literal::Real(crate::lit::LitReal(5.))),
                }
                .into(),
            ),
        )
        .eval(&mut env);

        assert_eq!(
            Expr::math_expr(
                MathExpr {
                    lhs: Expr::binding_ref(BindingRef { id: "x".into() }),
                    op: Op::Add,
                    rhs: Expr::simple(crate::lit::Literal::Real(LitReal(4.)))
                }
                .into()
            )
            .eval(&mut env),
            Ok(Val::Real(29.))
        )
    }

    #[test]
    fn parse_fn_call() {
        assert_eq!(
            Expr::parse("test(hello, world)"),
            Ok((
                "".into(),
                Expr::func_call(FuncCall {
                    callee: "test".into(),
                    params: vec![
                        Expr::binding_ref(BindingRef { id: "hello".into() }),
                        Expr::binding_ref(BindingRef { id: "world".into() })
                    ]
                })
            ))
        )
    }

    #[test]
    fn eval_negate_expr() {
        assert_eq!(
            Expr {
                negate: Some(crate::expr::Negate),
                inner: crate::expr::InnerExpr::Simple(crate::lit::Literal::Bool(
                    crate::lit::LitBool(true)
                ))
            }
            .eval(&mut Env::new()),
            Ok(Val::Bool(false))
        )
    }

    #[test]
    fn parse_negated_bool() {
        assert_eq!(
            Expr::parse("!true"),
            Ok((
                "".into(),
                Expr {
                    negate: Some(crate::expr::Negate),
                    inner: crate::expr::InnerExpr::Simple(crate::lit::Literal::Bool(
                        crate::lit::LitBool(true)
                    ))
                }
            ))
        )
    }
}
