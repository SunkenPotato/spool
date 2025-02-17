use std::num::ParseIntError;

pub mod expr;
pub mod utils;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ParseError {
    ParseIntError(ParseIntError),
}

pub type ParseOutput<S> = Result<(String, S), ParseError>;
pub trait Parse: Sized {
    fn parse(s: &str) -> ParseOutput<Self>;
}
