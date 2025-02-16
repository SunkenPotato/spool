use std::error::Error;

pub mod block;
pub mod expr;
pub mod stmt;
pub mod utils;
pub mod var;

mod env;
mod val;

pub type DynError<T> = Result<T, Box<dyn Error>>;

pub type ParseError = Box<dyn Error>;
pub type ParseOutput<S> = Result<(String, S), ParseError>;

pub trait Parse: Sized {
    fn parse(input: &str) -> ParseOutput<Self>;
}
