use std::num::{ParseFloatError, ParseIntError};

use env::Env;
use val::Val;

pub mod binding;
pub mod block;
pub mod env;
pub mod expr;
pub mod lit;
pub mod stmt;
pub mod utils;
pub mod val;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ParseError {
    ParseIntError(ParseIntError),
    ParseFloatError(ParseFloatError),
    SequenceNotFound { expected: String, received: String },
    InvalidSequence { expected: String, received: String },
}

pub type ParseOutput<S> = Result<(String, S), ParseError>;
pub trait Parse: Sized {
    fn parse(s: &str) -> ParseOutput<Self>;
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EvalError {
    IdentifierNotFound,
}

pub trait Eval {
    fn eval(&self, env: &mut Env) -> Result<Val, EvalError>;
}
