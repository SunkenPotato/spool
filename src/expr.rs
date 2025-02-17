use crate::{
    lit::{LitReal, Literal, Op},
    Parse,
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

#[cfg(test)]
mod tests {
    use crate::{
        expr::Expr,
        lit::{LitStr, Op},
        Parse,
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
}
