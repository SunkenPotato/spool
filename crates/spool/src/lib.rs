pub use env::Env;
pub use val::Val;

use std::error::Error;

use stmt::Stmt;

pub(crate) mod block;
pub(crate) mod expr;
pub(crate) mod stmt;
pub(crate) mod utils;
pub(crate) mod var;

mod env;
mod val;

pub(crate) type DynError<T> = Result<T, Box<dyn Error>>;

pub(crate) type ParseError = Box<dyn Error>;
pub(crate) type ParseOutput<S> = Result<(String, S), ParseError>;

pub(crate) trait Parse: Sized {
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

pub(crate) trait Eval: Sized {
    fn eval(&self, env: &mut Env) -> Result<Val, EvalError>;
}

#[derive(Debug, Clone)]
pub struct Parsed(Stmt);

pub fn parse(input: &str) -> Result<Parsed, ParseError> {
    Ok(Parsed(Stmt::parse(input)?.1))
}

impl Parsed {
    pub fn eval(&self, env: &mut Env) -> Result<Val, EvalError> {
        self.0.eval(env)
    }
}
