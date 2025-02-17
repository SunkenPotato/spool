use crate::{
    utils::{extract_numbers, extract_whitespace},
    Parse, ParseError,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Integer(pub i32);

impl Parse for Integer {
    fn parse(s: &str) -> crate::ParseOutput<Self> {
        let (_, s) = extract_whitespace(s);
        let (unparsed, other) = extract_numbers(&s);

        let int = unparsed.parse().map_err(|e| ParseError::ParseIntError(e))?;

        Ok((other, Integer(int)))
    }
}

#[cfg(test)]
mod tests {
    use crate::{expr::Integer, Parse};

    #[test]
    fn parse_integer() {
        assert_eq!(Integer::parse("5"), Ok((String::new(), Integer(5))))
    }

    #[test]
    fn parse_integer_with_whitespace() {
        assert_eq!(Integer::parse("  5  "), Ok(("  ".into(), Integer(5))))
    }
}
