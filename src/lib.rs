use std::num::{ParseFloatError, ParseIntError};

pub mod lit;
pub mod utils;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ParseError {
    ParseIntError(ParseIntError),
    ParseFloatError(ParseFloatError),
    SequenceNotFound { expected: String, received: String },
}

pub type ParseOutput<S> = Result<(String, S), ParseError>;
pub trait Parse: Sized {
    fn parse(s: &str) -> ParseOutput<Self>;
}
