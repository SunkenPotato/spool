pub mod expr;

#[cfg(test)]
mod tests {
    use super::expr::*;

    #[test]
    fn parse_op() {
        assert_eq!(Op::parse("+").unwrap(), ("", Op::Add));
        assert_eq!(Op::parse("-").unwrap(), ("", Op::Sub));
        assert_eq!(Op::parse("*").unwrap(), ("", Op::Mul));
        assert_eq!(Op::parse("/").unwrap(), ("", Op::Div));
    }

    #[test]
    fn parse_integer() {
        assert_eq!(Integer::parse("123").unwrap(), ("", Integer(123)));
        assert_eq!(Integer::parse("456").unwrap(), ("", Integer(456)));
    }

    #[test]
    fn extract_number() {
        assert_eq!(extract_digits("123+456"), ("123", "+456"));
    }

    #[test]
    fn extract_op() {
        assert_eq!(extract_operator("+456"), ("456", "+"));
    }

    #[test]
    fn parse_expr() {
        assert_eq!(
            Expr::parse("123+456").unwrap(),
            (
                "",
                Expr {
                    lhs: Integer(123),
                    op: Op::Add,
                    rhs: Integer(456),
                }
            )
        )
    }
}
