pub mod expr;
pub mod utils;
pub mod var;

#[cfg(test)]
mod tests {
    use crate::{
        utils::{extract_digits, extract_identifier, extract_operator, extract_whitespace},
        var::{Binding, Identifier},
    };

    use super::expr::*;

    #[test]
    fn parse_op() {
        assert_eq!(Op::parse("+").unwrap(), ("".into(), Op::Add));
        assert_eq!(Op::parse("-").unwrap(), ("".into(), Op::Sub));
        assert_eq!(Op::parse("*").unwrap(), ("".into(), Op::Mul));
        assert_eq!(Op::parse("/").unwrap(), ("".into(), Op::Div));
    }

    #[test]
    fn parse_integer() {
        assert_eq!(Integer::parse("123").unwrap(), ("".into(), Integer(123)));
        assert_eq!(Integer::parse("456").unwrap(), ("".into(), Integer(456)));
    }

    #[test]
    fn extract_number() {
        assert_eq!(extract_digits("123+456"), ("123".into(), "+456".into()));
    }

    #[test]
    fn extract_op() {
        assert_eq!(extract_operator("+456"), ("456".into(), "+".into()));
    }

    #[test]
    fn test_extract_whitespace() {
        assert_eq!(extract_whitespace("  123").1, "123");
    }

    #[test]
    fn parse_expr() {
        assert_eq!(
            Expr::parse("123 + 456").unwrap(),
            (
                "".into(),
                Expr {
                    lhs: Integer(123),
                    op: Op::Add,
                    rhs: Integer(456),
                }
            )
        )
    }

    #[test]
    fn extract_ident() {
        assert_eq!(extract_identifier("x 456"), ("x".into(), " 456".into()));
    }

    #[test]
    fn parse_binding() {
        assert_eq!(
            Binding::parse("bind x = 123 + 0").unwrap(),
            (
                "".into(),
                Binding {
                    name: Identifier::new("x".into()),
                    value: Expr {
                        lhs: Integer(123),
                        op: Op::Add,
                        rhs: Integer(0),
                    }
                }
            )
        )
    }
}
