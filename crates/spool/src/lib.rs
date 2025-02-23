use binding::Identifier;
pub use env::Env;
use stmt::Stmt;

use std::num::{ParseFloatError, ParseIntError};
use val::Val;

pub(crate) mod binding;
pub(crate) mod block;
pub(crate) mod env;
pub(crate) mod expr;
mod fn_call;
pub mod func;
pub(crate) mod lit;
mod reassignment;
pub(crate) mod stmt;
pub(crate) mod utils;
pub(crate) mod val;

const KEYWORDS: &[&str] = &["func", "bind", "final"];
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

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
    IdentifierNotFound(Identifier),
    InvalidStoredType,
    InvalidArgumentLen,
    ImmutableReassignment(Identifier),
    InvalidType { expected: String, received: String },
}

pub trait Eval {
    fn eval(&self, env: &mut Env) -> Result<Val, EvalError>;
}

pub struct Parsed(Stmt);

impl Parsed {
    pub fn parse(s: &str) -> ParseOutput<Self> {
        let (s, stmt) = Stmt::parse(s)?;

        Ok((s, Self(stmt)))
    }

    pub fn eval(&self, env: &mut Env) -> Result<Val, EvalError> {
        self.0.eval(env)
    }
}
