use std::error::Error;

use env::Env;
use val::Val;

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

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EvalError {
    NotFound(String),
}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Error for EvalError {}

pub trait Eval: Sized {
    fn eval(&self, env: &mut Env) -> Result<Val, EvalError>;
}
